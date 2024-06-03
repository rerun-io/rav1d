use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::LeftPixelRow2px;
use crate::include::common::intops::apply_sign;
use crate::include::common::intops::iclip;
use crate::src::cpu::CpuFlags;
use crate::src::tables::dav1d_cdef_directions;
use crate::src::wrap_fn_ptr::wrap_fn_ptr;
use bitflags::bitflags;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ptr;

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
use crate::include::common::bitdepth::bd_fn;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
use crate::include::common::bitdepth::{bpc_fn, BPC};

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct CdefEdgeFlags: u32 {
        const HAVE_LEFT = 1 << 0;
        const HAVE_RIGHT = 1 << 1;
        const HAVE_TOP = 1 << 2;
        const HAVE_BOTTOM = 1 << 3;
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn cdef(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const [LeftPixelRow2px<DynPixel>; 8],
    top: *const DynPixel,
    bottom: *const DynPixel,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    edges: CdefEdgeFlags,
    bitdepth_max: c_int,
) -> ());

impl cdef::Fn {
    /// CDEF operates entirely on pre-filter data.
    /// If bottom/right edges are present (according to `edges`),
    /// then the pre-filter data is located in `dst`.
    /// However, the edge pixels above `dst` may be post-filter,
    /// so in order to get access to pre-filter top pixels, use `top`.
    pub unsafe fn call<BD: BitDepth>(
        &self,
        dst: *mut BD::Pixel,
        stride: ptrdiff_t,
        left: &[LeftPixelRow2px<BD::Pixel>; 8],
        top: *const BD::Pixel,
        bottom: *const BD::Pixel,
        pri_strength: c_int,
        sec_strength: u8,
        dir: c_int,
        damping: u8,
        edges: CdefEdgeFlags,
        bd: BD,
    ) {
        let dst = dst.cast();
        let left = ptr::from_ref(left).cast();
        let top = top.cast();
        let bottom = bottom.cast();
        let sec_strength = sec_strength as c_int;
        let damping = damping as c_int;
        let bd = bd.into_c();
        self.get()(
            dst,
            stride,
            left,
            top,
            bottom,
            pri_strength,
            sec_strength,
            dir,
            damping,
            edges,
            bd,
        )
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn cdef_dir(
    dst: *const DynPixel,
    dst_stride: ptrdiff_t,
    variance: &mut c_uint,
    bitdepth_max: c_int,
) -> c_int);

impl cdef_dir::Fn {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        dst: *const BD::Pixel,
        dst_stride: ptrdiff_t,
        variance: &mut c_uint,
        bd: BD,
    ) -> c_int {
        let dst = dst.cast();
        let bd = bd.into_c();
        self.get()(dst, dst_stride, variance, bd)
    }
}

pub struct Rav1dCdefDSPContext {
    pub dir: cdef_dir::Fn,

    /// 444/luma, 422, 420
    pub fb: [cdef::Fn; 3],
}

#[inline]
pub fn constrain(diff: c_int, threshold: c_int, shift: c_int) -> c_int {
    let adiff = diff.abs();
    apply_sign(
        cmp::min(adiff, cmp::max(0, threshold - (adiff >> shift))),
        diff,
    )
}

#[inline]
pub unsafe fn fill(mut tmp: *mut i16, stride: ptrdiff_t, w: c_int, h: c_int) {
    for _ in 0..h {
        for x in 0..w {
            *tmp.offset(x as isize) = i16::MIN;
        }
        tmp = tmp.offset(stride);
    }
}

unsafe fn padding<BD: BitDepth>(
    mut tmp: *mut i16,
    tmp_stride: ptrdiff_t,
    mut src: *const BD::Pixel,
    src_stride: ptrdiff_t,
    left: &[LeftPixelRow2px<BD::Pixel>; 8],
    mut top: *const BD::Pixel,
    mut bottom: *const BD::Pixel,
    w: c_int,
    h: c_int,
    edges: CdefEdgeFlags,
) {
    let mut x_start = -2;
    let mut x_end = w + 2;
    let mut y_start = -2;
    let mut y_end = h + 2;
    if !edges.contains(CdefEdgeFlags::HAVE_TOP) {
        fill(
            tmp.offset(-2).offset(-(2 * tmp_stride)),
            tmp_stride,
            w + 4,
            2,
        );
        y_start = 0;
    }
    if !edges.contains(CdefEdgeFlags::HAVE_BOTTOM) {
        fill(
            tmp.offset(h as isize * tmp_stride).offset(-2),
            tmp_stride,
            w + 4,
            2,
        );
        y_end -= 2;
    }
    if !edges.contains(CdefEdgeFlags::HAVE_LEFT) {
        fill(
            tmp.offset(y_start as isize * tmp_stride).offset(-2),
            tmp_stride,
            2,
            y_end - y_start,
        );
        x_start = 0;
    }
    if !edges.contains(CdefEdgeFlags::HAVE_RIGHT) {
        fill(
            tmp.offset(y_start as isize * tmp_stride).offset(w as isize),
            tmp_stride,
            2,
            y_end - y_start,
        );
        x_end -= 2;
    }
    for y in y_start..0 {
        for x in x_start..x_end {
            *tmp.offset(x as isize + y as isize * tmp_stride) =
                (*top.offset(x as isize)).as_::<i16>();
        }
        top = top.offset(BD::pxstride(src_stride));
    }
    for y in 0..h as usize {
        for x in x_start..0 {
            *tmp.offset(x as isize + y as isize * tmp_stride) =
                left[y][(2 + x) as usize].as_::<i16>();
        }
    }
    for y in 0..h {
        for x in if y < h { 0 } else { x_start }..x_end {
            *tmp.offset(x as isize) = (*src.offset(x as isize)).as_::<i16>();
        }
        src = src.offset(BD::pxstride(src_stride));
        tmp = tmp.offset(tmp_stride);
    }
    for _ in h..y_end {
        for x in x_start..x_end {
            *tmp.offset(x as isize) = (*bottom.offset(x as isize)).as_::<i16>();
        }
        bottom = bottom.offset(BD::pxstride(src_stride));
        tmp = tmp.offset(tmp_stride);
    }
}

#[inline(never)]
unsafe fn cdef_filter_block_c<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: ptrdiff_t,
    left: &[LeftPixelRow2px<BD::Pixel>; 8],
    top: *const BD::Pixel,
    bottom: *const BD::Pixel,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    w: c_int,
    h: c_int,
    edges: CdefEdgeFlags,
    bd: BD,
) {
    let dir = dir as usize;

    let tmp_stride = 12;
    assert!((w == 4 || w == 8) && (h == 4 || h == 8));
    let mut tmp_buf = [0; 144];
    let mut tmp = tmp_buf.as_mut_ptr().offset(2 * tmp_stride).offset(2);
    padding::<BD>(
        tmp, tmp_stride, dst, dst_stride, left, top, bottom, w, h, edges,
    );
    if pri_strength != 0 {
        let bitdepth_min_8 = bd.bitdepth().as_::<c_int>() - 8;
        let pri_tap = 4 - (pri_strength >> bitdepth_min_8 & 1);
        let pri_shift = cmp::max(0, damping - pri_strength.ilog2() as c_int);
        if sec_strength != 0 {
            let sec_shift = damping - sec_strength.ilog2() as c_int;
            for _ in 0..h {
                for x in 0..w {
                    let px = (*dst.offset(x as isize)).as_::<c_int>();
                    let mut sum = 0;
                    let mut max = px;
                    let mut min = px;
                    let mut pri_tap_k = pri_tap;
                    for k in 0..2 {
                        let off1 = dav1d_cdef_directions[dir + 2][k] as c_int;
                        let p0 = *tmp.offset((x + off1) as isize) as c_int;
                        let p1 = *tmp.offset((x - off1) as isize) as c_int;
                        sum += pri_tap_k * constrain(p0 - px, pri_strength, pri_shift);
                        sum += pri_tap_k * constrain(p1 - px, pri_strength, pri_shift);
                        pri_tap_k = pri_tap_k & 3 | 2;
                        min = cmp::min(p0 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(p0, max);
                        min = cmp::min(p1 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(p1, max);
                        let off2 = dav1d_cdef_directions[dir + 4][k] as c_int;
                        let off3 = dav1d_cdef_directions[dir + 0][k] as c_int;
                        let s0 = *tmp.offset((x + off2) as isize) as c_int;
                        let s1 = *tmp.offset((x - off2) as isize) as c_int;
                        let s2 = *tmp.offset((x + off3) as isize) as c_int;
                        let s3 = *tmp.offset((x - off3) as isize) as c_int;
                        let sec_tap = 2 - k as c_int;
                        sum += sec_tap * constrain(s0 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s1 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s2 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s3 - px, sec_strength, sec_shift);
                        min = cmp::min(s0 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(s0, max);
                        min = cmp::min(s1 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(s1, max);
                        min = cmp::min(s2 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(s2, max);
                        min = cmp::min(s3 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(s3, max);
                    }
                    *dst.offset(x as isize) =
                        iclip(px + (sum - (sum < 0) as c_int + 8 >> 4), min, max)
                            .as_::<BD::Pixel>();
                }
                dst = dst.offset(BD::pxstride(dst_stride));
                tmp = tmp.offset(tmp_stride);
            }
        } else {
            for _ in 0..h {
                for x in 0..w {
                    let px = (*dst.offset(x as isize)).as_::<c_int>();
                    let mut sum = 0;
                    let mut pri_tap_k = pri_tap;
                    for k in 0..2 {
                        let off = dav1d_cdef_directions[dir + 2][k] as c_int;
                        let p0 = *tmp.offset((x + off) as isize) as c_int;
                        let p1 = *tmp.offset((x - off) as isize) as c_int;
                        sum += pri_tap_k * constrain(p0 - px, pri_strength, pri_shift);
                        sum += pri_tap_k * constrain(p1 - px, pri_strength, pri_shift);
                        pri_tap_k = pri_tap_k & 3 | 2;
                    }
                    *dst.offset(x as isize) =
                        (px + (sum - (sum < 0) as c_int + 8 >> 4)).as_::<BD::Pixel>();
                }
                dst = dst.offset(BD::pxstride(dst_stride));
                tmp = tmp.offset(tmp_stride);
            }
        }
    } else {
        let sec_shift = damping - sec_strength.ilog2() as c_int;
        for _ in 0..h {
            for x in 0..w {
                let px = (*dst.offset(x as isize)).as_::<c_int>();
                let mut sum = 0;
                for k in 0..2 {
                    let off1 = dav1d_cdef_directions[dir + 4][k] as c_int;
                    let off2 = dav1d_cdef_directions[dir + 0][k] as c_int;
                    let s0 = *tmp.offset((x + off1) as isize) as c_int;
                    let s1 = *tmp.offset((x - off1) as isize) as c_int;
                    let s2 = *tmp.offset((x + off2) as isize) as c_int;
                    let s3 = *tmp.offset((x - off2) as isize) as c_int;
                    let sec_tap = 2 - k as c_int;
                    sum += sec_tap * constrain(s0 - px, sec_strength, sec_shift);
                    sum += sec_tap * constrain(s1 - px, sec_strength, sec_shift);
                    sum += sec_tap * constrain(s2 - px, sec_strength, sec_shift);
                    sum += sec_tap * constrain(s3 - px, sec_strength, sec_shift);
                }
                *dst.offset(x as isize) =
                    (px + (sum - (sum < 0) as c_int + 8 >> 4)).as_::<BD::Pixel>();
            }
            dst = dst.offset(BD::pxstride(dst_stride));
            tmp = tmp.offset(tmp_stride);
        }
    };
}

unsafe extern "C" fn cdef_filter_block_c_erased<BD: BitDepth, const W: usize, const H: usize>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const [LeftPixelRow2px<DynPixel>; 8],
    top: *const DynPixel,
    bottom: *const DynPixel,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    edges: CdefEdgeFlags,
    bitdepth_max: c_int,
) {
    let dst = dst.cast();
    // SAFETY: Reverse of cast in `cdef::Fn::call`.
    let left = unsafe { &*left.cast() };
    let top = top.cast();
    let bottom = bottom.cast();
    let bd = BD::from_c(bitdepth_max);
    cdef_filter_block_c(
        dst,
        stride,
        left,
        top,
        bottom,
        pri_strength,
        sec_strength,
        dir,
        damping,
        W as c_int,
        H as c_int,
        edges,
        bd,
    )
}

unsafe extern "C" fn cdef_find_dir_c_erased<BD: BitDepth>(
    img: *const DynPixel,
    stride: ptrdiff_t,
    variance: &mut c_uint,
    bitdepth_max: c_int,
) -> c_int {
    cdef_find_dir_rust(img.cast(), stride, variance, BD::from_c(bitdepth_max))
}

unsafe fn cdef_find_dir_rust<BD: BitDepth>(
    mut img: *const BD::Pixel,
    stride: ptrdiff_t,
    variance: &mut c_uint,
    bd: BD,
) -> c_int {
    let bitdepth_min_8 = bd.bitdepth().as_::<c_int>() - 8;
    let mut partial_sum_hv = [[0; 8]; 2];
    let mut partial_sum_diag = [[0; 15]; 2];
    let mut partial_sum_alt = [[0; 11]; 4];
    for y in 0..8 {
        for x in 0..8 {
            let px = ((*img.offset(x as isize)).as_::<c_int>() >> bitdepth_min_8) - 128;
            partial_sum_diag[0][y + x] += px;
            partial_sum_alt[0][y + (x >> 1)] += px;
            partial_sum_hv[0][y] += px;
            partial_sum_alt[1][3 + y - (x >> 1)] += px;
            partial_sum_diag[1][7 + y - x] += px;
            partial_sum_alt[2][3 - (y >> 1) + x] += px;
            partial_sum_hv[1][x] += px;
            partial_sum_alt[3][(y >> 1) + x] += px;
        }
        img = img.offset(BD::pxstride(stride));
    }
    let mut cost = [0; 8];
    for n in 0..8 {
        cost[2] += (partial_sum_hv[0][n] * partial_sum_hv[0][n]) as c_uint;
        cost[6] += (partial_sum_hv[1][n] * partial_sum_hv[1][n]) as c_uint;
    }
    cost[2] *= 105;
    cost[6] *= 105;
    static div_table: [u16; 7] = [840, 420, 280, 210, 168, 140, 120];
    for n in 0..7 {
        let d = div_table[n] as c_int;
        cost[0] += ((partial_sum_diag[0][n] * partial_sum_diag[0][n]
            + partial_sum_diag[0][14 - n] * partial_sum_diag[0][14 - n])
            * d) as c_uint;
        cost[4] += ((partial_sum_diag[1][n] * partial_sum_diag[1][n]
            + partial_sum_diag[1][14 - n] * partial_sum_diag[1][14 - n])
            * d) as c_uint;
    }
    cost[0] += (partial_sum_diag[0][7] * partial_sum_diag[0][7] * 105) as c_uint;
    cost[4] += (partial_sum_diag[1][7] * partial_sum_diag[1][7] * 105) as c_uint;
    for n in 0..4 {
        let cost_ptr = &mut *cost.as_mut_ptr().offset((n * 2 + 1) as isize) as *mut c_uint;
        for m in 0..5 {
            *cost_ptr += (partial_sum_alt[n][3 + m] * partial_sum_alt[n][3 + m]) as c_uint;
        }
        *cost_ptr *= 105;
        for m in 0..3 {
            let d = div_table[2 * m + 1] as c_int;
            *cost_ptr += ((partial_sum_alt[n][m] * partial_sum_alt[n][m]
                + partial_sum_alt[n][10 - m] * partial_sum_alt[n][10 - m])
                * d) as c_uint;
        }
    }
    let mut best_dir = 0;
    let mut best_cost = cost[0];
    for n in 0..8 {
        if cost[n] > best_cost {
            best_cost = cost[n];
            best_dir = n;
        }
    }
    *variance = (best_cost - cost[best_dir ^ 4]) >> 10;
    best_dir as c_int
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
wrap_fn_ptr!(unsafe extern "C" fn padding(
    tmp: *mut u16,
    src: *const DynPixel,
    src_stride: ptrdiff_t,
    left: *const [LeftPixelRow2px<DynPixel>; 8],
    top: *const DynPixel,
    bottom: *const DynPixel,
    h: c_int,
    edges: CdefEdgeFlags,
) -> ());

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
wrap_fn_ptr!(unsafe extern "C" fn filter(
    dst: *mut DynPixel,
    dst_stride: ptrdiff_t,
    tmp: *const u16,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    h: c_int,
    edges: usize,
    bitdepth_max: c_int,
) -> ());

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
unsafe extern "C" fn cdef_filter_neon_erased<
    BD: BitDepth,
    const W: usize,
    const H: usize,
    const TMP_STRIDE: usize,
    const TMP_LEN: usize,
>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    left: *const [LeftPixelRow2px<DynPixel>; 8],
    top: *const DynPixel,
    bottom: *const DynPixel,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    edges: CdefEdgeFlags,
    bitdepth_max: c_int,
) {
    use crate::src::align::Align16;

    let mut tmp_buf = Align16([0; TMP_LEN]);
    let tmp = tmp_buf.0.as_mut_ptr().add(2 * TMP_STRIDE + 8);
    let (padding, filter) = match W {
        4 => (
            bd_fn!(padding::decl_fn, BD, cdef_padding4, neon),
            bd_fn!(filter::decl_fn, BD, cdef_filter4, neon),
        ),
        8 => (
            bd_fn!(padding::decl_fn, BD, cdef_padding8, neon),
            bd_fn!(filter::decl_fn, BD, cdef_filter8, neon),
        ),
        _ => unreachable!(),
    };
    padding.get()(tmp, dst, stride, left, top, bottom, H as c_int, edges);
    filter.get()(
        dst,
        stride,
        tmp,
        pri_strength,
        sec_strength,
        dir,
        damping,
        H as c_int,
        edges.bits() as usize,
        bitdepth_max,
    );
}

impl Rav1dCdefDSPContext {
    pub const fn default<BD: BitDepth>() -> Self {
        Self {
            dir: cdef_dir::Fn::new(cdef_find_dir_c_erased::<BD>),
            fb: [
                cdef::Fn::new(cdef_filter_block_c_erased::<BD, 8, 8>),
                cdef::Fn::new(cdef_filter_block_c_erased::<BD, 4, 8>),
                cdef::Fn::new(cdef_filter_block_c_erased::<BD, 4, 4>),
            ],
        }
    }

    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline(always)]
    const fn init_x86<BD: BitDepth>(mut self, flags: CpuFlags) -> Self {
        if matches!(BD::BPC, BPC::BPC8) {
            if !flags.contains(CpuFlags::SSE2) {
                return self;
            }

            self.fb[0] = bpc_fn!(cdef::decl_fn, 8 bpc, cdef_filter_8x8, sse2);
            self.fb[1] = bpc_fn!(cdef::decl_fn, 8 bpc, cdef_filter_4x8, sse2);
            self.fb[2] = bpc_fn!(cdef::decl_fn, 8 bpc, cdef_filter_4x4, sse2);
        }

        if !flags.contains(CpuFlags::SSSE3) {
            return self;
        }

        self.dir = bd_fn!(cdef_dir::decl_fn, BD, cdef_dir, ssse3);
        self.fb[0] = bd_fn!(cdef::decl_fn, BD, cdef_filter_8x8, ssse3);
        self.fb[1] = bd_fn!(cdef::decl_fn, BD, cdef_filter_4x8, ssse3);
        self.fb[2] = bd_fn!(cdef::decl_fn, BD, cdef_filter_4x4, ssse3);

        if !flags.contains(CpuFlags::SSE41) {
            return self;
        }

        self.dir = bd_fn!(cdef_dir::decl_fn, BD, cdef_dir, sse4);
        if matches!(BD::BPC, BPC::BPC8) {
            self.fb[0] = bpc_fn!(cdef::decl_fn, 8 bpc, cdef_filter_8x8, sse4);
            self.fb[1] = bpc_fn!(cdef::decl_fn, 8 bpc, cdef_filter_4x8, sse4);
            self.fb[2] = bpc_fn!(cdef::decl_fn, 8 bpc, cdef_filter_4x4, sse4);
        }

        #[cfg(target_arch = "x86_64")]
        {
            if !flags.contains(CpuFlags::AVX2) {
                return self;
            }

            self.dir = bd_fn!(cdef_dir::decl_fn, BD, cdef_dir, avx2);
            self.fb[0] = bd_fn!(cdef::decl_fn, BD, cdef_filter_8x8, avx2);
            self.fb[1] = bd_fn!(cdef::decl_fn, BD, cdef_filter_4x8, avx2);
            self.fb[2] = bd_fn!(cdef::decl_fn, BD, cdef_filter_4x4, avx2);

            if !flags.contains(CpuFlags::AVX512ICL) {
                return self;
            }

            self.fb[0] = bd_fn!(cdef::decl_fn, BD, cdef_filter_8x8, avx512icl);
            self.fb[1] = bd_fn!(cdef::decl_fn, BD, cdef_filter_4x8, avx512icl);
            self.fb[2] = bd_fn!(cdef::decl_fn, BD, cdef_filter_4x4, avx512icl);
        }

        self
    }

    #[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
    #[inline(always)]
    const fn init_arm<BD: BitDepth>(mut self, flags: CpuFlags) -> Self {
        if !flags.contains(CpuFlags::NEON) {
            return self;
        }

        self.dir = bd_fn!(cdef_dir::decl_fn, BD, cdef_find_dir, neon);
        self.fb[0] = cdef::Fn::new(cdef_filter_neon_erased::<BD, 8, 8, 16, { 12 * 16 + 8 }>);
        self.fb[1] = cdef::Fn::new(cdef_filter_neon_erased::<BD, 4, 8, 8, { 12 * 8 + 8 }>);
        self.fb[2] = cdef::Fn::new(cdef_filter_neon_erased::<BD, 4, 4, 8, { 12 * 8 + 8 }>);

        self
    }

    #[inline(always)]
    const fn init<BD: BitDepth>(self, flags: CpuFlags) -> Self {
        #[cfg(feature = "asm")]
        {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                return self.init_x86::<BD>(flags);
            }
            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            {
                return self.init_arm::<BD>(flags);
            }
        }

        #[allow(unreachable_code)] // Reachable on some #[cfg]s.
        {
            let _ = flags;
            self
        }
    }

    pub const fn new<BD: BitDepth>(flags: CpuFlags) -> Self {
        Self::default::<BD>().init::<BD>(flags)
    }
}

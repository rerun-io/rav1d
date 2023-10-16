use crate::include::common::bitdepth::ArrayDefault;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BPC;
use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Rav1dFilmGrainData;
use crate::include::dav1d::headers::RAV1D_MC_IDENTITY;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I444;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::src::align::Align16;
use crate::src::align::Align64;
use crate::src::filmgrain::Rav1dFilmGrainDSPContext;
use libc::intptr_t;
use libc::memcpy;
use libc::memset;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;

unsafe fn generate_scaling<BD: BitDepth>(
    bitdepth: c_int,
    points: *const [u8; 2],
    num: c_int,
    scaling: *mut u8,
) {
    let (shift_x, scaling_size) = match BD::BPC {
        BPC::BPC8 => (0, 256),
        BPC::BPC16 => {
            if !(bitdepth > 8) {
                unreachable!();
            }
            let shift_x = bitdepth - 8;
            let scaling_size = (1 as c_int) << bitdepth;
            (shift_x, scaling_size)
        }
    };
    if num == 0 {
        memset(scaling as *mut c_void, 0 as c_int, scaling_size as usize);
        return;
    }
    memset(
        scaling as *mut c_void,
        (*points.offset(0))[1] as c_int,
        (((*points.offset(0))[0] as c_int) << shift_x) as usize,
    );
    let mut i = 0;
    while i < num - 1 {
        let bx = (*points.offset(i as isize))[0] as c_int;
        let by = (*points.offset(i as isize))[1] as c_int;
        let ex = (*points.offset((i + 1) as isize))[0] as c_int;
        let ey = (*points.offset((i + 1) as isize))[1] as c_int;
        let dx = ex - bx;
        let dy = ey - by;
        if !(dx > 0) {
            unreachable!();
        }
        let delta = dy * ((0x10000 + (dx >> 1)) / dx);
        let mut x = 0;
        let mut d = 0x8000 as c_int;
        while x < dx {
            *scaling.offset((bx + x << shift_x) as isize) = (by + (d >> 16)) as u8;
            d += delta;
            x += 1;
        }
        i += 1;
    }
    let n = ((*points.offset((num - 1) as isize))[0] as c_int) << shift_x;
    memset(
        &mut *scaling.offset(n as isize) as *mut u8 as *mut c_void,
        (*points.offset((num - 1) as isize))[1] as c_int,
        (scaling_size - n) as usize,
    );

    if BD::BPC != BPC::BPC8 {
        let pad = (1 as c_int) << shift_x;
        let rnd = pad >> 1;
        let mut i = 0;
        while i < num - 1 {
            let bx = ((*points.offset(i as isize))[0] as c_int) << shift_x;
            let ex = ((*points.offset((i + 1) as isize))[0] as c_int) << shift_x;
            let dx = ex - bx;
            let mut x = 0;
            while x < dx {
                let range = *scaling.offset((bx + x + pad) as isize) as c_int
                    - *scaling.offset((bx + x) as isize) as c_int;
                let mut n = 1;
                let mut r = rnd;
                while n < pad {
                    r += range;
                    *scaling.offset((bx + x + n) as isize) =
                        (*scaling.offset((bx + x) as isize) as c_int + (r >> shift_x)) as u8;
                    n += 1;
                }
                x += pad;
            }
            i += 1;
        }
    }
}

pub(crate) unsafe fn rav1d_prep_grain<BD: BitDepth>(
    dsp: &Rav1dFilmGrainDSPContext,
    out: &mut Rav1dPicture,
    r#in: &Rav1dPicture,
    scaling: *mut BD::Scaling,
    grain_lut: *mut [[BD::Entry; 82]; 74],
) {
    let data: *const Rav1dFilmGrainData = &mut (*out.frame_hdr).film_grain.data;
    let bitdepth_max = ((1 as c_int) << out.p.bpc) - 1;
    (dsp.generate_grain_y).expect("non-null function pointer")(
        (*grain_lut.offset(0)).as_mut_ptr().cast(),
        data,
        bitdepth_max,
    );
    if (*data).num_uv_points[0] != 0 || (*data).chroma_scaling_from_luma != 0 {
        (dsp.generate_grain_uv
            [(r#in.p.layout as c_uint).wrapping_sub(1 as c_int as c_uint) as usize])
            .expect("non-null function pointer")(
            (*grain_lut.offset(1)).as_mut_ptr().cast(),
            (*grain_lut.offset(0)).as_mut_ptr().cast(),
            data,
            0 as c_int as intptr_t,
            bitdepth_max,
        );
    }
    if (*data).num_uv_points[1] != 0 || (*data).chroma_scaling_from_luma != 0 {
        (dsp.generate_grain_uv
            [(r#in.p.layout as c_uint).wrapping_sub(1 as c_int as c_uint) as usize])
            .expect("non-null function pointer")(
            (*grain_lut.offset(2)).as_mut_ptr().cast(),
            (*grain_lut.offset(0)).as_mut_ptr().cast(),
            data,
            1 as c_int as intptr_t,
            bitdepth_max,
        );
    }
    if (*data).num_y_points != 0 || (*data).chroma_scaling_from_luma != 0 {
        generate_scaling::<BD>(
            r#in.p.bpc,
            ((*data).y_points).as_ptr(),
            (*data).num_y_points,
            (*scaling.offset(0)).as_mut().as_mut_ptr(),
        );
    }
    if (*data).num_uv_points[0] != 0 {
        generate_scaling::<BD>(
            r#in.p.bpc,
            ((*data).uv_points[0]).as_ptr(),
            (*data).num_uv_points[0],
            (*scaling.offset(1)).as_mut().as_mut_ptr(),
        );
    }
    if (*data).num_uv_points[1] != 0 {
        generate_scaling::<BD>(
            r#in.p.bpc,
            ((*data).uv_points[1]).as_ptr(),
            (*data).num_uv_points[1],
            (*scaling.offset(2)).as_mut().as_mut_ptr(),
        );
    }
    if !(out.stride[0] == r#in.stride[0]) {
        unreachable!();
    }
    if (*data).num_y_points == 0 {
        let stride: ptrdiff_t = out.stride[0];
        let sz: ptrdiff_t = out.p.h as isize * stride;
        if sz < 0 {
            memcpy(
                (out.data[0] as *mut u8)
                    .offset(sz as isize)
                    .offset(-(stride as isize)) as *mut c_void,
                (r#in.data[0] as *mut u8)
                    .offset(sz as isize)
                    .offset(-(stride as isize)) as *const c_void,
                -sz as usize,
            );
        } else {
            memcpy(out.data[0], r#in.data[0], sz as usize);
        }
    }
    if r#in.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint
        && (*data).chroma_scaling_from_luma == 0
    {
        if !(out.stride[1] == r#in.stride[1]) {
            unreachable!();
        }
        let ss_ver =
            (r#in.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
        let stride: ptrdiff_t = out.stride[1];
        let sz: ptrdiff_t = (out.p.h + ss_ver >> ss_ver) as isize * stride;
        if sz < 0 {
            if (*data).num_uv_points[0] == 0 {
                memcpy(
                    (out.data[1] as *mut u8)
                        .offset(sz as isize)
                        .offset(-(stride as isize)) as *mut c_void,
                    (r#in.data[1] as *mut u8)
                        .offset(sz as isize)
                        .offset(-(stride as isize)) as *const c_void,
                    -sz as usize,
                );
            }
            if (*data).num_uv_points[1] == 0 {
                memcpy(
                    (out.data[2] as *mut u8)
                        .offset(sz as isize)
                        .offset(-(stride as isize)) as *mut c_void,
                    (r#in.data[2] as *mut u8)
                        .offset(sz as isize)
                        .offset(-(stride as isize)) as *const c_void,
                    -sz as usize,
                );
            }
        } else {
            if (*data).num_uv_points[0] == 0 {
                memcpy(out.data[1], r#in.data[1], sz as usize);
            }
            if (*data).num_uv_points[1] == 0 {
                memcpy(out.data[2], r#in.data[2], sz as usize);
            }
        }
    }
}

pub(crate) unsafe fn rav1d_apply_grain_row<BD: BitDepth>(
    dsp: &Rav1dFilmGrainDSPContext,
    out: &mut Rav1dPicture,
    r#in: &Rav1dPicture,
    scaling: *const BD::Scaling,
    grain_lut: *const [[BD::Entry; 82]; 74],
    row: c_int,
) {
    let data: *const Dav1dFilmGrainData = &mut (*out.frame_hdr).film_grain.data;
    let ss_y = (r#in.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
    let ss_x = (r#in.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint) as c_int;
    let cpw = out.p.w + ss_x >> ss_x;
    let is_id = ((*out.seq_hdr).mtrx as c_uint == RAV1D_MC_IDENTITY as c_int as c_uint) as c_int;
    let luma_src: *mut BD::Pixel = (r#in.data[0] as *mut BD::Pixel)
        .offset(((row * 32) as isize * BD::pxstride(r#in.stride[0] as usize) as isize) as isize);
    let bitdepth_max = ((1 as c_int) << out.p.bpc) - 1;
    if (*data).num_y_points != 0 {
        let bh = cmp::min(out.p.h - row * 32, 32 as c_int);
        (dsp.fgy_32x32xn).expect("non-null function pointer")(
            (out.data[0] as *mut BD::Pixel)
                .offset(
                    ((row * 32) as isize * BD::pxstride(out.stride[0] as usize) as isize) as isize,
                )
                .cast(),
            luma_src.cast(),
            out.stride[0],
            data,
            out.p.w as usize,
            (*scaling.offset(0)).as_ref().as_ptr(),
            (*grain_lut.offset(0)).as_ptr().cast(),
            bh,
            row,
            bitdepth_max,
        );
    }
    if (*data).num_uv_points[0] == 0
        && (*data).num_uv_points[1] == 0
        && (*data).chroma_scaling_from_luma == 0
    {
        return;
    }
    let bh = cmp::min(out.p.h - row * 32, 32 as c_int) + ss_y >> ss_y;
    if out.p.w & ss_x != 0 {
        let mut ptr: *mut BD::Pixel = luma_src;
        let mut y = 0;
        while y < bh {
            *ptr.offset(out.p.w as isize) = *ptr.offset((out.p.w - 1) as isize);
            ptr = ptr.offset(((BD::pxstride(r#in.stride[0] as usize) as isize) << ss_y) as isize);
            y += 1;
        }
    }
    let uv_off: ptrdiff_t =
        (row * 32) as isize * BD::pxstride(out.stride[1] as usize) as isize >> ss_y;
    if (*data).chroma_scaling_from_luma != 0 {
        let mut pl = 0;
        while pl < 2 {
            (dsp.fguv_32x32xn
                [(r#in.p.layout as c_uint).wrapping_sub(1 as c_int as c_uint) as usize])
                .expect("non-null function pointer")(
                (out.data[(1 + pl) as usize] as *mut BD::Pixel)
                    .offset(uv_off as isize)
                    .cast(),
                (r#in.data[(1 + pl) as usize] as *const BD::Pixel)
                    .offset(uv_off as isize)
                    .cast(),
                r#in.stride[1],
                data,
                cpw as usize,
                (*scaling.offset(0)).as_ref().as_ptr(),
                (*grain_lut.offset((1 + pl) as isize)).as_ptr().cast(),
                bh,
                row,
                luma_src.cast(),
                r#in.stride[0],
                pl,
                is_id,
                bitdepth_max,
            );
            pl += 1;
        }
    } else {
        let mut pl = 0;
        while pl < 2 {
            if (*data).num_uv_points[pl as usize] != 0 {
                (dsp.fguv_32x32xn
                    [(r#in.p.layout as c_uint).wrapping_sub(1 as c_int as c_uint) as usize])
                    .expect("non-null function pointer")(
                    (out.data[(1 + pl) as usize] as *mut BD::Pixel)
                        .offset(uv_off as isize)
                        .cast(),
                    (r#in.data[(1 + pl) as usize] as *const BD::Pixel)
                        .offset(uv_off as isize)
                        .cast(),
                    r#in.stride[1],
                    data,
                    cpw as usize,
                    (*scaling.offset((1 + pl) as isize)).as_ref().as_ptr(),
                    (*grain_lut.offset((1 + pl) as isize)).as_ptr().cast(),
                    bh,
                    row,
                    luma_src.cast(),
                    r#in.stride[0],
                    pl,
                    is_id,
                    bitdepth_max,
                );
            }
            pl += 1;
        }
    };
}

pub(crate) unsafe fn rav1d_apply_grain<BD: BitDepth>(
    dsp: &Rav1dFilmGrainDSPContext,
    out: &mut Rav1dPicture,
    r#in: &Rav1dPicture,
) {
    let mut grain_lut = Align16([[[Default::default(); 82]; 74]; 3]);
    // Only `x86_64` [`BitDepth8`] needs [`Align64`],
    // but it shouldn't be a problem to over-align.
    // [`GrainLutScaling::scaling`] over-aligns, for example.
    let mut scaling = Align64([ArrayDefault::default(); 3]);
    let rows = out.p.h + 31 >> 5;
    rav1d_prep_grain::<BD>(
        dsp,
        out,
        r#in,
        scaling.0.as_mut_ptr(),
        grain_lut.0.as_mut_ptr(),
    );
    let mut row = 0;
    while row < rows {
        rav1d_apply_grain_row::<BD>(
            dsp,
            out,
            r#in,
            scaling.0.as_mut_ptr() as *const BD::Scaling,
            grain_lut.0.as_mut_ptr() as *const [[BD::Entry; 82]; 74],
            row,
        );
        row += 1;
    }
}

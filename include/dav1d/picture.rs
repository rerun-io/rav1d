#![deny(unsafe_op_in_unsafe_fn)]

use crate::include::common::bitdepth::BitDepth;
use crate::include::common::validate::validate_input;
use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::headers::DRav1d;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dContentLightLevel;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dITUTT35;
use crate::include::dav1d::headers::Rav1dMasteringDisplay;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::src::assume::assume;
use crate::src::c_arc::RawArc;
use crate::src::disjoint_mut::AsMutPtr;
use crate::src::disjoint_mut::DisjointImmutGuard;
use crate::src::disjoint_mut::DisjointMut;
use crate::src::disjoint_mut::DisjointMutGuard;
use crate::src::disjoint_mut::SliceBounds;
use crate::src::error::Dav1dResult;
use crate::src::error::Rav1dError;
use crate::src::error::Rav1dError::EINVAL;
use crate::src::error::Rav1dResult;
use libc::ptrdiff_t;
use libc::uintptr_t;
use std::array;
use std::ffi::c_int;
use std::ffi::c_void;
use std::mem;
use std::ptr::NonNull;
use std::sync::Arc;
use to_method::To as _;
use zerocopy::AsBytes;
use zerocopy::FromBytes;
use zerocopy::FromZeroes;

pub(crate) const RAV1D_PICTURE_ALIGNMENT: usize = 64;
pub const DAV1D_PICTURE_ALIGNMENT: usize = RAV1D_PICTURE_ALIGNMENT;

#[derive(Default)]
#[repr(C)]
pub struct Dav1dPictureParameters {
    pub w: c_int,
    pub h: c_int,
    pub layout: Dav1dPixelLayout,
    pub bpc: c_int,
}

// TODO(kkysen) Eventually the [`impl Default`] might not be needed.
#[derive(Clone, Default)]
#[repr(C)]
pub(crate) struct Rav1dPictureParameters {
    pub w: c_int,
    pub h: c_int,
    pub layout: Rav1dPixelLayout,
    pub bpc: u8,
}

impl From<Dav1dPictureParameters> for Rav1dPictureParameters {
    fn from(value: Dav1dPictureParameters) -> Self {
        let Dav1dPictureParameters { w, h, layout, bpc } = value;
        Self {
            w,
            h,
            layout: layout.try_into().unwrap(),
            bpc: bpc.try_into().unwrap(),
        }
    }
}

impl From<Rav1dPictureParameters> for Dav1dPictureParameters {
    fn from(value: Rav1dPictureParameters) -> Self {
        let Rav1dPictureParameters { w, h, layout, bpc } = value;
        Self {
            w,
            h,
            layout: layout.into(),
            bpc: bpc.into(),
        }
    }
}

#[derive(Default)]
#[repr(C)]
pub struct Dav1dPicture {
    pub seq_hdr: Option<NonNull<Dav1dSequenceHeader>>,
    pub frame_hdr: Option<NonNull<Dav1dFrameHeader>>,
    pub data: [Option<NonNull<c_void>>; 3],
    pub stride: [ptrdiff_t; 2],
    pub p: Dav1dPictureParameters,
    pub m: Dav1dDataProps,
    pub content_light: Option<NonNull<Rav1dContentLightLevel>>,
    pub mastering_display: Option<NonNull<Rav1dMasteringDisplay>>,
    pub itut_t35: Option<NonNull<Dav1dITUTT35>>,
    pub n_itut_t35: usize,
    pub reserved: [uintptr_t; 4],
    pub frame_hdr_ref: Option<RawArc<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>>, // opaque, so we can change this
    pub seq_hdr_ref: Option<RawArc<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>>, // opaque, so we can change this
    pub content_light_ref: Option<RawArc<Rav1dContentLightLevel>>, // opaque, so we can change this
    pub mastering_display_ref: Option<RawArc<Rav1dMasteringDisplay>>, // opaque, so we can change this
    pub itut_t35_ref: Option<RawArc<DRav1d<Box<[Rav1dITUTT35]>, Box<[Dav1dITUTT35]>>>>, // opaque, so we can change this
    pub reserved_ref: [uintptr_t; 4],
    pub r#ref: Option<RawArc<Rav1dPictureData>>, // opaque, so we can change this
    pub allocator_data: Option<NonNull<c_void>>,
}

#[derive(Clone, FromZeroes, FromBytes, AsBytes)]
#[repr(C, align(64))]
pub struct AlignedPixelChunk([u8; RAV1D_PICTURE_ALIGNMENT]);

const _: () = assert!(mem::align_of::<AlignedPixelChunk>() == RAV1D_PICTURE_ALIGNMENT);
const _: () = assert!(mem::size_of::<AlignedPixelChunk>() == RAV1D_PICTURE_ALIGNMENT);

pub struct Rav1dPictureDataComponentInner {
    /// A ptr to the start of this slice of [`BitDepth::Pixel`]s*,
    /// even if [`Self::stride`] is negative.
    ptr: NonNull<AlignedPixelChunk>,

    /// The length of [`Self::ptr`] in [`AlignedPixelChunk`] chunks.
    len: usize,

    /// The stride of [`Self::ptr`] in [`AlignedPixelChunk`] chunks.
    stride: isize,
}

impl Rav1dPictureDataComponentInner {
    /// `len` and `stride` are in terms of [`u8`] bytes.
    ///
    /// # Safety
    ///
    /// `ptr`, `len`, and `stride` must follow the requirements of [`Dav1dPicAllocator::alloc_picture_callback`].
    unsafe fn new(ptr: NonNull<AlignedPixelChunk>, len: usize, stride: isize) -> Self {
        let ptr = if stride < 0 {
            let ptr = ptr.as_ptr();
            // SAFETY: According to `Dav1dPicAllocator::alloc_picture_callback`,
            // if the `stride` is negative, this is how we get the start of the data.
            // `.offset(-stride)` puts us at one element past the end of the slice,
            // and `.sub(len)` puts us back at the start of the slice.
            let ptr = unsafe { ptr.byte_offset(-stride).byte_sub(len) };
            NonNull::new(ptr).unwrap()
        } else {
            ptr
        };
        assert!(ptr.is_aligned());
        const CHUNK_SIZE: usize = mem::size_of::<AlignedPixelChunk>();
        const _: () = assert!(CHUNK_SIZE == mem::align_of::<AlignedPixelChunk>());
        // Guaranteed by `Dav1dPicAllocator::alloc_picture_callback`.
        assert!(len % CHUNK_SIZE == 0);
        let len = len / CHUNK_SIZE;
        // Guaranteed by `Dav1dPicAllocator::alloc_picture_callback` to be a multiple of 128 pixels.
        assert!(stride % CHUNK_SIZE as isize == 0);
        let stride = stride / CHUNK_SIZE as isize;
        Self { ptr, len, stride }
    }
}

// SAFETY: We only store the raw pointer, so we never materialize a `&mut`.
unsafe impl AsMutPtr for Rav1dPictureDataComponentInner {
    type Target = u8;

    #[inline] // Inline so callers can see our over-alignment.
    unsafe fn as_mut_ptr(ptr: *mut Self) -> *mut Self::Target {
        // SAFETY: Safe to dereference by unsafe preconditions.
        // Since we don't store any `&mut`s, just a raw ptr, we can have a `&Self`.
        let this = unsafe { &*ptr };

        // Assume this so that the compiler knows `ptr` is aligned.
        // Normally we'd store this as a slice so the compiler would know,
        // but since it's a ptr due to `DisjointMut`, we explicitly assume it here.
        // SAFETY: We already checked this in `Self::new`.
        unsafe { assume(this.ptr.is_aligned()) };

        this.ptr.cast::<u8>().as_ptr()
    }

    fn len(&self) -> usize {
        self.len * mem::size_of::<AlignedPixelChunk>()
    }
}

pub struct Rav1dPictureDataComponent(DisjointMut<Rav1dPictureDataComponentInner>);

impl Rav1dPictureDataComponent {
    fn chunk_len(&self) -> usize {
        // SAFETY: We're only accessing the `stride` fields, not `ptr`.
        unsafe { (*self.0.inner()).len }
    }

    /// Length in number of pixels.
    pub fn len<BD: BitDepth>(&self) -> usize {
        self.chunk_len() * (mem::size_of::<AlignedPixelChunk>() / mem::size_of::<BD::Pixel>())
    }

    fn chunk_stride(&self) -> isize {
        // SAFETY: We're only accessing the `stride` fields, not `ptr`.
        unsafe { (*self.0.inner()).stride }
    }

    /// Stride in number of pixels.
    pub fn stride<BD: BitDepth>(&self) -> isize {
        self.chunk_stride()
            * (mem::size_of::<AlignedPixelChunk>() / mem::size_of::<BD::Pixel>()) as isize
    }

    /// Strided ptr to chunks.
    fn as_chunk_mut_ptr(&self) -> *mut AlignedPixelChunk {
        let ptr = self.0.as_mut_ptr().cast::<AlignedPixelChunk>();
        let stride = self.chunk_stride();
        if stride < 0 {
            // SAFETY: This puts `ptr` one element past the end of the slice of pixels.
            let ptr = unsafe { ptr.add(self.chunk_len()) };
            // SAFETY: `stride` is negative and `-stride < len`, so this should stay in bounds.
            let ptr = unsafe { ptr.offset(stride) };
            ptr
        } else {
            ptr
        }
    }

    /// Strided ptr to pixels.
    pub fn as_mut_ptr<BD: BitDepth>(&self) -> *mut BD::Pixel {
        self.as_chunk_mut_ptr().cast()
    }

    /// Strided ptr to pixels.
    pub fn as_ptr<BD: BitDepth>(&self) -> *const BD::Pixel {
        self.as_mut_ptr::<BD>().cast_const()
    }

    fn as_dav1d(&self) -> Option<NonNull<c_void>> {
        if self.chunk_len() == 0 {
            None
        } else {
            NonNull::new(self.as_chunk_mut_ptr().cast())
        }
    }

    pub fn copy_from(&self, src: &Self) {
        let dst = &mut *self.0.index_mut(..);
        let src = &*src.0.index(..);
        dst.clone_from_slice(src);
    }

    #[inline] // Inline to see bounds checks in order to potentially elide them.
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn index<'a, BD: BitDepth>(
        &'a self,
        index: usize,
    ) -> DisjointImmutGuard<'a, Rav1dPictureDataComponentInner, BD::Pixel> {
        self.0.element_as(index)
    }

    #[inline] // Inline to see bounds checks in order to potentially elide them.
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn index_mut<'a, BD: BitDepth>(
        &'a self,
        index: usize,
    ) -> DisjointMutGuard<'a, Rav1dPictureDataComponentInner, BD::Pixel> {
        self.0.mut_element_as(index)
    }

    #[inline] // Inline to see bounds checks in order to potentially elide them.
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn slice<'a, BD, I>(
        &'a self,
        index: I,
    ) -> DisjointImmutGuard<'a, Rav1dPictureDataComponentInner, [BD::Pixel]>
    where
        BD: BitDepth,
        I: SliceBounds,
    {
        self.0.slice_as(index)
    }

    #[inline] // Inline to see bounds checks in order to potentially elide them.
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn slice_mut<'a, BD, I>(
        &'a self,
        index: I,
    ) -> DisjointMutGuard<'a, Rav1dPictureDataComponentInner, [BD::Pixel]>
    where
        BD: BitDepth,
        I: SliceBounds,
    {
        self.0.mut_slice_as(index)
    }
}

pub struct Rav1dPictureData {
    pub data: [Rav1dPictureDataComponent; 3],
    pub(crate) allocator_data: Option<NonNull<c_void>>,
    pub(crate) allocator: Rav1dPicAllocator,
}

impl Drop for Rav1dPictureData {
    fn drop(&mut self) {
        let Self {
            data,
            allocator_data,
            allocator,
        } = self;
        allocator.dealloc_picture_data(data, *allocator_data);
    }
}

// TODO(kkysen) Eventually the [`impl Default`] might not be needed.
// It's needed currently for a [`mem::take`] that simulates a move,
// but once everything is Rusty, we may not need to clear the `dst` anymore.
// This also applies to the `#[derive(Default)]`
// on [`Rav1dPictureParameters`] and [`Rav1dPixelLayout`].
#[derive(Clone, Default)]
#[repr(C)]
pub(crate) struct Rav1dPicture {
    pub seq_hdr: Option<Arc<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>>,
    pub frame_hdr: Option<Arc<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>>,
    pub data: Option<Arc<Rav1dPictureData>>,
    pub stride: [ptrdiff_t; 2],
    pub p: Rav1dPictureParameters,
    pub m: Rav1dDataProps,
    pub content_light: Option<Arc<Rav1dContentLightLevel>>,
    pub mastering_display: Option<Arc<Rav1dMasteringDisplay>>,
    pub itut_t35: Arc<DRav1d<Box<[Rav1dITUTT35]>, Box<[Dav1dITUTT35]>>>,
}

impl From<Dav1dPicture> for Rav1dPicture {
    fn from(value: Dav1dPicture) -> Self {
        let Dav1dPicture {
            seq_hdr: _,
            frame_hdr: _,
            data: _,
            stride,
            p,
            m,
            content_light: _,
            mastering_display: _,
            itut_t35: _,
            n_itut_t35: _,
            reserved: _,
            frame_hdr_ref,
            seq_hdr_ref,
            content_light_ref,
            mastering_display_ref,
            itut_t35_ref,
            reserved_ref: _,
            r#ref: data_ref,
            allocator_data: _,
        } = value;
        Self {
            // We don't `.update_rav1d()` [`Rav1dSequenceHeader`] because it's meant to be read-only.
            // Safety: `raw` came from [`RawArc::from_arc`].
            seq_hdr: seq_hdr_ref.map(|raw| unsafe { raw.into_arc() }),
            // We don't `.update_rav1d()` [`Rav1dFrameHeader`] because it's meant to be read-only.
            // Safety: `raw` came from [`RawArc::from_arc`].
            frame_hdr: frame_hdr_ref.map(|raw| unsafe { raw.into_arc() }),
            // Safety: `raw` came from [`RawArc::from_arc`].
            data: data_ref.map(|raw| unsafe { raw.into_arc() }),
            stride,
            p: p.into(),
            m: m.into(),
            // Safety: `raw` came from [`RawArc::from_arc`].
            content_light: content_light_ref.map(|raw| unsafe { raw.into_arc() }),
            // Safety: `raw` came from [`RawArc::from_arc`].
            mastering_display: mastering_display_ref.map(|raw| unsafe { raw.into_arc() }),
            // We don't `.update_rav1d` [`Rav1dITUTT35`] because never read it.
            // Safety: `raw` came from [`RawArc::from_arc`].
            itut_t35: itut_t35_ref
                .map(|raw| unsafe { raw.into_arc() })
                .unwrap_or_default(),
        }
    }
}

impl From<Rav1dPicture> for Dav1dPicture {
    fn from(value: Rav1dPicture) -> Self {
        let Rav1dPicture {
            seq_hdr,
            frame_hdr,
            data,
            stride,
            p,
            m,
            content_light,
            mastering_display,
            itut_t35,
        } = value;
        Self {
            // [`DRav1d::from_rav1d`] is called right after [`parse_seq_hdr`].
            seq_hdr: seq_hdr.as_ref().map(|arc| (&arc.as_ref().dav1d).into()),
            // [`DRav1d::from_rav1d`] is called in [`parse_frame_hdr`].
            frame_hdr: frame_hdr.as_ref().map(|arc| (&arc.as_ref().dav1d).into()),
            data: data
                .as_ref()
                .map(|arc| arc.data.each_ref().map(|data| data.as_dav1d()))
                .unwrap_or_default(),
            stride,
            p: p.into(),
            m: m.into(),
            content_light: content_light.as_ref().map(|arc| arc.as_ref().into()),
            mastering_display: mastering_display.as_ref().map(|arc| arc.as_ref().into()),
            // [`DRav1d::from_rav1d`] is called in [`rav1d_parse_obus`].
            itut_t35: Some(NonNull::new(itut_t35.dav1d.as_ptr().cast_mut()).unwrap()),
            n_itut_t35: itut_t35.len(),
            reserved: Default::default(),
            frame_hdr_ref: frame_hdr.map(RawArc::from_arc),
            seq_hdr_ref: seq_hdr.map(RawArc::from_arc),
            content_light_ref: content_light.map(RawArc::from_arc),
            mastering_display_ref: mastering_display.map(RawArc::from_arc),
            itut_t35_ref: Some(itut_t35).map(RawArc::from_arc),
            reserved_ref: Default::default(),
            // Order flipped so that the borrow comes before the move.
            allocator_data: data.as_ref().and_then(|arc| arc.allocator_data),
            r#ref: data.map(RawArc::from_arc),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dPicAllocator {
    /// Custom data to pass to the allocator callbacks.
    pub cookie: *mut c_void,

    /// Allocate the picture buffer based on the [`Dav1dPictureParameters`].
    ///
    /// [`data`]`[0]`, [`data`]`[1]` and [`data`]`[2]`
    /// must be [`DAV1D_PICTURE_ALIGNMENT`]-byte aligned
    /// and with a pixel width/height multiple of 128 pixels.
    /// Any allocated memory area should also be padded by [`DAV1D_PICTURE_ALIGNMENT`] bytes.
    /// [`data`]`[1]` and [`data`]`[2]` must share the same [`stride`]`[1]`.
    ///
    /// # Safety
    ///
    /// If frame threading is used, accesses to [`Self::cookie`] must be thread-safe.
    ///
    /// ### Additional `rav1d` requirement:
    ///
    /// The allocated data must be initialized.
    /// If newly (e.x. not reused) allocated data is zero initialized using OS APIs,
    /// it is possible for this to not be slower than an uninitialized allocation.
    /// For example, see `dav1d_default_picture_alloc` and `MemPool::pop_init`.
    ///
    /// If the allocated data is not initialized,
    /// it is possible there will be reads of uninitialized data.
    /// `rav1d` should not read this data before writing to it first,
    /// but it does not guarantee that it does so.
    /// Instead, initializing the allocated data guarantees all uses of it will be sound.
    ///
    /// # Args
    ///
    /// * `pic`: The picture to allocate the buffer for.
    ///     The callback needs to fill the picture
    ///     [`data`]`[0]`, [`data`]`[1]`, [`data`]`[2]`,
    ///     [`stride`]`[0]`, and [`stride`]`[1]`.
    ///     The allocator can fill the pic [`allocator_data`] pointer
    ///     with a custom pointer that will be passed to
    ///     [`release_picture_callback`].
    ///
    ///     The only fields of `pic` that will be already set are:
    ///     * [`Dav1dPicture::p`]
    ///     * [`Dav1dPicture::seq_hdr`]
    ///     * [`Dav1dPicture::frame_hdr`]
    ///     
    ///     This is not a change from the original `DAV1D_API`,
    ///     just a clarification of it.
    ///
    /// * `cookie`: Custom pointer passed to all calls.
    ///
    /// *Note*: No fields other than [`data`], [`stride`] and [`allocator_data`]
    /// must be filled by this callback.
    ///
    /// # Return
    ///
    /// 0 on success. A negative `DAV1D_ERR` value on error.
    /// <!--- TODO(kkysen) Translate `DAV1D_ERR` -->
    ///
    /// [`data`]: Dav1dPicture::data
    /// [`stride`]: Dav1dPicture::data
    /// [`allocator_data`]: Dav1dPicture::allocator_data
    /// [`release_picture_callback`]: Self::release_picture_callback
    pub alloc_picture_callback:
        Option<unsafe extern "C" fn(pic: *mut Dav1dPicture, cookie: *mut c_void) -> Dav1dResult>,

    /// Release the picture buffer.
    ///
    /// # Safety
    ///
    /// If frame threading is used, accesses to `cookie` must be thread-safe.
    ///
    /// If frame threading is used, this function may be called by the main thread
    /// (the thread which calls [`dav1d_get_picture`]),
    /// or any of the frame threads and thus must be thread-safe.
    /// If frame threading is not used, this function will only be called on the main thread.
    ///
    /// # Args
    ///
    /// * `pic`: The picture that was filled by [`alloc_picture_callback`].
    ///     
    ///     The only fields of `pic` that will be set are
    ///     the ones allocated by [`Self::alloc_picture_callback`]:
    ///     * [`Dav1dPicture::data`]
    ///     * [`Dav1dPicture::allocator_data`]
    ///     
    ///     NOTE: This is a slight change from the original `DAV1D_API`, which was underspecified.
    ///     However, all known uses of this API follow this already:
    ///     * `libdav1d`: [`dav1d_default_picture_release`](https://code.videolan.org/videolan/dav1d/-/blob/16ed8e8b99f2fcfffe016e929d3626e15267ad3e/src/picture.c#L85-87)
    ///     * `dav1d`: [`picture_release`](https://code.videolan.org/videolan/dav1d/-/blob/16ed8e8b99f2fcfffe016e929d3626e15267ad3e/tools/dav1d.c#L180-182)
    ///     * `dav1dplay`: [`placebo_release_pic`](https://code.videolan.org/videolan/dav1d/-/blob/16ed8e8b99f2fcfffe016e929d3626e15267ad3e/examples/dp_renderer_placebo.c#L375-383)
    ///     * `libplacebo`: [`pl_release_dav1dpicture`](https://github.com/haasn/libplacebo/blob/34e019bfedaa5a64f268d8f9263db352c0a8f67f/src/include/libplacebo/utils/dav1d_internal.h#L594-L607)
    ///     * `ffmpeg`: [`libdav1d_picture_release`](https://github.com/FFmpeg/FFmpeg/blob/00b288da73f45acb78b74bcc40f73c7ba1fff7cb/libavcodec/libdav1d.c#L124-L129)
    ///
    ///     Making this API safe without this slight tightening of the API
    ///     [is very difficult](https://github.com/memorysafety/rav1d/pull/685#discussion_r1458171639).
    ///
    /// * `cookie`: Custom pointer passed to all calls.
    ///
    /// [`dav1d_get_picture`]: crate::src::lib::dav1d_get_picture
    /// [`alloc_picture_callback`]: Self::alloc_picture_callback
    pub release_picture_callback:
        Option<unsafe extern "C" fn(pic: *mut Dav1dPicture, cookie: *mut c_void) -> ()>,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dPicAllocator {
    /// See [`Dav1dPicAllocator::cookie`].
    ///
    /// # Safety
    ///
    /// If [`Self::is_default`]`()`, then this cookie is a reference to
    /// [`Rav1dContext::picture_pool`], a `&Arc<MemPool<u8>`.
    /// Thus, its lifetime is that of `&c.picture_pool`,
    /// so the lifetime of the `&`[`Rav1dContext`].
    /// This is used from `dav1d_default_picture_alloc`
    /// ([`Self::default`]`().alloc_picture_callback`),
    /// which is called from [`Self::alloc_picture_data`],
    /// which is called further up on the call stack with a `&`[`Rav1dContext`].
    /// Thus, the lifetime will always be valid where used.
    ///
    /// Note that this is an `&Arc<MemPool<u8>` turned into a raw pointer,
    /// not an [`Arc::into_raw`] of that [`Arc`].
    /// This is because storing the [`Arc`] would require C to
    /// free data owned by a [`Dav1dPicAllocator`] potentially,
    /// which it may not do, as there are no current APIs for doing so.
    ///
    /// [`Rav1dContext::picture_pool`]: crate::src::internal::Rav1dContext::picture_pool
    /// [`Rav1dContext`]: crate::src::internal::Rav1dContext
    pub cookie: *mut c_void,

    /// See [`Dav1dPicAllocator::alloc_picture_callback`].
    ///
    /// # Safety
    ///
    /// `pic` is passed as a `&mut`.
    ///
    /// If frame threading is used, accesses to [`Self::cookie`] must be thread-safe,
    /// i.e. [`Self::cookie`] must be [`Send`]` + `[`Sync`].
    pub alloc_picture_callback:
        unsafe extern "C" fn(pic: *mut Dav1dPicture, cookie: *mut c_void) -> Dav1dResult,

    /// See [`Dav1dPicAllocator::release_picture_callback`].
    ///
    /// # Safety
    ///
    /// `pic` is passed as a `&mut`.
    ///
    /// If frame threading is used, accesses to [`Self::cookie`] must be thread-safe,
    /// i.e. [`Self::cookie`] must be [`Send`]` + `[`Sync`].
    pub release_picture_callback:
        unsafe extern "C" fn(pic: *mut Dav1dPicture, cookie: *mut c_void) -> (),
}

impl TryFrom<Dav1dPicAllocator> for Rav1dPicAllocator {
    type Error = Rav1dError;

    fn try_from(value: Dav1dPicAllocator) -> Result<Self, Self::Error> {
        let Dav1dPicAllocator {
            cookie,
            alloc_picture_callback,
            release_picture_callback,
        } = value;
        Ok(Self {
            cookie,
            alloc_picture_callback: validate_input!(alloc_picture_callback.ok_or(EINVAL))?,
            release_picture_callback: validate_input!(release_picture_callback.ok_or(EINVAL))?,
        })
    }
}

impl From<Rav1dPicAllocator> for Dav1dPicAllocator {
    fn from(value: Rav1dPicAllocator) -> Self {
        let Rav1dPicAllocator {
            cookie,
            alloc_picture_callback,
            release_picture_callback,
        } = value;
        Self {
            cookie,
            alloc_picture_callback: Some(alloc_picture_callback),
            release_picture_callback: Some(release_picture_callback),
        }
    }
}

impl Rav1dPicAllocator {
    pub fn alloc_picture_data(
        &self,
        w: c_int,
        h: c_int,
        seq_hdr: Arc<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>,
        frame_hdr: Option<Arc<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>>,
    ) -> Rav1dResult<Rav1dPicture> {
        let pic = Rav1dPicture {
            p: Rav1dPictureParameters {
                w,
                h,
                layout: seq_hdr.layout,
                bpc: 8 + 2 * seq_hdr.hbd,
            },
            seq_hdr: Some(seq_hdr),
            frame_hdr,
            ..Default::default()
        };
        let mut pic_c = pic.to::<Dav1dPicture>();
        // Safety: `pic_c` is a valid `Dav1dPicture` with `data`, `stride`, `allocator_data` unset.
        let result = unsafe { (self.alloc_picture_callback)(&mut pic_c, self.cookie) };
        result.try_to::<Rav1dResult>().unwrap()?;
        // `data`, `stride`, and `allocator_data` are the only fields set by the allocator.
        // Of those, only `data` and `allocator_data` are read through `r#ref`,
        // so we need to read those directly first and allocate the `Arc`.
        let data = pic_c.data;
        let allocator_data = pic_c.allocator_data;
        let mut pic = pic_c.to::<Rav1dPicture>();
        let len = pic.p.pic_len(pic.stride);
        // TODO fallible allocation
        pic.data = Some(Arc::new(Rav1dPictureData {
            data: array::from_fn(|i| {
                // SAFETY: We'll check size and alignment in `Rav1dPictureDataComponentInner::new`.
                let ptr = data[i].map(|ptr| ptr.cast::<AlignedPixelChunk>());
                // Need to cast before `NonNull::dangling` to get the right alignment.
                let ptr = ptr.unwrap_or_else(NonNull::dangling);
                let len = len[(i != 0) as usize];
                let stride = pic.stride[(i != 0) as usize];
                // SAFETY: These args come from `Self::alloc_picture_callback`.
                let component = unsafe { Rav1dPictureDataComponentInner::new(ptr, len, stride) };
                Rav1dPictureDataComponent(DisjointMut::new(component))
            }),
            allocator_data,
            allocator: self.clone(),
        }));
        Ok(pic)
    }

    pub fn dealloc_picture_data(
        &self,
        data: &mut [Rav1dPictureDataComponent; 3],
        allocator_data: Option<NonNull<c_void>>,
    ) {
        let data = data.each_mut().map(|data| data.as_dav1d());
        let mut pic_c = Dav1dPicture {
            data,
            allocator_data,
            ..Default::default()
        };
        // Safety: `pic_c` contains the same `data` and `allocator_data`
        // that `Self::alloc_picture_data` set, which now get deallocated here.
        unsafe {
            (self.release_picture_callback)(&mut pic_c, self.cookie);
        }
    }
}

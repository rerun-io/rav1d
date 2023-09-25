use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::dav1d::Dav1dRef;
use crate::include::dav1d::headers::Dav1dContentLightLevel;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::src::r#ref::Rav1dRef;
use libc::ptrdiff_t;
use libc::uintptr_t;
use std::ffi::c_int;
use std::ffi::c_void;

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dPictureParameters {
    pub w: c_int,
    pub h: c_int,
    pub layout: Dav1dPixelLayout,
    pub bpc: c_int,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dPictureParameters {
    pub w: c_int,
    pub h: c_int,
    pub layout: Dav1dPixelLayout,
    pub bpc: c_int,
}

impl From<Dav1dPictureParameters> for Rav1dPictureParameters {
    fn from(value: Dav1dPictureParameters) -> Self {
        let Dav1dPictureParameters { w, h, layout, bpc } = value;
        Self { w, h, layout, bpc }
    }
}

impl From<Rav1dPictureParameters> for Dav1dPictureParameters {
    fn from(value: Rav1dPictureParameters) -> Self {
        let Rav1dPictureParameters { w, h, layout, bpc } = value;
        Self { w, h, layout, bpc }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dPicture {
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub data: [*mut c_void; 3],
    pub stride: [ptrdiff_t; 2],
    pub p: Dav1dPictureParameters,
    pub m: Dav1dDataProps,
    pub content_light: *mut Dav1dContentLightLevel,
    pub mastering_display: *mut Dav1dMasteringDisplay,
    pub itut_t35: *mut Dav1dITUTT35,
    pub reserved: [uintptr_t; 4],
    pub frame_hdr_ref: *mut Dav1dRef,
    pub seq_hdr_ref: *mut Dav1dRef,
    pub content_light_ref: *mut Dav1dRef,
    pub mastering_display_ref: *mut Dav1dRef,
    pub itut_t35_ref: *mut Dav1dRef,
    pub reserved_ref: [uintptr_t; 4],
    pub r#ref: *mut Dav1dRef,
    pub allocator_data: *mut c_void,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dPicture {
    pub seq_hdr: *mut Dav1dSequenceHeader, // TODO(kkysen) make Rav1d
    pub frame_hdr: *mut Dav1dFrameHeader,  // TODO(kkysen) make Rav1d
    pub data: [*mut c_void; 3],
    pub stride: [ptrdiff_t; 2],
    pub p: Rav1dPictureParameters,
    pub m: Rav1dDataProps,
    pub content_light: *mut Dav1dContentLightLevel, // TODO(kkysen) make Rav1d
    pub mastering_display: *mut Dav1dMasteringDisplay, // TODO(kkysen) make Rav1d
    pub itut_t35: *mut Dav1dITUTT35,                // TODO(kkysen) make Rav1d
    pub reserved: [uintptr_t; 4],
    pub frame_hdr_ref: *mut Rav1dRef,
    pub seq_hdr_ref: *mut Rav1dRef,
    pub content_light_ref: *mut Rav1dRef,
    pub mastering_display_ref: *mut Rav1dRef,
    pub itut_t35_ref: *mut Rav1dRef,
    pub reserved_ref: [uintptr_t; 4],
    pub r#ref: *mut Rav1dRef,
    pub allocator_data: *mut c_void,
}

impl From<Dav1dPicture> for Rav1dPicture {
    fn from(value: Dav1dPicture) -> Self {
        let Dav1dPicture {
            seq_hdr,
            frame_hdr,
            data,
            stride,
            p,
            m,
            content_light,
            mastering_display,
            itut_t35,
            reserved,
            frame_hdr_ref,
            seq_hdr_ref,
            content_light_ref,
            mastering_display_ref,
            itut_t35_ref,
            reserved_ref,
            r#ref,
            allocator_data,
        } = value;
        Self {
            seq_hdr,
            frame_hdr,
            data,
            stride,
            p: p.into(),
            m: m.into(),
            content_light,
            mastering_display,
            itut_t35: itut_t35,
            reserved,
            frame_hdr_ref,
            seq_hdr_ref,
            content_light_ref,
            mastering_display_ref,
            itut_t35_ref,
            reserved_ref,
            r#ref,
            allocator_data,
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
            reserved,
            frame_hdr_ref,
            seq_hdr_ref,
            content_light_ref,
            mastering_display_ref,
            itut_t35_ref,
            reserved_ref,
            r#ref,
            allocator_data,
        } = value;
        Self {
            seq_hdr,
            frame_hdr,
            data,
            stride,
            p: p.into(),
            m: m.into(),
            content_light,
            mastering_display,
            itut_t35: itut_t35,
            reserved,
            frame_hdr_ref,
            seq_hdr_ref,
            content_light_ref,
            mastering_display_ref,
            itut_t35_ref,
            reserved_ref,
            r#ref,
            allocator_data,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dPicAllocator {
    pub cookie: *mut c_void,
    pub alloc_picture_callback:
        Option<unsafe extern "C" fn(*mut Dav1dPicture, *mut c_void) -> c_int>,
    pub release_picture_callback:
        Option<unsafe extern "C" fn(*mut Dav1dPicture, *mut c_void) -> ()>,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dPicAllocator {
    pub cookie: *mut c_void,
    pub alloc_picture_callback:
        Option<unsafe extern "C" fn(*mut Dav1dPicture, *mut c_void) -> c_int>,
    pub release_picture_callback:
        Option<unsafe extern "C" fn(*mut Dav1dPicture, *mut c_void) -> ()>,
}

impl From<Dav1dPicAllocator> for Rav1dPicAllocator {
    fn from(value: Dav1dPicAllocator) -> Self {
        let Dav1dPicAllocator {
            cookie,
            alloc_picture_callback,
            release_picture_callback,
        } = value;
        Self {
            cookie,
            alloc_picture_callback,
            release_picture_callback,
        }
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
            alloc_picture_callback,
            release_picture_callback,
        }
    }
}

impl Rav1dPicAllocator {
    pub unsafe fn alloc_picture(&mut self, p: *mut Rav1dPicture) -> c_int {
        let mut p_c = p.read().into();
        let result = self
            .alloc_picture_callback
            .expect("non-null function pointer")(&mut p_c, self.cookie);
        p.write(p_c.into());
        result
    }

    pub unsafe fn release_picture(&mut self, p: *mut Rav1dPicture) {
        let mut p_c = p.read().into();
        self.release_picture_callback
            .expect("non-null function pointer")(&mut p_c, self.cookie);
        p.write(p_c.into());
    }
}

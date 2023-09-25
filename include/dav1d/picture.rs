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

impl Dav1dPictureParameters {
    pub(crate) fn into_rust(self) -> Rav1dPictureParameters {
        let Self { w, h, layout, bpc } = self;
        Rav1dPictureParameters { w, h, layout, bpc }
    }
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dPictureParameters {
    pub w: c_int,
    pub h: c_int,
    pub layout: Dav1dPixelLayout,
    pub bpc: c_int,
}

impl Rav1dPictureParameters {
    pub fn into_c(self) -> Dav1dPictureParameters {
        let Self { w, h, layout, bpc } = self;
        Dav1dPictureParameters { w, h, layout, bpc }
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

impl Dav1dPicture {
    pub(crate) fn into_rust(self) -> Rav1dPicture {
        let Self {
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
        } = self;
        Rav1dPicture {
            seq_hdr,
            frame_hdr,
            data,
            stride,
            p: p.into_rust(),
            m: m.into_rust(),
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

impl Rav1dPicture {
    pub fn into_c(self) -> Dav1dPicture {
        let Self {
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
        } = self;
        Dav1dPicture {
            seq_hdr,
            frame_hdr,
            data,
            stride,
            p: p.into_c(),
            m: m.into_c(),
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

impl Dav1dPicAllocator {
    pub(crate) fn into_rust(self) -> Rav1dPicAllocator {
        let Self {
            cookie,
            alloc_picture_callback,
            release_picture_callback,
        } = self;
        Rav1dPicAllocator {
            cookie,
            alloc_picture_callback,
            release_picture_callback,
        }
    }
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

impl Rav1dPicAllocator {
    pub fn into_c(self) -> Dav1dPicAllocator {
        let Self {
            cookie,
            alloc_picture_callback,
            release_picture_callback,
        } = self;
        Dav1dPicAllocator {
            cookie,
            alloc_picture_callback,
            release_picture_callback,
        }
    }
}

impl Rav1dPicAllocator {
    pub unsafe fn alloc_picture(&mut self, p: *mut Rav1dPicture) -> c_int {
        let mut p_c = p.read().into_c();
        let result = self
            .alloc_picture_callback
            .expect("non-null function pointer")(&mut p_c, self.cookie);
        p.write(p_c.into_rust());
        result
    }

    pub unsafe fn release_picture(&mut self, p: *mut Rav1dPicture) {
        let mut p_c = p.read().into_c();
        self.release_picture_callback
            .expect("non-null function pointer")(&mut p_c, self.cookie);
        p.write(p_c.into_rust());
    }
}

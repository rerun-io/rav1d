#[inline]
pub fn ctz(mask: libc::c_uint) -> libc::c_int {
    return mask.trailing_zeros() as i32;
}

#[inline]
pub fn clz(mask: libc::c_uint) -> libc::c_int {
    return mask.leading_zeros() as i32;
}

#[inline]
pub fn clzll(mask: libc::c_ulonglong) -> libc::c_int {
    return mask.leading_zeros() as i32;
}

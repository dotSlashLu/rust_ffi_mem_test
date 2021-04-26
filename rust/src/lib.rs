#[no_mangle]
pub extern "C" fn a_string() -> *const std::os::raw::c_char {
    unsafe {
        let s = std::ffi::CStr::from_bytes_with_nul_unchecked(b"this string is from rust\0");
        s.as_ptr()
    }
}

// this one allocates a rust String first then makes a CStr from it
#[no_mangle]
pub extern "C" fn a_string2() -> *const std::os::raw::c_char {
    let s = String::from("this is a rust string");
    let s = std::ffi::CString::new(s).unwrap();
    let ptr = s.as_ptr();
    std::mem::forget(s);
    ptr
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

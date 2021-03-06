/// Changes chmod of path to 600.
pub fn set_mode_600<T: AsRef<str>>(path: T) -> Result<(), i32> {
    let path = std::ffi::CString::new(path.as_ref()).unwrap();

    let result = unsafe { libc::chmod(path.as_ptr(), 0o600) };

    if result >= 0 {
        Ok(())
    } else {
        Err(result)
    }
}

/// Changes chmod of path to 700.
pub fn set_mode_700<T: AsRef<str>>(path: T) -> Result<(), i32> {
    let path = std::ffi::CString::new(path.as_ref()).unwrap();

    let result = unsafe { libc::chmod(path.as_ptr(), 0o700) };

    if result >= 0 {
        Ok(())
    } else {
        Err(result)
    }
}

/// Gets the mode of a path.
pub fn get_mode<T: AsRef<str>>(path: T) -> Result<u16, i32> {
    let path = std::ffi::CString::new(path.as_ref()).unwrap();
    let mode_mask = 0o777;

    let (result, stat) = unsafe {
        let mut stat = std::mem::zeroed();
        let result = libc::stat(path.as_ptr(), &mut stat);
        (result, stat)
    };

    if result >= 0 {
        let mode = stat.st_mode & mode_mask;
        return Ok(mode as u16);
    } else {
        Err(result)
    }
}

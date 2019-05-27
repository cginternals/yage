#[cfg(not(target_arch = "wasm32"))]
pub fn gl_check_error(file: &str, line: u32) -> u32 {
    unsafe {
        let mut error_code = gl::GetError();
        while error_code != gl::NO_ERROR {
            let error = match error_code {
                gl::INVALID_ENUM => "INVALID_ENUM",
                gl::INVALID_VALUE => "INVALID_VALUE",
                gl::INVALID_OPERATION => "INVALID_OPERATION",
                gl::STACK_OVERFLOW => "STACK_OVERFLOW",
                gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
                gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
                gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
                _ => "unknown GL error code",
            };

            println!("{} | {} ({})", error, file, line);

            error_code = gl::GetError();
        }
        error_code
    }
}

#[cfg(target_arch = "wasm32")]
pub fn gl_check_error(_file: &str, _line: u32) -> u32 {
    // TODO!: implement
    0
}

#[macro_export]
macro_rules! check_error {
    () => {
        $crate::utils::gl_check_error(file!(), line!())
    };
}

macro_rules! propagate_error {
    ($fallible:expr) => {
        match $fallible {
            Ok(value) => value,
            Err(error) => return error.into_compile_error().into(),
        }
    };
}

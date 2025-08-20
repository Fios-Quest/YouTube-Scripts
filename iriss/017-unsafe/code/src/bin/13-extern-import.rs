struct SomeErrorType;

// SAFETY: The listed function signatures match those exposed in
// the some_external_library header files.
#[link(name = "some_external_library")]
unsafe extern "C" {
    fn some_function_exposed_by_the_library(x: i32) -> bool;
}

fn some_validator(test: bool) -> Result<bool, SomeErrorType> {
    // Do some validation of our data
    Ok(test)
}

fn safe_abstraction(x: i32) -> Result<bool, SomeErrorType> {
    // SAFETY: We validate data returned from the library
    unsafe {
        let answer = some_function_exposed_by_the_library(x);
        some_validator(answer)
    }
}

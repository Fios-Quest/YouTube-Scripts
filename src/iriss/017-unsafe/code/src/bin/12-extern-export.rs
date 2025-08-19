// SAFETY: No other global function will use this name, this does not need to
// be part of the documentation though.
#[unsafe(no_mangle)]
pub extern "C" fn exported_function(input: i32) -> bool {
    println!("Function called from another program with the value {input}");
    true
}

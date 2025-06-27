// macro_rules! <macro_name> {
//     // We then list each rule which are made up of function-like code blocks
//     // with brackets containing a match pattern, potentially including
//     // "metavariables".
//     //
//     // Each rule is matched based on the pattern and stores matching
//     // "metavariables" for use in the macro.  Don't worry, we''ll explain all
//     // of this very soon.
//     ( <match_pattern> ) => {
//     // curly braces surround the macro body. This is used to generate code
//     // at the invocation site of the macro.
//     };
//     // You can have more rules, but they need to have a different pattern of
//     // metavariables to match against.
//     ( <match_pattern> ) => {
//     // different rules can do completely different things, and can even
//     // call the macro again recursively
//     };
// }

macro_rules! hello {
    () => {
        String::from("Hello, world")
    };
}

fn main() {
    assert_eq!(hello!(), "Hello, world".to_string());
}

// macro_rules! <macro_name> {
//     ( <match_pattern> ) => {
//         <code goes here>
//     };

//     ( <match_pattern> ) => {
//         <code goes here>
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

#![recursion_limit = "2048"]

macro_rules! brain_fudge {
    ($($token:tt)+) => {{
        let mut memory = vec![0u8];
        let mut pointer = 0_usize;
        let mut buffer: Vec<u8> = Vec::new();

        brain_fudge_helper!(memory; pointer; buffer; $($token)+);

        buffer.into_iter().map(char::from).collect::<String>()
    }};
}

macro_rules! brain_fudge_helper {
    // +
    ($memory:ident; $pointer:ident; $buffer:ident; + $($tokens:tt)*) => {
        $memory[$pointer] = $memory[$pointer].wrapping_add(1);
        brain_fudge_helper!($memory; $pointer; $buffer; $($tokens)*);
    };
    // >
    ($memory:ident; $pointer:ident; $buffer:ident; > $($tokens:tt)*) => {
        $pointer = $pointer.saturating_add(1);
        while $pointer >= $memory.len() {
            $memory.push(0);
        }
        brain_fudge_helper!($memory; $pointer; $buffer; $($tokens)*);
    };
    // .
    ($memory:ident; $pointer:ident; $buffer:ident; . $($tokens:tt)*) => {
        $buffer.push($memory[$pointer]);
        brain_fudge_helper!($memory; $pointer; $buffer; $($tokens)*);
    };
    // NOP
    ($memory:ident; $pointer:ident; $buffer:ident; ) => {};
}

fn main() {
    assert_eq!(
        brain_fudge!(
            ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            >++++++++++++++++++++++++++++++++.
            >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            >+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            >++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
            >+++++++++++++++++++++++++++++++++.
            >++++++++++.
        ),
        "Hello World!\n"
    );
}

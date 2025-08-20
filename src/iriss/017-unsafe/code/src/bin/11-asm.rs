#[cfg(target_arch = "x86_64")]
fn multiply_by_six(mut input: u64) -> u64 {
    unsafe {
        std::arch::asm!(
            "mov {tmp}, {x}",
            "shl {tmp}, 1",
            "shl {x}, 2",
            "add {x}, {tmp}",
            x = inout(reg) input,
            tmp = out(reg) _,
        );
    }
    input
}

#[cfg(target_arch = "aarch64")]
fn multiply_by_six(mut input: u64) -> u64 {
    unsafe {
        std::arch::asm!(
        "mov {tmp}, {x}",
        "lsl {tmp}, {tmp}, #1",
        "lsl {x}, {x}, #2",
        "add {x}, {x}, {tmp}",
        x = inout(reg) input,
        tmp = out(reg) _,
        );
    }
    input
}

fn main() {
    let four_times_six = multiply_by_six(4);
    assert_eq!(four_times_six, 24);
    println!("4 * 6 is {}", four_times_six);
}

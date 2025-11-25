struct MonthStruct(u64);

#[repr(u64)]
enum MonthEnum {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

fn get_memory<T>(input: &T) -> &[u8] {
    // Credit: https://bennett.dev/rust/dump-struct-bytes/
    // SAFETY: Size of slice being read is the sizeof T which must be known (not ?Sized)
    unsafe { std::slice::from_raw_parts(input as *const _ as *const u8, size_of::<T>()) }
}

fn main() {
    let sept_num: u64 = 9;
    let sept_struct = MonthStruct(9);
    let sept_enum = MonthEnum::September;

    let num_bytes = get_memory(&sept_num);
    let struct_bytes = get_memory(&sept_struct);
    let enum_bytes = get_memory(&sept_enum);

    assert_eq!([9, 0, 0, 0, 0, 0, 0, 0], num_bytes);
    assert_eq!([9, 0, 0, 0, 0, 0, 0, 0], struct_bytes);
    assert_eq!([9, 0, 0, 0, 0, 0, 0, 0], enum_bytes);
}

enum ThisIsATaggedUnion {
    Number(u64),
    Character(char),
}

fn main() {
    let number = ThisIsATaggedUnion::Number(42);
    let character = ThisIsATaggedUnion::Character('c');

    assert_eq!(size_of_val(&number), size_of_val(&character));
    assert_ne!(size_of_val(&'c'), size_of_val(&character));

    println!("Size of character: {} bytes", size_of_val(&'c'));
    println!("Size of u64: {} bytes", size_of_val(&42_u64));

    let discriminant_size = size_of_val(&std::mem::discriminant(&number));
    println!("Size of enum discriminant: {} bytes", discriminant_size);

    println!("Size of enum number: {} bytes", size_of_val(&number));
    println!("Size of enum character: {} bytes", size_of_val(&character));
}

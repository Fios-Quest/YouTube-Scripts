use anyhow::Result;
use itertools::Itertools;
use std::thread::sleep;
use std::time::Duration;

const BINARY: &str = include_str!("../binary.txt");

fn main() -> Result<()> {
    let binary = BINARY.trim();

    let bytes: [u8; 8] = binary
        .chars()
        .collect::<Vec<_>>()
        .chunks(8)
        .map(|c| c.iter().collect::<String>())
        .map(|byte_str| u8::from_str_radix(&byte_str, 2).unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let bytes_str = bytes
        .iter()
        .map(|byte| format!("{byte:#8b}"))
        .intersperse(", ".to_string())
        .collect::<String>();

    let be_u64 = u64::from_be_bytes(bytes);
    let le_u64 = u64::from_le_bytes(bytes);
    let be_f64 = f64::from_be_bytes(bytes);
    let le_f64 = f64::from_le_bytes(bytes);

    let utf8 = str::from_utf8(&bytes)?;

    println!();
    println!("Binary: {binary}");
    sleep(Duration::from_secs(1));
    println!("Bytes:  {bytes_str}");
    sleep(Duration::from_secs(1));
    println!("Big Endian u64: {be_u64}");
    sleep(Duration::from_secs(1));
    println!("Little Endian u64: {le_u64}");
    sleep(Duration::from_secs(1));
    println!("Big Endian f64: {be_f64}");
    sleep(Duration::from_secs(1));
    println!("Little Endian f64: {le_f64}");
    sleep(Duration::from_secs(1));
    println!("UTF-8: {utf8}");
    println!();

    Ok(())
}

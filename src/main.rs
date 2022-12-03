use anyhow::{Context, Result};
use std::env;

fn main() -> Result<()> {
    let input = env::args()
        .skip(1)
        .next()
        .with_context(|| format!("please supply a nunber"))?;

    let number = if input.starts_with("0x") {
        i64::from_str_radix(input.trim_start_matches("0x"), 16)? as u32
    } else if input.starts_with("0b") {
        i64::from_str_radix(input.trim_start_matches("0b"), 2)? as u32
    } else {
        i64::from_str_radix(&input, 10)? as u32
    };
    
    if (number as i32) < 0 {
        println!("DECIMAL: {} (signed), {} (unsigned)", number as i32, number);
    } else {
        println!("DECIMAL: {}", number as i32);
    }
    println!(
        "    HEX: {:02x}_{:02x}_{:02x}_{:02x}",
        number / 0x1_000_000,
        number / 0x10_000 % 0x100,
        number / 0x100 % 0x100,
        number % 0x100
    );
    println!(
        " BINARY: {:08b}_{:08b}_{:08b}_{:08b}",
        number / 0x1_000_000,
        number / 0x10_000 % 0x100,
        number / 0x100 % 0x100,
        number % 0x100
    );

    Ok(())
}

use anyhow::Result;
use clap::Parser;

/// Convert between decimal, hex and binary in 32-bit (default) or 64-bit.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, allow_hyphen_values(true))]
struct Args {
    /// As signed decimal (eg. '42', '-69') or unsigned hex or binary (eg. '0xdeadbeef', '0b110110110)
    number: String,

    /// Use 64-bit
    #[arg(short, default_value_t = false)]
    x: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let (stripped, base) = if args.number.starts_with("0x") {
        (args.number.trim_start_matches("0x"), 16)
    } else if args.number.starts_with("0b") {
        (args.number.trim_start_matches("0b"), 2)
    } else {
        (&*args.number, 10)
    };

    let negative = stripped.starts_with("-") && base == 10;
    let n = if args.x {
        if negative {
            i64::from_str_radix(stripped, base)? as u64
        } else {
            u64::from_str_radix(stripped, base)?
        }
    } else {
        if negative {
            i32::from_str_radix(stripped, base)? as u64
        } else {
            u32::from_str_radix(stripped, base)? as u64
        }
    };

    if (n as i64) < 0 {
        if args.x {
            println!("DECIMAL: {} (signed), {} (unsigned)", n as i64, n);
        } else {
            println!("DECIMAL: {} (signed), {} (unsigned)", n as i32, n as u32);
        }
    } else {
        println!("DECIMAL: {}", n);
    }

    let bytes = n.to_le_bytes();
    let mut hex = String::from("    HEX: ");
    let mut bin = String::from(" BINARY: ");

    for byte in bytes.iter().take(if args.x { 8 } else { 4 }).rev() {
        hex.push_str(&format!("{:02x}_", byte));
        bin.push_str(&format!("{:08b}_", byte));
    }

    println!("{}", &hex[..hex.len() - 1]);
    println!("{}", &bin[..bin.len() - 1]);
    Ok(())
}

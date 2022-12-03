# What The Number
A simple command-line tool to get equivalent representations of a number (decimal, hex and binary). Assumes a 32-bit architecture.

# Installation
In the root directory run:
```bash
cargo install --path .
```

# Usage
```bash
wtn <number>
```

### Examples
```bash
# Decimal
wtn -123

# Hex
wtn 0xdeadbeef

# Binary
wtn 0b100010101110
```

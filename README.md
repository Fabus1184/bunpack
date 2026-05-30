[![Rust](https://github.com/Fabus1184/bunpack/actions/workflows/rust.yml/badge.svg)](https://github.com/Fabus1184/bunpack/actions/workflows/rust.yml)
![Crates.io Version](https://img.shields.io/crates/v/bunpack)


# bunpack 

A library for packing and unpacking binary data in Rust using format strings similar to Python's `struct` module.

## Usage

```rust
use bunpack::{pack, unpack};
let data: Vec<u8> = pack!("i", 1234);
let value: i32 = unpack!("i", &data);
assert_eq!(value, 1234);
```

# Format Specifiers

## Byte Order

Format specifiers can begin with an optional character to specify the byte order:
| Specifier | Description       |
| --------- | ----------------- |
| `@`       | Native byte order |
| `<`       | Little-endian     |
| `>`       | Big-endian        |

## Type Specifiers

Following the optional byte order, format specifiers can include one or more type specifiers:

| Specifier | Rust Type   | Size in bytes              |
| --------- | ----------- | -------------------------- |
| `c`       | `char`      | 4                          |
| `b`       | `i8`        | 1                          |
| `B`       | `u8`        | 1                          |
| `?`       | `bool`      | 1                          |
| `h`       | `i16`       | 2                          |
| `H`       | `u16`       | 2                          |
| `i`       | `i32`       | 4                          |
| `I`       | `u32`       | 4                          |
| `q`       | `i64`       | 8                          |
| `Q`       | `u64`       | 8                          |
| `o`       | `i128`      | 16                         |
| `O`       | `u128`      | 16                         |
| `n`       | `isize`     | native                     |
| `N`       | `usize`     | native                     |
| `f`       | `f32`       | 4                          |
| `d`       | `f64`       | 8                          |
| `s`       | `&str`      | pack: length of string     |
| `p`       | `&[u8]`     | pack: length of byte array |
| `P`       | `*const ()` | native                     |

## Repeat Counts

Type specifiers can be chained together to create tuples of values. For example, `iHf` specifies a `(i32, u16, f32)` tuple.

Type specifiers can be wrapped in Rust-style arrays to indicate repeat counts.
Example: `[i; 3]` specifies a `[i32; 3]` array, and `[i; 2][H; 4]` specifies `([i32; 2], [u16; 4])`.

```rust
let values = [(1, 1.0, '🐈'), (2, 2.0, '🐕'), (3, 3.0, '🦅')];

let bytes = bunpack::pack!("<[ifc; 3]", values);
let unpacked: [(i32, f32, char); 3] = bunpack::unpack!("<[ifc; 3]", &bytes);
assert_eq!(unpacked, values);
```

## More examples

```rust
let str = "Hello, World!";

let bytes = bunpack::pack!("<p", str.as_bytes());
let unpacked: [u8; 13] = bunpack::unpack!("[B;13]", &bytes);
assert_eq!(unpacked, *str.as_bytes());
```

---

```rust
let str = "Hello, World!";

let bytes = bunpack::pack!("<p", str.as_bytes());
let unpacked: [u8; 13] = bunpack::unpack!("[B;13]", &bytes);
assert_eq!(unpacked, *str.as_bytes());
```

---


```rust
let bytes = bunpack::pack!(">H", 0x1234);
let unpacked: u16 = bunpack::unpack!("<H", &bytes);
assert_eq!(unpacked, 0x3412);
```

//! A library for packing and unpacking binary data in Rust using format strings similar to Python's `struct` module.
//! # Examples
//!
//! ```
//! use bunpack::{pack, unpack};
//!
//! let data: Vec<u8> = pack!("i", 1234);
//! let value: i32 = unpack!("i", &data);
//! assert_eq!(value, 1234);
//! ```
//! # Format Specifiers
//!
//! ## Byte Order
//!
//! Format specifiers can begin with an optional character to specify the byte order:
//!
//! | Specifier | Description |
//! | - | - |
//! | `@` | Native byte order |
//! | `<` | Little-endian |
//! | `>` | Big-endian |
//!
//! ## Type Specifiers
//!
//! Following the optional byte order, format specifiers can include one or more type specifiers:
//!
//! | Specifier | Rust Type | Size in bytes |
//! | - | - | - |
//! | `c` | `char`           | 4                    |
//! | `b` | `i8`             | 1                    |
//! | `B` | `u8`             | 1                    |
//! | `?` | `bool`           | 1                    |
//! | `h` | `i16`            | 2                    |
//! | `H` | `u16`            | 2                    |
//! | `i` | `i32`            | 4                    |
//! | `I` | `u32`            | 4                    |
//! | `q` | `i64`            | 8                    |
//! | `Q` | `u64`            | 8                    |
//! | `o` | `i128`           | 16                   |
//! | `O` | `u128`           | 16                   |
//! | `n` | `isize`          | native               |
//! | `N` | `usize`          | native               |
//! | `f` | `f32`            | 4                    |
//! | `d` | `f64`            | 8                    |
//! | `s` | `&str`           | pack only: length of string |
//! | `p` | `&[u8]`          | pack only: length of byte array |
//! | `P` | `*const ()` | native               |

pub use bunpack_proc_macros::{pack, unpack};

pub trait Unpack {
    fn unpack_le(data: &mut &[u8]) -> Self;
    fn unpack_be(data: &mut &[u8]) -> Self;
}

pub trait Pack {
    fn pack_le(self, buf: &mut Vec<u8>);
    fn pack_be(self, buf: &mut Vec<u8>);
}

macro_rules! impl_packs {
    ($ty:ty) => {
        impl Unpack for $ty {
            fn unpack_le(data: &mut &[u8]) -> Self {
                const SIZE: usize = std::mem::size_of::<$ty>();
                let bytes: [u8; SIZE] =
                    data[..SIZE].try_into().expect("Not enough bytes to unpack");
                *data = &data[SIZE..];
                Self::from_le_bytes(bytes)
            }

            fn unpack_be(data: &mut &[u8]) -> Self {
                const SIZE: usize = std::mem::size_of::<$ty>();
                let bytes: [u8; SIZE] =
                    data[..SIZE].try_into().expect("Not enough bytes to unpack");
                *data = &data[SIZE..];
                Self::from_be_bytes(bytes)
            }
        }

        impl Pack for $ty {
            fn pack_le(self, buf: &mut Vec<u8>) {
                buf.extend_from_slice(&self.to_le_bytes());
            }

            fn pack_be(self, buf: &mut Vec<u8>) {
                buf.extend_from_slice(&self.to_be_bytes());
            }
        }
    };
}

impl_packs!(u8);
impl_packs!(i8);
impl_packs!(u16);
impl_packs!(i16);
impl_packs!(u32);
impl_packs!(i32);
impl_packs!(u64);
impl_packs!(i64);
impl_packs!(u128);
impl_packs!(i128);
impl_packs!(isize);
impl_packs!(usize);
impl_packs!(f32);
impl_packs!(f64);

impl Unpack for bool {
    fn unpack_le(data: &mut &[u8]) -> Self {
        let byte = u8::unpack_le(data);
        bool::try_from(byte).expect("Invalid boolean encoding")
    }

    fn unpack_be(data: &mut &[u8]) -> Self {
        let byte = u8::unpack_be(data);
        bool::try_from(byte).expect("Invalid boolean encoding")
    }
}
impl Pack for bool {
    fn pack_le(self, buf: &mut Vec<u8>) {
        (self as u8).pack_le(buf);
    }
    fn pack_be(self, buf: &mut Vec<u8>) {
        (self as u8).pack_be(buf);
    }
}

impl Unpack for char {
    fn unpack_le(data: &mut &[u8]) -> Self {
        char::from_u32(u32::unpack_le(data)).expect("Invalid char encoding")
    }

    fn unpack_be(data: &mut &[u8]) -> Self {
        char::from_u32(u32::unpack_be(data)).expect("Invalid char encoding")
    }
}
impl Pack for char {
    fn pack_le(self, buf: &mut Vec<u8>) {
        (self as u32).pack_le(buf);
    }

    fn pack_be(self, buf: &mut Vec<u8>) {
        (self as u32).pack_be(buf);
    }
}

impl<T> Pack for *const T {
    fn pack_le(self, buf: &mut Vec<u8>) {
        (self as usize).pack_le(buf);
    }

    fn pack_be(self, buf: &mut Vec<u8>) {
        (self as usize).pack_be(buf);
    }
}
impl<T> Unpack for *const T {
    fn unpack_le(data: &mut &[u8]) -> Self {
        usize::unpack_le(data) as *const T
    }

    fn unpack_be(data: &mut &[u8]) -> Self {
        usize::unpack_be(data) as *const T
    }
}

impl Pack for &str {
    fn pack_le(self, buf: &mut Vec<u8>) {
        self.as_bytes().pack_le(buf);
    }

    fn pack_be(self, buf: &mut Vec<u8>) {
        self.as_bytes().pack_be(buf);
    }
}

impl Pack for &[u8] {
    fn pack_le(self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self);
    }

    fn pack_be(self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self);
    }
}

impl<T: Pack, const N: usize> Pack for [T; N] {
    fn pack_le(self, buf: &mut Vec<u8>) {
        for item in self {
            item.pack_le(buf);
        }
    }

    fn pack_be(self, buf: &mut Vec<u8>) {
        for item in self {
            item.pack_be(buf);
        }
    }
}
impl<T: Unpack, const N: usize> Unpack for [T; N] {
    fn unpack_le(data: &mut &[u8]) -> Self {
        std::array::from_fn(|_| T::unpack_le(data))
    }

    fn unpack_be(data: &mut &[u8]) -> Self {
        std::array::from_fn(|_| T::unpack_be(data))
    }
}

macro_rules! impl_packs_tuple {
    ($($name:ident),*) => {
        impl<$($name: Unpack),*> Unpack for ($($name,)*) {
            fn unpack_le(data: &mut &[u8]) -> Self {
                ($($name::unpack_le(data),)*)
            }

            fn unpack_be(data: &mut &[u8]) -> Self {
                ($($name::unpack_be(data),)*)
            }
        }

        impl<$($name: Pack),*> Pack for ($($name,)*) {
            fn pack_le(self, buf: &mut Vec<u8>) {
                let ($($name,)*) = self;
                $(
                    $name.pack_le(buf);
                )*
            }

            fn pack_be(self, buf: &mut Vec<u8>) {
                let ($($name,)*) = self;
                $(
                    $name.pack_be(buf);
                )*
            }
        }
    };
}

impl_packs_tuple!(A);
impl_packs_tuple!(A, B);
impl_packs_tuple!(A, B, C);
impl_packs_tuple!(A, B, C, D);
impl_packs_tuple!(A, B, C, D, E);
impl_packs_tuple!(A, B, C, D, E, F);
impl_packs_tuple!(A, B, C, D, E, F, G);
impl_packs_tuple!(A, B, C, D, E, F, G, H);
impl_packs_tuple!(A, B, C, D, E, F, G, H, I);
impl_packs_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_packs_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_packs_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_packs_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_packs_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_packs_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_packs_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);

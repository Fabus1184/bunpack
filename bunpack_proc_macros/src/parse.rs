peg::parser! {
    pub grammar fmt_parser() for str {
        rule _() = [c if c.is_ascii_whitespace()]*

        pub rule fmt_pack() -> (bool, Vec<syn::Type>)
            = le:endianness_is_little() _ types:ty_pack() _ { (le, types) }

        pub rule fmt_unpack() -> (bool, Vec<syn::Type>)
            = le:endianness_is_little() _ types:ty_unpack() _ { (le, types) }

        rule endianness_is_little() -> bool
            = "<" { true }
            / ">" { false }
            / "@" {
                #[cfg(target_endian = "little")] { true }
                #[cfg(target_endian = "big")] { false }
            }
            / { true }

        rule ty_pack() -> Vec<syn::Type>
            = tys:ty_pack_()+ { tys }
        rule ty_unpack() -> Vec<syn::Type>
            = tys:ty_unpack_()+ { tys }

        rule ty_pack_() -> syn::Type
            = ty_both()
            / _ "s" { syn::parse_quote!(&str) }
            / _ "p" { syn::parse_quote!(&[u8]) }
            / _ "[" _ ty:ty_pack() _ ";" _ n:usize() _ "]" { syn::parse_quote!( [(#(#ty),*); #n] ) }

        rule ty_unpack_() -> syn::Type
            = ty_both()
            / _ "[" _ ty:ty_unpack() _ ";" _ n:usize() _ "]" { syn::parse_quote!( [(#(#ty),*); #n] ) }

        rule ty_both() -> syn::Type
            = _ "c" { syn::parse_quote!(char) }
            / _ "b" { syn::parse_quote!(i8) }
            / _ "B" { syn::parse_quote!(u8) }
            / _ "?" { syn::parse_quote!(bool) }
            / _ "h" { syn::parse_quote!(i16) }
            / _ "H" { syn::parse_quote!(u16) }
            / _ "i" { syn::parse_quote!(i32) }
            / _ "I" { syn::parse_quote!(u32) }
            / _ "q" { syn::parse_quote!(i64) }
            / _ "Q" { syn::parse_quote!(u64) }
            / _ "o" { syn::parse_quote!(i128) }
            / _ "O" { syn::parse_quote!(u128) }
            / _ "n" { syn::parse_quote!(isize) }
            / _ "N" { syn::parse_quote!(usize) }
            / _ "f" { syn::parse_quote!(f32) }
            / _ "d" { syn::parse_quote!(f64) }
            / _ "P" { syn::parse_quote!(*const ()) }

        rule usize() -> usize
            = n:$(['0'..='9']+) { n.parse().unwrap() }
    }
}

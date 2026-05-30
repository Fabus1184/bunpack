mod parse;

struct UnpackArgs {
    fmt: String,
    data: syn::Expr,
}

impl syn::parse::Parse for UnpackArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fmt = input.parse::<syn::LitStr>()?.value();
        _ = input.parse::<syn::Token![,]>()?;
        let data = input.parse()?;

        Ok(Self { fmt, data })
    }
}

struct UnpackReadArgs {
    reader: syn::Expr,
    fmt: String,
}

impl syn::parse::Parse for UnpackReadArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let reader = input.parse()?;
        _ = input.parse::<syn::Token![,]>()?;
        let fmt = input.parse::<syn::LitStr>()?.value();

        Ok(Self { reader, fmt })
    }
}

struct PackArgs {
    fmt: String,
    args: Vec<syn::Expr>,
}

impl syn::parse::Parse for PackArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fmt = input.parse::<syn::LitStr>()?.value();
        _ = input.parse::<syn::Token![,]>()?;
        let args = input
            .parse_terminated(syn::Expr::parse, syn::Token![,])?
            .into_iter()
            .collect();
        Ok(Self { fmt, args })
    }
}

struct PackWriteArgs {
    writer: syn::Expr,
    fmt: String,
    args: Vec<syn::Expr>,
}

impl syn::parse::Parse for PackWriteArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let writer = input.parse()?;
        _ = input.parse::<syn::Token![,]>()?;
        let fmt = input.parse::<syn::LitStr>()?.value();
        _ = input.parse::<syn::Token![,]>()?;
        let args = input
            .parse_terminated(syn::Expr::parse, syn::Token![,])?
            .into_iter()
            .collect();
        Ok(Self { writer, fmt, args })
    }
}

#[proc_macro]
pub fn pack(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let PackArgs { fmt, args } = syn::parse_macro_input!(input as PackArgs);

    let (little_endian, types) =
        parse::fmt_parser::fmt_pack(&fmt).unwrap_or_else(|e| panic!("Invalid format string: {e}"));

    if types.len() != args.len() {
        panic!(
            "Number of format specifiers ({}) does not match number of arguments ({})",
            types.len(),
            args.len()
        );
    }

    let arg_names = (0..args.len())
        .map(|i| syn::Ident::new(&format!("arg{i}"), proc_macro2::Span::call_site()))
        .collect::<Vec<_>>();
    let fn_args = arg_names
        .iter()
        .zip(types.iter())
        .map(|(arg, ty)| quote::quote! { #arg: #ty })
        .collect::<Vec<_>>();

    quote::quote! {{
        fn pack( #( #fn_args ),* ) -> Vec<u8>
        {
            let mut buf = Vec::new();
            #(
                if #little_endian {
                    <#types as ::bunpack::Pack>::pack_le(#arg_names, &mut buf);
                } else {
                    <#types as ::bunpack::Pack>::pack_be(#arg_names, &mut buf);
                }
            )*
            buf
        }

        pack( #( #args ),* )
    }}
    .into()
}

#[proc_macro]
pub fn pack_write(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let PackWriteArgs { writer, fmt, args } = syn::parse_macro_input!(input as PackWriteArgs);

    let (little_endian, types) =
        parse::fmt_parser::fmt_pack(&fmt).unwrap_or_else(|e| panic!("Invalid format string: {e}"));

    if types.len() != args.len() {
        panic!(
            "Number of format specifiers ({}) does not match number of arguments ({})",
            types.len(),
            args.len()
        );
    }

    let arg_names = (0..args.len())
        .map(|i| syn::Ident::new(&format!("arg{i}"), proc_macro2::Span::call_site()))
        .collect::<Vec<_>>();
    let fn_args = arg_names
        .iter()
        .zip(types.iter())
        .map(|(arg, ty)| quote::quote! { #arg: #ty })
        .collect::<Vec<_>>();

    quote::quote! {{
        fn pack_write<W: std::io::Write>(writer: &mut W, #( #fn_args ),* ) -> std::io::Result<()>
        {
            let mut buf = Vec::new();
            #(
                if #little_endian {
                    <#types as ::bunpack::Pack>::pack_le(#arg_names, &mut buf);
                } else {
                    <#types as ::bunpack::Pack>::pack_be(#arg_names, &mut buf);
                }
            )*

            writer.write_all(&buf)
        }

        pack_write(#writer, #( #args ),* )
    }}
    .into()
}

#[proc_macro]
pub fn unpack(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let UnpackArgs { fmt, data } = syn::parse_macro_input!(input as UnpackArgs);

    let (little_endian, types) = parse::fmt_parser::fmt_unpack(&fmt)
        .unwrap_or_else(|e| panic!("Invalid format string: {e}"));

    quote::quote! {{
        fn unpack<T: AsRef<[u8]>>(data: T) -> ( #( #types ),* )
        {
            let mut data: &[u8] = data.as_ref();
            (#(
                if #little_endian {
                    <#types as ::bunpack::Unpack>::unpack_le(&mut data)
                } else {
                    <#types as ::bunpack::Unpack>::unpack_be(&mut data)
                }
            ),*)
        }

        unpack(#data)
    }}
    .into()
}

#[proc_macro]
pub fn unpack_read(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let UnpackReadArgs { reader, fmt } = syn::parse_macro_input!(input as UnpackReadArgs);

    let (little_endian, types) = parse::fmt_parser::fmt_unpack(&fmt)
        .unwrap_or_else(|e| panic!("Invalid format string: {e}"));

    quote::quote! {{
        fn unpack_read<R: std::io::Read>(reader: &mut R) -> std::io::Result<( #( #types ),* )>
        {
            Ok((#({
                let mut buf = [0u8; <#types as ::bunpack::Unpack>::SIZE];
                reader.read_exact(&mut buf)?;
                let value = if #little_endian {
                    <#types as ::bunpack::Unpack>::unpack_le(&mut &buf[..])
                } else {
                    <#types as ::bunpack::Unpack>::unpack_be(&mut &buf[..])
                };
                value
            }),*))
        }

        unpack_read(&mut #reader)
    }}
    .into()
}

use proc_macro::{self};
use proc_macro2::{Punct, Spacing::Alone, TokenStream};
use quote::{ToTokens, quote};
use syn::{Expr, LitInt, LitStr, Token, Type, parse, parse::Parse, parse_macro_input};

struct NestedType {
    dim_expr: Expr,
    inner_type: Type,
    outer_type: Type,
    open_delim: char,
    close_delim: char,
}

//TODO: maybe use not string for delimiters
impl Parse for NestedType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let number = input.parse::<Expr>()?;
        let _ = input.parse::<Token![,]>()?;
        let outer_type = input.parse()?;
        let _ = input.parse::<Token![,]>()?;
        let inner_type = input.parse()?;
        let _ = input.parse::<Token![,]>()?;
        let delimiters = input.parse::<LitStr>()?;
        let delimiters = delimiters.value();
        let delimiters = delimiters.trim();
        if delimiters.len() != 2 {
            panic!("Delimiters have to be 2 long");
        }
        let chars: Vec<char> = delimiters.chars().collect();

        Ok(Self {
            dim_expr: number,
            inner_type,
            outer_type,
            open_delim: chars[0],
            close_delim: chars[1],
        })
    }
}

impl ToTokens for NestedType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let dim_expr_expl = &self.dim_expr;

        let dim_val_bind = quote! {#dim_expr_expl};

        let dim_val;
        match parse::<LitInt>(dim_val_bind.clone().into()) {
            Ok(val_lit) => match val_lit.base10_parse() {
                Ok(val) => dim_val = val,
                Err(e) => panic!(
                    "Dimension expression didn't evaluate correctly, was: {val_lit}, got error: {e}"
                ),
            },
            Err(e) => {
                panic!(
                    "Dimension expression has to evaluate to integer, but was {dim_val_bind}, got error: {e}"
                );
            }
        }

        for _ in 0..dim_val {
            self.outer_type.to_tokens(tokens);
            Punct::new(self.open_delim, Alone).to_tokens(tokens);
        }
        self.inner_type.to_tokens(tokens);
        TokenStream::from_iter(::std::iter::repeat_n(
            Punct::new(self.close_delim, Alone).into_token_stream(),
            dim_val,
        ))
        .to_tokens(tokens);
    }
}

/// macro transforms type into dim-times nested type
/// signature:
/// ```ignore
/// pub fn nested_type(dim: Expr, outer_type: Type, inner_type: Type, delim: &str)
/// ```
/// delim has to be string of length 2, containing opening and closing delimiter (eg.: "<>", "[]", "{}" etc.)
///
/// At the moment non-explicit expressions like:
/// ```ignore
/// let test = 2;
/// let vec: nested_type!(test,Vec,u32,"<>")
/// ```
/// are not possible in rust, even with test declared as constant :(
///
/// # Examples
///
/// ```ignore
/// # use rep_proc_macro_test::nested_type;
/// type nested = Vec<Vec<u32>>;
/// type bare = Vec<u32>;
/// assert_eq!(nested, nested_type!(2,Vec,u32,"<>"));
/// ```
#[proc_macro]
pub fn nested_type(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as NestedType);
    quote! {#input}.into()
}

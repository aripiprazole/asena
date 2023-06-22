use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::*;

#[allow(clippy::redundant_clone)]
pub fn expand_ast_leaf(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);
    let name = input.sig.ident.clone();
    let cursor_name = Ident::new(&format!("find_{name}"), Span::call_site());
    let output = match input.sig.output {
        ReturnType::Default => quote!(()),
        ReturnType::Type(_, ty) => quote!(#ty),
    };

    input.sig.output = parse(quote!(-> asena_leaf::ast::Cursor<#output>).into()).unwrap();

    let mut impl_fn_tokens = input.clone();
    impl_fn_tokens.sig.ident = Ident::new(&format!("_impl_{name}"), Span::call_site());

    let impl_name = impl_fn_tokens.sig.ident.clone();

    let mut get_fn_tokens = input.clone();
    get_fn_tokens.sig.ident = Ident::new(&format!("{name}"), Span::call_site());
    get_fn_tokens.sig.output = parse(quote!(-> #output).into()).unwrap();
    get_fn_tokens.block =
        Box::new(parse(quote! {{ self.#cursor_name().as_leaf() }}.into()).unwrap());

    let mut set_fn_tokens = input.clone();
    set_fn_tokens.sig.output = parse(quote!(-> ()).into()).unwrap();
    set_fn_tokens
        .sig
        .inputs
        .push(parse(quote!(value: #output).into()).unwrap());
    set_fn_tokens.sig.ident = Ident::new(&format!("set_{name}"), Span::call_site());
    set_fn_tokens.block = Box::new(
        parse(
            quote! {{
                self.#cursor_name().set(asena_leaf::ast::Cursor::of(value))
            }}
            .into(),
        )
        .unwrap(),
    );

    let mut find_fn_tokens = input.clone();
    let key_name = name.to_string();
    find_fn_tokens.sig.ident = cursor_name;
    find_fn_tokens.block = Box::new(
        parse(
            quote! {{
               self.memoize(#key_name, |this| {
                   let this = Self::new(this.clone());
                   this.#impl_name()
               })
            }}
            .into(),
        )
        .unwrap(),
    );

    TokenStream::from(quote! {
        #impl_fn_tokens
        #get_fn_tokens
        #set_fn_tokens
        #find_fn_tokens
    })
}

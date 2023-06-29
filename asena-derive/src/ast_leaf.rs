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

    input.sig.output = parse_quote!(-> asena_leaf::ast::Cursor<#output>);

    let mut impl_fn_tokens = input.clone();
    impl_fn_tokens.attrs.push(parse_quote! {
        #[doc(hidden)]
    });
    impl_fn_tokens.sig.ident = Ident::new(&format!("_impl_{name}"), Span::call_site());

    let impl_name = impl_fn_tokens.sig.ident.clone();

    let mut get_fn_tokens = input.clone();
    get_fn_tokens.sig.ident = Ident::new(&format!("{name}"), Span::call_site());
    get_fn_tokens.sig.output = parse_quote!(-> #output);
    get_fn_tokens.block = Box::new(parse_quote! {{ self.#cursor_name().as_leaf() }});

    let mut set_fn_tokens = input.clone();
    set_fn_tokens.sig.output = parse_quote!(-> ());
    set_fn_tokens
        .sig
        .inputs
        .push(parse(quote!(value: impl Into<#output>).into()).unwrap());
    set_fn_tokens.sig.ident = Ident::new(&format!("set_{name}"), Span::call_site());
    set_fn_tokens.block = Box::new(parse_quote! {{
        self.insert(stringify!(#name), value.into())
    }});

    let mut find_fn_tokens = input.clone();
    let key_name = name.to_string();
    find_fn_tokens.sig.ident = cursor_name;
    find_fn_tokens.block = Box::new(parse_quote! {{
       self.memoize(#key_name, |this| {
           use asena_leaf::ast::Node;
           let this = Self::new(this.clone());
           this.#impl_name()
       })
    }});

    TokenStream::from(quote! {
        #impl_fn_tokens
        #get_fn_tokens
        #set_fn_tokens
        #find_fn_tokens
    })
}

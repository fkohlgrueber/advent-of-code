extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;


#[proc_macro]
pub fn parse_multiple(input: TokenStream) -> TokenStream {
    input
}

enum Ty {
    Simple(String),
    Vec(String)
}

impl Ty {
    pub fn inner_ty(&self) -> &str {
        match self {
            Ty::Simple(s) => &s,
            Ty::Vec(s) => &s,
        }
    }
}

#[proc_macro_attribute]
pub fn parse(attr: TokenStream, item: TokenStream) -> TokenStream {

    // parse item as a syn struct
    let mut strukt = syn::parse_macro_input!(item as syn::ItemStruct);
    let name = &strukt.ident;
    
    
    // extract name and type for each struct element
    let mut items = vec!();
    let fields = match &mut strukt.fields {
        syn::Fields::Named(nf) => {
            &mut nf.named
        }
        syn::Fields::Unnamed(uf) => {
            &mut uf.unnamed
        }
        syn::Fields::Unit => panic!("Unit structs aren't supported!")
    };
    for field in fields {
        let mut custom_pattern = None;
        if !field.attrs.is_empty() && &field.attrs[0].path.to_token_stream().to_string() == "parse" {
            let meta = field.attrs[0].parse_meta().unwrap();
            if let syn::Meta::NameValue(syn::MetaNameValue { lit: syn::Lit::Str(lit_str), .. }) = meta {
                custom_pattern = Some(lit_str.value());
                field.attrs.remove(0);
            }
        }
        let ty_str = field.ty.to_token_stream().to_string();
        let ty = if ty_str.starts_with("Vec < ") {
            Ty::Vec(ty_str[6..ty_str.len()-2].to_string())
        } else {
            Ty::Simple(ty_str)
        };
        items.push(
            (field.ident.as_ref(), ty, custom_pattern)
        )
    }

    // parse groups from input. A group is delimited by curly braces
    let pattern: String = syn::parse_macro_input!(attr as syn::LitStr).value();
    let parts = pattern.split("{}").collect::<Vec<_>>();

    // panic if number of brace pairs in pattern doesn't match number of struct elements
    assert!(parts.len() == items.len()+1);

    // generate the regex pattern string
    let mut regex_str = String::new();
    for (pattern_prefix, item) in parts.iter().zip(items.iter()) {
        let regex_for_type = match &item.2 {
            Some(s) => s.clone(),
            None => get_regex_for_type(&item.1.inner_ty()).to_string(),
        };
        regex_str.push_str(&format!("{}({})", pattern_prefix, regex_for_type));
    }
    regex_str.push_str(parts.last().unwrap());

    // generate initializers for each struct element
    let mut inits = vec!();
    for (idx, item) in items.iter().enumerate() {
        let name = match &item.0 {
            Some(n) => quote!( #n: ),
            None => quote!()
        };
        let ts = match &item.1 {
            Ty::Simple(s) => quote!( #name cap[#idx+1].parse().unwrap()),
            Ty::Vec(s) => {
                let ty = syn::Ident::new(s, proc_macro2::Span::call_site());
                quote!( #name #ty::from_str_multiple(&cap[#idx+1]))
            },
        };
        inits.push(ts);
    }

    let initializer = match &strukt.fields {
        syn::Fields::Named(_) => quote!( Self { #(#inits),* } ),
        syn::Fields::Unnamed(_) => quote!( Self ( #(#inits),* ) ),
        _ => panic!()
    };

    quote!(
        #strukt

        impl #name {
            fn get_regex() -> &'static aoc_tools::__imp::Regex {
                use aoc_tools::__imp::Regex;
                aoc_tools::__imp::lazy_static! {
                    static ref RE: Regex = Regex::new(#regex_str).unwrap();
                }
                &RE
            }

            pub fn from_str_multiple(s: &str) -> Vec<Self> {
                Self::get_regex().captures_iter(s).map(|cap| #initializer ).collect()
            }
        }

        impl std::str::FromStr for #name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match Self::get_regex().captures(s){
                    Some(cap) => Ok(#initializer),
                    None => Err("Regex didn't match".to_string()),
                }
            }
        }
    ).into()
}


fn get_regex_for_type(ty: &str) -> &'static str {
    match ty {
        "usize" => r"\d+",
        "i32" => r"[-+]?\d+",
        "f64" => r"[0-9]*\.?[0-9]*",
        "char" => r".",
        "bool" => r"true|false",
        t => panic!(format!("Regex for type '{}' is unknown", t))
    }
}
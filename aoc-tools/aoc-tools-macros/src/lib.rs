extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;



#[derive(Debug)]
struct ParseInput(Vec<syn::ItemStruct>);

impl syn::parse::Parse for ParseInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut elmts = vec!();
        while !input.is_empty() {
            elmts.push(input.parse()?)
        }
        Ok(Self(elmts))
    }
}

#[proc_macro]
pub fn parse_multiple(input: TokenStream) -> TokenStream {
    let items = syn::parse_macro_input!(input as ParseInput);

    let mut regexes = std::collections::HashMap::new();
    let mut out_tokens = vec!();

    for mut item in items.0.into_iter() {
        let pattern = get_pattern_str(&item);
        item.attrs.remove(0);
        let tokens = parse_struct(pattern, item, &mut regexes);
        out_tokens.push(tokens);
    }
    
    out_tokens.into_iter().collect()
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
    let strukt = syn::parse_macro_input!(item as syn::ItemStruct);
    let pattern = syn::parse_macro_input!(attr as syn::LitStr).value();
    let mut hm = std::collections::HashMap::new();
    parse_struct(pattern, strukt, &mut hm)
}

fn extract_items(strukt: &mut syn::ItemStruct) -> Vec<(Option<&syn::Ident>, Ty, Option<String>)> {
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
    items
}

fn get_pattern_str(item: &syn::ItemStruct) -> String {
    // TODO: asssert that attr.path is "parse"
    let pattern_expr: syn::ExprParen = syn::parse2(item.attrs[0].tokens.clone()).unwrap();
    if let syn::Expr::Lit(l) = *pattern_expr.expr {
        if let syn::Lit::Str(ls) = l.lit {
            return ls.value();
        }
    }
    panic!("something went wrong!")
}

fn split_pattern_str(pattern: &str) -> Vec<&str> {
    pattern.split("{}").collect()
}

fn parse_struct(pattern: String, mut strukt: syn::ItemStruct, regexes: &mut std::collections::HashMap<String, String>) -> TokenStream {
    let name = strukt.ident.clone();
    
    // extract name and type for each struct element
    let items = extract_items(&mut strukt);

    // parse groups from input. A group is delimited by curly braces
    let parts = split_pattern_str(&pattern);

    // panic if number of brace pairs in pattern doesn't match number of struct elements
    assert!(parts.len() == items.len()+1);
    //println!("{}, {}", parts.len(), items.len());

    // calculate the regex for each item
    let item_regexes = items.iter().map(|item| {
        // respect custom regex
        if let Some(s) = &item.2 {
            return s.clone();
        }
        let item_type = &item.1.inner_ty();
        // check if type is known (primitive types)
        if let Some(s) = get_regex_for_type(item_type){
            return s.to_string();
        }
        if let Some(s) = regexes.get(*item_type) {
            return s.to_string();
        }
        panic!("Unknown type!");
    });

    // generate the regex pattern string
    let mut regex_str = String::new();
    let mut regex_str_without_groups = String::new();
    for (pattern_prefix, item_regex) in parts.iter().zip(item_regexes) {
        regex_str.push_str(&format!("{}({})", pattern_prefix, item_regex));
        regex_str_without_groups.push_str(&format!("{}{}", pattern_prefix, item_regex));
    }
    regex_str.push_str(parts.last().unwrap());
    regex_str_without_groups.push_str(parts.last().unwrap());

    regexes.insert(name.to_string(), regex_str_without_groups);

    // generate initializers for each struct element
    let mut inits = vec!();
    for (idx, item) in items.iter().enumerate() {
        let name = match &item.0 {
            Some(n) => quote!( #n: ),
            None => quote!()
        };
        let ts = match &item.1 {
            Ty::Simple(_s) => quote!( #name cap[#idx+1].parse().unwrap()),
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


fn get_regex_for_type(ty: &str) -> Option<&'static str> {
    match ty {
        "usize" => Some(r"\d+"),
        "i32" => Some(r"[-+]?\d+"),
        "f64" => Some(r"[0-9]*\.?[0-9]*"),
        "char" => Some(r"."),
        "bool" => Some(r"true|false"),
        _ => None
    }
}
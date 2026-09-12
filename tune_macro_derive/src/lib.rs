use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr::{self, Assign},
    FieldsNamed,
    Lit::Int,
    Result, Token,
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
};

#[derive(Debug)]
struct TunableValue {
    default: i32,
    min: i32,
    max: i32,
    step: i32,
}

impl Parse for TunableValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let content: Punctuated<Expr, syn::token::Comma> =
            input.parse_terminated(Expr::parse, Token![,]).unwrap();

        let mut tv = TunableValue {
            default: 0,
            min: -999999,
            max: 999999,
            step: 10,
        };

        for k in content.into_pairs() {
            let ex: Expr = k.into_tuple().0;

            if let Assign(exa) = ex {
                if let Expr::Path(p) = *exa.left {
                    if let Expr::Lit(l) = *exa.right {
                        if let Int(i) = l.lit {
                            let ipar = i.base10_parse::<i32>().unwrap();

                            if p.path == parse_quote!(default) {
                                tv.default = ipar;
                            } else if p.path == parse_quote!(min) {
                                tv.min = ipar;
                            } else if p.path == parse_quote!(max) {
                                tv.max = ipar;
                            } else if p.path == parse_quote!(step) {
                                tv.step = ipar;
                            }
                        }
                    }
                }
            }
        }

        Ok(tv)
    }
}

#[proc_macro_derive(TuneContainer, attributes(Tunable))]
pub fn tune_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    if let syn::Data::Struct(ds) = ast.data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = ds.fields {
            eprintln!("COMPILING");
            let fields = named.iter().map(|f| &f.ident);
            let fields2 = named.iter().map(|f| &f.ident);
            let tunables = named.iter().map(|f| {
                let k: TunableValue = f
                    .attrs
                    .iter()
                    .find(|p| *p.path() == parse_quote!(Tunable))
                    .unwrap()
                    .parse_args()
                    .unwrap();
                k
            });
            let defaults = tunables.clone().map(|f| f.default);
            let defaults2 = tunables.clone().map(|f| f.default);
            let defaults3 = tunables.clone().map(|f| f.default);
            let mins = tunables.clone().map(|f| f.min);
            let mins2 = tunables.clone().map(|f| f.min);
            let maxs = tunables.clone().map(|f| f.max);
            let maxs2 = tunables.clone().map(|f| f.max);
            let steps = tunables.clone().map(|f| f.step);
            let fieldstr = named
                .iter()
                .map(|f| format!("tune_{}", f.ident.clone().unwrap().to_string()));
            let fieldstr2 = named
                .iter()
                .map(|f| format!("tune_{}", f.ident.clone().unwrap().to_string()));
            let fieldstr3 = named
                .iter()
                .map(|f| format!("tune_{}", f.ident.clone().unwrap().to_string()));

            let name = &ast.ident;
            let generated = quote!(
                impl Default for #name {
                    fn default() -> Self {
                        Self {
                            #(#fields2: #defaults),*
                        }
                    }
                }

                impl TuneContainer for #name {
                    fn get_refmut(&mut self, nm: &str) -> &mut i32 {
                        match nm {
                            #(#fieldstr => &mut self.#fields),*,
                            _ => { panic!() }
                        }
                    }

                    fn list_options(&self) {
                        #(println!("option name {} type spin default {} min {} max {}", #fieldstr2, #defaults2, #mins, #maxs));*
                    }

                    fn list_wf_json_config(&self) {
                        println!("{{");
                        #(println!("\n\t\"{}\": {{\n\t\t\"value\": {},\n\t\t\"min_value\": {},\n\t\t\"max_value\": {},\n\t\t\"step\": {}\n\t}},", #fieldstr3, #defaults3, #mins2, #maxs2, #steps));*;
                        println!("}}");
                    }
                }
            );

            generated.into()
        } else {
            panic!()
        }
    } else {
        panic!()
    }
}
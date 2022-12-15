use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(EnumIter)]
pub fn my_enum_iter(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    return impl_my_enum_iter(&ast);
}

fn impl_my_enum_iter(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl EnumIter for #name {
            fn iter() -> Vec<#name> {
                let vec = all::<#name>().collect::<Vec<_>>();
                // remove anything like None, Empty, Unknown, or Any
                let vec = vec.into_iter().filter(|x| {
                    let s = format!("{:?}", x).to_lowercase();
                    !s.contains("none") && !s.contains("empty") && !s.contains("unknown") && !s.contains("any")
                }).collect::<Vec<_>>();
                return vec;
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(EnumString)]
pub fn my_enum_string(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    return impl_my_enum_string(&ast);
}

fn impl_my_enum_string(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                // Convert PascalCase to Kebab-Case
                fn pascal_to_kebab(pascal_string: String) -> String {
                    let mut kebab = String::new();
                    let mut first = true;

                    for c in pascal_string.chars() {
                        if c.is_uppercase() {
                            if !first {
                                kebab.push('-');
                            }
                        }
                        kebab.push(c);
                        first = false;
                    }

                    return kebab;
                }
                let kebab = pascal_to_kebab(format!("{:?}", self));
                write!(f, "{}", kebab)
            }
        }

        impl From<#name> for String {
            fn from(t: #name) -> Self {
                return t.to_string();
            }
        }

        impl From<String> for #name {
            fn from(s: String) -> Self {
                return #name::from(s.as_str());
            }
        }

        impl From<&str> for #name {
            fn from(s: &str) -> Self {
                return #name::from(s);
            }
        }

    };
    gen.into()
}

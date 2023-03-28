/* --- Required Code for most procedural macro crates to process Rust code -- */

use proc_macro::TokenStream;
use quote::quote;
use syn;

// *The hello_macro_derive function will be called when a user of our library specifies #[derive(HelloMacro)] on a type.
// because we’ve annotated the hello_macro_derive function here with proc_macro_derive and specified the name HelloMacro, which matches our trait name; 
// this is the convention most procedural macros follow.

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

// Implementing the HelloMacro trait using the parsed Rust code
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // The quote! macro lets us define the Rust code that we want to return.
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    // by calling the into method, which consumes this intermediate representation 
    // and returns a value of the required TokenStream type.
    gen.into()
}

/* -------------------------------------------------------------------------- */
/*                                Inner Working                               */
/* -------------------------------------------------------------------------- */
// *Notice that we’ve split the code into 
// 
// the hello_macro_derive function, which is responsible for parsing the TokenStream, 
// and the impl_hello_macro function, which is responsible for transforming the syntax tree: 
// this makes writing a procedural macro more convenient.
// 
// The code in the outer function (hello_macro_derive in this case) 
// will be the same for almost every procedural macro crate you see or create

// The code you specify in the body of the inner function (impl_hello_macro in this case) 
// will be different depending on your procedural macro’s purpose.

/* ------- From Parsing TokenStream into DeriveInput with syn::parse ------- */

// *DeriveInput struct

// DeriveInput {
//     // --snip--

//     ident: Ident {
//         ident: "Pancakes",
//         span: #0 bytes(95..103)
//     },
//     data: Struct(
//         DataStruct {
//             struct_token: Struct,
//             fields: Unit,
//             semi_token: Some(
//                 Semi
//             )
//         }
//     )
// }
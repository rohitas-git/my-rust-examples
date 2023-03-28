/* -------------------------------------------------------------------------- */
/*                          Info abt Attribute Macro                          */
/* -------------------------------------------------------------------------- */

// *Attribute-like macros allow you to create new attributes.
// attributes can be applied to other items as well, such as functions.

// #[route(GET, "/")]
// fn index() {}
// 
// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
// 
// Here, we have two parameters of type TokenStream. 
// The first is for the contents of the attribute: the GET, "/" part. 
// The second is the body of the item the attribute is attached to: in this case, fn index() {} and the rest of the function’s body.

// Other than that, attribute-like macros work the same way as custom derive macros: 
// *you create a crate with the proc-macro crate type and implement a function that generates the code you want! 
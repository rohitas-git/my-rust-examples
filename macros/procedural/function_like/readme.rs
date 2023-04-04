/* -------------------------------------------------------------------------- */
/*                          Info Function-like Macros                         */
/* -------------------------------------------------------------------------- */

// *Function-like macros define macros that look like function calls.
// 
// Similarly to macro_rules! macros, they’re more flexible than functions; 
// for example, they can take an unknown number of arguments.
// 
// *Function-like macros take a TokenStream parameter and their definition manipulates that TokenStream using Rust code as the other two types of procedural macros do.

// let sql = sql!(SELECT * FROM posts WHERE id=1);
// 
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {}
// 
// *we receive the tokens that are inside the parentheses and return the code we wanted to generate.

/// * The "self" word is a pseudo-argument of a method, 
/// * and the "Self" word represents the type of such an argument.
/// Therefore, "self" and "Self" can be used only inside a "trait or impl block. 
/// And "self", if present, must be the first argument of a method.
/// 
/// In the statement "trait CanBeDoubled { fn double(self) -> Self; }", 
/// "self" means the value on which the "double" method will be applied, whichever it will be,
/// and "Self" means the type of "self".
/// 
/// 
/// Insidethe"impl CanBeDoubled for i32"block,thefollowingsixlinesareequivalent:
/// 
/// fn double(self) -> Self {
/// fn double(self: Self) -> Self { 
/// fn double(self: i32) -> Self { 
/// fn double(self) -> i32 {
/// fn double(self: Self) -> i32 { 
/// fn double(self: i32) -> i32 {
/// 
mod self_Self{


}
/* -------------------------------------------------------------------------- */
/*                         Memory Usage of Composition                        */
/* -------------------------------------------------------------------------- */
// There is no difference between the memory usage of composition and that of
// inheritance. They both use just the needed memory:

struct Base1 {
    _x: f64
}
struct Base2 {}
struct Derived1 {
    _b1: Base1,
    _b2: Base2,
}
struct Derived2 {
    _d1: Derived1,
    _other: f64,
}

use std::mem::size_of;
fn main(){
print!("{} {} {} {}",
    size_of::<Base1>(), size_of::<Base2>(),
    size_of::<Derived1>(), size_of::<Derived2>());
}
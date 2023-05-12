// Generic Function Shorthand

fn my_fn(foo: impl Foo){}

fn my_fn_long<T>(foo:T)
    where 
    T:Foo
    {}

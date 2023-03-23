mod MutatingMethods{
    struct S { x: u32 }
    impl S {
        fn get_x(&self) -> u32 { self.x }
        fn set_x(&mut self, x: u32) { self.x = x; }
    }
}


mod if_rust_was_dynamic_lang{
    fn draw_text(txt) {
        txt.draw();
    }
}

// Solution: -Static Dispatch   -Dynamic Dispatch

/* -------------------------------------------------------------------------- */
/*                               Static Dispatch                              */
/* -------------------------------------------------------------------------- */
// This technique is named “static dispatch”
// In computer science, 
// * “dispatch” means choosing which function to invoke when there are several functions with the same name.
mod solution_1{
    trait Draw {
        fn draw(&self);
    }
    impl Draw for Text{}
    impl Draw for Boxed_Text{}

    // SOLUTION 1 //
    fn draw_text<T>(txt: T) where T: Draw {
        txt.draw();
    }

    // In this program there are two functions named "draw", 
    // and so a dispatch is required to choose between them. 
    // * In this program, this choice is performed by the compiler, at compile time, and so this dispatch is “static”.
}

mod static_dispatch{
    // SOLUTION 1 //
    fn draw_text<T>(txt: T) where T: Draw {
        txt.draw();
    }

    // SOLUTION 1 //
    fn draw_text<T>(txt: &T) where T: Draw {
        txt.draw();
    }

    // we see that static dispatch can work both with pass-by-value and with pass-by-reference.
}
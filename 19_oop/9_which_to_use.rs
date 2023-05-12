// Which should you use?

// Like any instance of static-vs-dynamic dilemma, where “static” means “compile-­
// time”, and “dynamic” means “run-time”, static requires a somewhat longer compilation
// time, and generates somewhat faster code, but if not enough information is available to
// the compiler, the dynamic solution is the only possible one.

mod AssumeProblem{
    // Assume that, for the example program showed before the solutions to the dispatch
    // problem, there is the following requirement. The user is asked to enter a string, and if
    // that string is “b”, a boxed text should be printed, and for any other input a non-boxed text
    // should be printed.

    mod Static_Dispatch_Solution{
        fn draw_text<T>(txt: T) where T: Draw {
            txt.draw();
        }
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "b" {
            draw_text(boxed_greeting);
        } else {
            draw_text(greeting);
        }
    }

    mod Dynamic_Dispatch_Solution{
        fn draw_text(txt: &Draw) {
            txt.draw();
        }
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let dr: &Draw = if input.trim() == "b" {
            &boxed_greeting
        } else {
            &greeting
        };
        draw_text(dr);
    }

    // The static dispatch requires you to write several function invocations, while the
    // dynamic dispatch allows you to save in the "dr" variable the object chosen by the user,
    // and then write just one function invocation for it.
    // 
    // In addition, static dispatch uses generic functions, and this technique may create
    // code bloat, and so it may end up being slower.
}
mod mutable_borrow{
    // Borrowing Mutable Ownership with References
    // We can also borrow mutable access to a resource with the &mut operator.

    // A resource owner cannot be moved or modified while mutably borrowed.

    // Memory detail:

    // Rust prevents having two ways to mutate an owned value because it introduces the possibility of a data race.

    struct Foo {
        x: i32,
    }

    fn do_something(f: Foo) {
        println!("{}", f.x);
        // f is dropped here
    }

    fn main() {
        let mut foo = Foo { x: 42 };
        let f = &mut foo;

        // FAILURE: do_something(foo) would fail because
        // foo cannot be moved while mutably borrowed

        // FAILURE: foo.x = 13; would fail here because
        // foo is not modifiable while mutably borrowed

        f.x = 13;
        // f is dropped here because it's no longer used after this point
        
        println!("{}", foo.x);
        
        // this works now because all mutable references were dropped
        foo.x = 7;
        
        // move foo's ownership to a function
        do_something(foo);
    }
}

mod passing_Around{
    // Passing Around Borrowed Data
    // Rust's rules for references might best be summarized by:
    
    // Rust only allows there to be one mutable reference or multiple non-mutable references but not both.
    // A reference must never live longer than its owner.
    // This doesn't tend to be a problem when passing around references to functions.
    
    // Memory details:
    
    // The first rule of references prevents data races. What's a data race? A data race when reading from data has the possibility of being out of sync due to the existence of a writer to the data at the same time. This happens often in multi-threaded programming.
    // The second rule of references prevents the misuse of references that refer to non-existent data (called dangling pointers in C).
    struct Foo {
        x: i32,
    }
    
    fn do_something(f: &mut Foo) {
        f.x += 1;
        // mutable reference f is dropped here
    }
    
    fn main() {
        let mut foo = Foo { x: 42 };
        do_something(&mut foo);
        // because all mutable references are dropped within
        // the function do_something, we can create another.
        do_something(&mut foo);
        // foo is dropped here
    }
    
}
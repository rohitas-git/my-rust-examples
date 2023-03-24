// Text and Boxed_Text defined in 4_CompositionOverInheritance

use std::mem::size_of_val;
print!("{} {} {}, {} {} {}, ",
    size_of_val(&greeting), // 24
    size_of_val(&&greeting), //8
    size_of_val(&&&greeting),//8
    size_of_val(&boxed_greeting), //32
    size_of_val(&&boxed_greeting),//8
    size_of_val(&&&boxed_greeting));//8

fn draw_text(txt: &Draw) {
    print!("{} {} {} ",
        size_of_val(txt),
        size_of_val(&txt),
        size_of_val(&&txt));
    txt.draw();
}

draw_text(&greeting); // 24 16 8 Hello
print!(", "); 
draw_text(&boxed_greeting); // 32 16 8 [Hi]

/*

Then the expression "&greeting", having type "&Text", is passed as an argument to the
"draw_text" function, where it is used to initialize the argument "txt", having type "&Draw".

The "txt" argument is a kind of reference, and so it is possible to evaluate the
expression "size_of_val(txt)". It will return the size of the referred object. 
But which type has the object referenced to by an object of type "&Draw"? Of course, it is not "Draw",
because "Draw" is not a type. Actually, this cannot be said at compile time. It depends on
the object referenced at runtime by the expression used to initialize the "txt" argument.
The first time that the "draw_text" function is invoked, the "txt" argument receives a
reference to a "Text" object, and so 24 is printed.

*/

// * Why is a reference to a trait so large?
// * The mechanism of dynamic dispatch lies in how a reference to a trait is initialized.
// * Actually any reference to a trait has two fields. 
// * The first one is a copy of the reference used to initialize it, 
// * and the second one is a pointer used to choose the proper version of the "draw" function, or any other function needing dynamic dispatch. 
// * It is named a "virtual table pointer". This name comes from C++.

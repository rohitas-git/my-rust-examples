/* -------------------------------------------------------------------------- */
/*                                 RC Pointer                                 */
/* -------------------------------------------------------------------------- */

// Rust is known to be notorious difficult when it comes to certain data structures like linked lists, trees, etc. 
// 
// *Many of such data structures have in common the necessity of several variables pointing to one value. 
// 
// For instance in a graph, two vertices could have a connection to a shared vertex. 
// But when removing just one of these vertices, the shared vertex should not be dropped. 
// In other words, non of these vertices can own the shared vertex in the strict sense.

// Rust’s answer to this requirement is a Rc reference. 
// A Rc reference owns a so called ‘inner value’. 
// Moreover, we can clone borrowed references to Rc references 
// and thus can have many such clones referring to the same Rc‘s inner value. 
// This is the solution to the above stated problem, 
// since now the shared vertex could be wrapped as inner value by a Rc reference (the owner of the vertex) 
// and further all vertices having a connection to this shared vertex could be given a clone of this Rc reference:

use std::rc::Rc;
let ref = Rc::new(value); // moves 'value' into Rc

let clone1 = Rc::clone(&ref); // create a clone of a reference
let clone2 = Rc::clone(&ref); // creates another clone

// If we just had moved the shared vertex into one of the others, and had provided a borrowed reference to the others, 
// the compiler would have complained for not being able to ensure the reference to always point to a still existing value. 
// For instance, the new owner of the moved value could be dropped somewhere without the borrowed reference being aware of it.

// Rc stands for reference-counter, and this is doing what it says: 
// whenever a reference to the Rc‘s inner value is obtained by calling Rc:clone(&ref), a counter is increased by 1. 
// Whenever such a cloned reference is getting dropped because its scope ends, the counter is informed and decreases by 1. 
// This way, the Rc instance always knows how many references to its inner value actively exists. 
// When the counter runs to 0 and the Rc instance goes out of scope, it will be get dropped together with its inner value.

// One important fact to acknowledge about Rc references is that they only provide immutable access to their inner value.

// Let us wrap up: Use Rc references always in scenarios where you need references onto a value that are decoupled from the value’s ‘normal’ lifetime. 
// In that sense, the value first gets dropped, when no further reference is pointing to it. 
// Use RefCells whenever you need to obtain mutable reference on the inner values of a Rc reference.
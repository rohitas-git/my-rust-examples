/* -------------------------------------------------------------------------- */
/*                                   RefCell                                  */
/* -------------------------------------------------------------------------- */

// One important fact to acknowledge about Rc references is that they only provide immutable access to their inner value. 
// This ensures that two such references never can mutate the same value at the same time.

// Though, sometimes it is desired to allow mutations of a value through a Rc reference by taking care only one such mutation exists at a time. 
// For this Rust provides a so called RefCell. 
// *This is another wrapper of an inner value that allows 
// *to obtain a mutable reference even though the RefCell itself is accessed through an immutable reference like a Rc.

use std::rc:Rc;
use std:cell::RefCell;

let ref = Rc::new(RefCell::new(value)); //moves 'value' into RefCell
let clone1 = Rc::clone(&ref); // creates a clone

// creates a mutable reference on value
let mutRef = clone1.borrow_mut()

let clone2 = Rc::clone(&ref); // creates another clone
clone2.borrow_mut()  // creates another mutable reference on value

// Important here is that the mutable reference provided by borrow_mut() must get dropped before another such mutable reference is obtained. 
// As usually dropping of such a reference occurs when its scope is over or the compiler can ensure its value is not being used anymore.

// Let us wrap up: 
// Use Rc references always in scenarios where you need references onto a value that are decoupled from the value’s ‘normal’ lifetime. 
// In that sense, the value first gets dropped, when no further reference is pointing to it. 
// Use RefCells whenever you need to obtain mutable reference on the inner values of a Rc reference.
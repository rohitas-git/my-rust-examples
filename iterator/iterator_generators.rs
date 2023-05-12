use iterator::*;

fn main(){
    chars::string_to_u8_slice("€èe");
    
    string_iterator::iterator_char_of_str("€èe", 4);
    string_iterator::iter_bytes_of_str();

    string_iterator::print_codes_of_char("€èe");
    
    string_iterator::string_iter_v0("Rohitas---Bansal");
    string_iterator::string_iter_v1();
    string_iterator::string_iter_v1();

    // string_iterator::range_iterator();
    

    slice_arr_vec_iterator::invoking_iter_for_slice_arr_vec();

    // iterations_without_mutation::try_change_range_value();
    iterations_without_mutation::try_change_loop_variable();
    iteration_with_mutation::mutating_iterator();

}

// * Iterator: 
// * In cs, the iterator is an object that performs the behavior of extracting an item
// * at the current position in a sequence, and then advance that position.

// * But what is exactly an iterator? It is not a type, but a type specification. 
// * An iterator is considered to be any expression that has a "next" method returning an Option<T> value.

// *"Iterator generators": functions that don't get iterators but return iterators
// * chars, bytes, iter, iter_mut

pub mod iterator{

    /// *String is array of bytes that represents a sequence of chars 
    /// *Unicode Character represented by a sequence of 1 to 6 bytes 
    /// *string[index] is not allowed in Rust <- Because variable-length bytes sequence of char
    /// *Why need string iterator? -> Need to scan all the prev char
    pub mod chars{
        pub fn string_to_u8_slice(given_string:&str){
            //1. Convert string to a slice of bytes
                // * as_bytes gives immutable slice refrence of [u8] that is the representation of the string. 
            //2. To get a char in a given position, need to scan all the previous chars
                // Situation of reading the nth line in a variable-line-length file
            let s = given_string;
            for i in 0..s.len() {
                println!("{}: {}", i, s.as_bytes()[i]);
            }
        }
    }
    
    ///  * Iterator for string's [Chars | Bytes]
    /// * s.chars() - iterator over string's chars
    /// * s.bytes() - iterator over string's bytes
    pub mod string_iterator{
    
        /// * std::str::Chars - string iterator type
        /// * s.chars() - iterator over string's chars
        /// * next fn returns the next char of string
        /// 'next' function returns the next item of the 
        /// underlying sequence at the current position, and advances the current position.
        pub fn iterator_char_of_str(s: &str, mut n: u32) {
            let mut iter: std::str::Chars = s.chars();
            loop {
                let item: Option<char> = iter.next();
                match item {
                    Some(c) => if n == 1 { print!("{}", c); },
                    None => { println!("The End"); break; },
                }
                n -= 1;
            }
        }
    
        // * std::str::Bytes - string iterator type
        // * s.bytes() - iterator over string s
        // * 'next' fn returns the next bytes of string
        pub fn iter_bytes_of_str(){
            let string: &str = "€èe";
            let string_it: std::str::Bytes = string.bytes();
            for byte in string_it {
                print!("{} ", byte);
            }
        }
    
        /// print the numeric codes of its chars
        pub fn print_codes_of_char(s: &str) {
            let mut iter = s.chars();
            loop {
                match iter.next() {
                    Some(c) => { println!("{}: {}", c, c as u32); },
                    None => { break; },
                }
            }
        }
    
        // * Typically used, iterator of string's characters
        pub fn string_iter_v0(s:&str){
            for c in s.chars() {
                print!("{} ", c);
            }
            println!("");
        }
    
        pub fn string_iter_v1(){
            //  the string iterator on the bytes of a string is created using the bytes function
            for byte in "€èe".bytes() {
                print!("{} ", byte);
            }
        }
    
        // getting iterator on string's byte represenetation's slice ref
        pub fn string_iter_v2(){
            // iterate over the bytes of a string is first to create the slice reference 
            // over the bytes of the string, using the as_bytes function, 
            // and then iterate over such slice reference.
            for byte in "€èe".as_bytes().iter() {
                print!("{} ", byte);
            }
        }
        
    
        pub fn range_iterator(){
            // *OK: std::ops::Range<u32> is an iterator
            let _v1 = (0u32..10).next();
            // *OK: std::ops::RangeFrom<u32> is an iterator
            let _v2 = (5u32..).next();
            // Illegal: std::ops::RangeTo<u32> is not an iterator
            // let _v3 = (..8u32).next();
            // Illegal: std::ops::RangeFull is not an iterator
            // let _v4 = (..).next();
        }
        
    }
    
    /// Slice, Array, Vector iterator  obtained by invoking the iter function
    /// The iter function returns a value of type std::slice::Iter<T>
    /// The type of returned values is an iterator type, and so it can be used in a for loop
    /// When iterating over a sequence of T type, the loop variable is of &T type, that is, it is a reference.
    /// LOOP VARIABLE: to access its value, a dereference operator (*) can (and sometimes must) be applied.
    /// ILLEGAL : In fact, the loop variable is immutable
    /// 
    pub mod slice_arr_vec_iterator{
        pub fn invoking_iter_for_slice_arr_vec(){
            // * Slice Iterator
            for item_ref in (&[11u8, 22, 33]).iter() {
                // *item_ref += 1;  ILLEGAL : In fact, the loop variable is immutable
                print!("{} ", *item_ref);
            }
            // * Array Iterator
            for item_ref in [44, 55, 66].iter() {
                // *item_ref += 1; ILLEGAL : In fact, the loop variable is immutable
                print!("{} ", *item_ref);
            }
            // * Vector Iterator
            for item_ref in vec!['a', 'b', 'c'].iter() {
                // *item_ref = if *item_ref == 'b' { 'B' } else { '-' }; ILLEGAL : In fact, the loop variable is immutable
                print!("{} ", *item_ref);
            }
        }
    
        fn breakdown_invokocation(){
            let slice: &[u8] = &[11u8, 22, 33];
            let slice_it: std::slice::Iter<u8> = slice.iter();
            for item_ref in slice_it {
                // *item_ref += 1;
                print!("{} ", *item_ref);
            }
            let arr: [i32; 3] = [44, 55, 66];
            let arr_it: std::slice::Iter<i32> = arr.iter();
            for item_ref in arr_it {
                // *item_ref += 1;
                print!("{} ", *item_ref);
            }
            let vec: Vec<char> = vec!['a', 'b', 'c'];
            let vec_it: std::slice::Iter<char> = vec.iter();
            for item_ref in vec_it {
                // *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
                print!("{} ", *item_ref);
            }
        }
    
    }
    
    pub mod iterations_without_mutation{
        // Above, we have used iterator only to read sequences 
    
        // * When iterating over the characters of a string, 
        // it is unreasonable to try to change them,
        // as the new character may be represented by a different number of bytes than the existing character
        // Therefore, the Rust standard library has no way to change a string character-by-character using a character string iterator.
        
        // * When iterating over the bytes of a string,
        // it is unsafe to try to change them, as the new byte may be creating an invalid UTF-8 string. 
        // Therefore, the Rust standard library has no way to change a string byte-by-byte using a byte string iterator.
    
        // * When iterating over a range, 
        // as we have already seen, the range value used by the loop is the one at the beginning of the loop, even if it is changed inside the loop:
        pub fn try_change_range_value(){
            let mut r = "abc".chars(); 
            for i in r {
                r = "XY".chars();
                print!("{} {}; ", i, r.next().unwrap());
            }
        }
        // The increment inside the loop is possible because i has the mut clause, 
        // but i is reinitialized at the next iteration.
        pub fn try_change_loop_variable(){
            let r = 0..5; 
            for mut i in r {
                print!("Before modifying loop variable, i: {} ", i);
                i += 10;
                println!("After :{} ", i);
            }
        }
        // * Therefore, for strings and for ranges there is no need of iterators that allow changes to the items of the sequence.
    }
    pub mod iteration_with_mutation{
        // * Mutable Iterator:
        // In fact, a mutable iterator is something that can and may be made to iterate over another sequence, 
        // not that can be used to mutate the sequence that it iterates over.
    
        // An iterator is similar to a reference, 
        // in that a mutable reference is not the same concept of a reference to a mutable object.
        #[allow(dead_code)]
        // fn false_change_sequence(){
        //     // Although this program contains several mut clauses, 
        //     // it generates a compiler error at the line in the body of the loop, 
        //     // because *item_ref is still immutable.
        //     let mut slice = &mut [3, 4, 5]; {
        //         let mut iterator = slice.iter(); 
        //         for mut item_ref in iterator {
        //             *item_ref += 1;
        //         }
        //     }
        //     print!("{:?}", slice);
        // }
    
        //For such a purpose, you need another type of iterator, 
        // * a mutating iterator, 
        // which, of course, must be initialized over a mutable sequence.
        pub fn mutating_iterator(){
            let slice = &mut [3, 4, 5]; 
            {
                let iterator = slice.iter_mut(); 
                for item_ref in iterator {
                    *item_ref += 1;
                }
            }
            print!("{:?}", slice);
        }
    
        pub fn invoking_iter_mut_for_slice_arr_vec(){
            // * Mutating slice through itertator_mutable
            for item_ref in (&mut [11u8, 22, 33]).iter_mut() { 
                *item_ref += 1;
                print!("{} ", *item_ref);
            }
            // * Mutating array through itertator_mutable
            for item_ref in [44, 55, 66].iter_mut() { *item_ref += 1;
                print!("{} ", *item_ref);
            }
            // * Mutating vector through itertator_mutable
            for item_ref in vec!['a', 'b', 'c'].iter_mut() { *item_ref = if *item_ref == 'b' { 'B' } else { '-' }; print!("{} ", *item_ref);
            }
        }
    
        fn breakdown_invokocation_iter_mutable(){
            let slice: &mut [u8] = &mut [11u8, 22, 33];
            let slice_it: std::slice::IterMut<u8> = slice.iter_mut(); for item_ref in slice_it {
                *item_ref += 1;
                print!("{} ", *item_ref);
            }
            let mut arr: [i32; 3] = [44, 55, 66];
            let arr_it: std::slice::IterMut<i32> = arr.iter_mut(); for item_ref in arr_it {
                *item_ref += 1;
                print!("{} ", *item_ref);
            }
            let mut vec: Vec<char> = vec!['a', 'b', 'c'];
            let vec_it: std::slice::IterMut<char> = vec.iter_mut(); for item_ref in vec_it {
                *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
                print!("{} ", *item_ref);
            }
        }
    }
}



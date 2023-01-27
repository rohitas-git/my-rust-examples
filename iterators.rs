// * Iterator: 
// * In cs, the iterator is an object that performs the behavior of extracting an item
// * at the current position in a sequence, and then advance that position.

// * But what is exactly an iterator? It is not a type, but a type specification. 
// * An iterator is considered to be any expression that has a "next" method returning an Option<T> value.
fn main(){
    chars::string_to_u8_slice("€èe");
    
    string_iterator::print_nth_char("€èe", 4);
    string_iterator::print_codes("€èe");
    // string_iterator::range_iterator();
    string_iterator::iter_bytes_of_str();

    slice_arr_vec_iterator::invoking_iter_for_slice_arr_vec();
    // slice_arr_vec_iterator::string_iter_v1();
    // slice_arr_vec_iterator::string_iter_v2();

}

pub mod chars{
    /// *String is array of bytes that represents a sequence of chars 
    /// *Unicode Character represented by a sequence of 1 to 6 bytes 
    /// *string[index] is not allowed in Rust <- Because variable-length bytes sequence of char
    /// *Why need string iterator? -> Need to scan all the prev char
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

pub mod string_iterator{
    /// Scan the string and print the req char
    pub fn print_nth_char(s: &str, mut n: u32) {
        // * std::str::Chars - string iterator type
        // * s.chars() - iterator over string s
        // * next fn returns the next char of string
        // 'next' function returns the next item of the 
        // underlying sequence at the current position, and advances the current position.
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
    /// print the numeric codes of its chars
    pub fn print_codes(s: &str) {
        let mut iter = s.chars();
        loop {
            match iter.next() {
                Some(c) => { println!("{}: {}", c, c as u32); },
                None => { break; },
            }
        }
    }

    pub fn for_loop_iterator(s:&str){
        for c in s.chars() {
            println!("{}: {}", c, c as u32);
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

    pub fn iter_bytes_of_str(){
        // * std::str::Bytes - string iterator type
        // * s.bytes() - iterator over string s
        // * 'next' fn returns the next bytes of string
        for byte in "€èe".bytes() {
            println!("{} ", byte);
        }

        // BREAKDOWN
        // let string: &str = "€èe";
        // let string_it: std::str::Bytes = string.bytes();
        // for byte in string_it {
        //     print!("{} ", byte);
        // }
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
    pub fn string_iter_v1(){
        //  the string iterator on the bytes of a string is created using the bytes function
        for byte in "€èe".bytes() {
            print!("{} ", byte);
        }
    }
    pub fn string_iter_v2(){
        // iterate over the bytes of a string is first to create the slice reference 
        // over the bytes of the string, using the as_bytes function, 
        // and then iterate over such slice reference.
        for byte in "€èe".as_bytes().iter() {
            print!("{} ", byte);
        }
    }

}

pub mod iterator_adapters{
}

pub mod iterator_consumers{
}

pub mod lazy_processing_iterator_chains{
}


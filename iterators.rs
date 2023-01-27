fn main(){


}

pub mod chars_in_rust{
    /// String is array of bytes that represents a sequence of chars 
    /// Unicode Character represented by a sequence of 1 to 6 bytes 
    /// string[index] is not allowed in Rust
    /// 
    fn get_first_char(given_string:String){
        //1. Convert string to a slice of bytes
            // as_bytes gives immutable slice of u8 in the representation of the string. 
        let s = given_string;
        for i in 0..s.len() {
            println!("{}: {}", i, s.as_bytes()[i]);
        }
    }
}

pub mod string_char_bytes_iterator{
}

pub mod slice_arr_vec_iterator{
}

pub mod iterator_adapters{
}

pub mod iterator_consumers{
}

pub mod lazy_processing_iterator_chains{
}


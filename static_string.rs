fn main(){
    // illegal_str();
    static_string_ex();
    abnormal_str_ref_header();
}

///! Static Strings:
/// String objects can be changed to refer to other string content
/// But they have immutable content, i.e
/// * it is not possible to overwrite some characters or add or remove characters 
///
///! &str (string literal):
/// * str is defined as the type of unmodifiable undefined-length sequence of bytes representing a UTF-8 String
/// 
/// Each time the compiler parses a literal string,
/// it stores in a static program area the characters of that string, and such area is of str
/// type. Then the compiler uses a reference to such area as the value of such literal string
/// expression, and so any string literal is of type &str.
/// 
fn static_string_ex(){
    use std::mem::*;
    let a: &str = "";
    let b: &str = "0123456789";
    let c: &str = "abcdè"; // 5 chars, (a,b,c,d;1byte) (è;2bytes)
    println!("{} {} {}",
    size_of_val(a),//0 bytes
    size_of_val(b), //10 bytes
    size_of_val(c)); //6 bytes
}

///! Limitation of &str: 
/// 
/// Remember that function (size_of_val) returns the size of the object referenced by its 
/// argument. 
/// If the argument is a, that is of type &str, this function returns the size of the
/// string buffer referenced by a, which is of type str.
/// 
/// Notice that the buffers referred to by the a, b, and c variables are of the same type, 
/// which is str, but they have different lengths: 0, 10, and 6
/// 
/// So here, we see a type that doesn't have an associated length
/// 
/// ! Limitation of type that doesn't have an associated length:
/// - Cannot declare a variable or a fn of such type
/// - Cannot ask size of such type
/// 
// fn illegal_str(){
//     let a: str; 
//     fn f(a: str) {}
//     print!("{}", std::mem::size_of::<str>());
// }

///! &str is not a normal Rust reference
/// 
/// Header:
/// - Pointer: the address of the beginning of the string buffer,
/// - Length: number of bytes of the string buffer
/// 
/// Such variables result in sizes that are twice as large as that of a normal 
/// reference, as they contain a pointer object and a usize object
fn abnormal_str_ref_header(){
    use std::mem::*;
    let a: &str = "";
    let b: &str = "0123456789";
    let c: &str = "abcdè";
    println!("{} {} {}; ",
    size_of_val(&a), // size of variable, &str type
    size_of_val(&b), //usize*2
    size_of_val(&c));
    print!("{} {} {}",
    size_of_val(&&a), //usize 
    size_of_val(&&b), // reference to variable &str; normal reference
    size_of_val(&&c));
}

// * https://coderscat.com/rust-string-or-str/
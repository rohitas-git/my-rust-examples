
fn main(){
    dynamic_string();
    string_structure();
    create_strings();
    convert_static_to_dynamic_string();
    concatenate_strings();
    append_string();
    append_string_obj_char();
    implicit_conv_ref_string();
}

//-----------------------------------------------------------------//

///! Dynamic String:
/// 
/// any Rust dynamic string, like any Rust static string,
///! contains an array of bytes that represent a UTF-8 string; 
/// it does not contain an array of characters.
/// 
///! Similarities:
/// 
/// While static string buffers are similar to arrays, that is, 
/// the str type is similar to the generic [u8; N] type, 
/// 
/// dynamic strings are similar to vectors of bytes, that is, 
/// the String type is similar to the Vec<u8> type.
/// 
fn dynamic_string(){
    // let mut a: String = "He".to_string();
    // a.push('l');
    // a.push('l');
    // a.push('o');
    // print!("{}", a);

    let mut a: String = "Xy".to_string(); // "Xy"
    a.remove(0); // "y"
    a.insert(0, 'H'); // "Hy"
    a.pop(); // "H"
    a.push('i'); // "Hi"
    println!("{}", a);
}

///! Structure of dynamic string
///  
/// In addition, both dynamic strings and vectors have the same implementation. 
/// 
/// Both are structures consisting of three fields:
/// 
/// -Pointer:   the address of the beginning of the heap-allocated buffer containing the data items; 
/// -Capacity:  the number of items that may be contained in the allocated buffer;
/// -Length:    the number of items presently used in the allocated buffer.
/// 
/// However, notice that for the strings, such “items” are bytes, not characters:
/// 
fn string_structure(){
    let mut s = "".to_string();
    for _ in 0..10 {
        println!("{:?} {} {}",
        s.as_ptr(), s.capacity(), s.len());
        s.push('a');
    }
    println!("{:?} {} {}: {}",
    s.as_ptr(), s.capacity(), s.len(), s);
    
    // let mut s1 = "".to_string();
    // s1.push('e');
    // let mut s2 = "".to_string();
    // s2.push('è');
    // let mut s3 = "".to_string();
    // s3.push('€');
    // print!("{} {}; ", s1.capacity(), s1.len());
    // print!("{} {}; ", s2.capacity(), s2.len());
    // print!("{} {}", s3.capacity(), s3.len());
}

fn create_strings(){
    // except this, all other can also be used to convert a non-empty static string to a dynamic string
    let s1 = String::new(); 

    let s2 = String::from("");
    let s3 = "".to_string();
    let s4 = "".to_owned();
    let s5 = format!("");

    println!("(`|{}{}{}{}{}|`)", s1, s2, s3, s4, s5);
}

fn convert_static_to_dynamic_string(){
    let s = "a,";
    let s1 = String::from(s);
    let s2 = s.to_string();
    let s3 = s.to_owned();

    // Indeed, the format macro, like the print and println macros, require that their
    // first argument is a literal, and that such literal contains as many placeholders as the
    // successive arguments to the macro.
    //
    //let s4 = format!(s);
    //let s5 = format!("a,{}");  
    
    let s6 = format!("{}", s);
    println!("({}{}{}{})", s1, s2, s3, s6);
}

///! A dynamic string can be obtained also by
/// concatenating two static strings, 
/// two dynamic strings, 
/// or a dynamic string and a static string.
fn concatenate_strings(){
    let ss1 = "He";
    let ss2 = "llo ";
    let ds1 = ss1.to_string();
    let ds2 = ss2.to_string();

    let ds3 = format!("{}{}", ss1, ss2);
    print!("{}", ds3);
    
    let ds3 = format!("{}{}", ss1, ds2);
    print!("{}", ds3);
    
    let ds3 = format!("{}{}", ds1, ss2);
    print!("{}", ds3);
    
    let ds3 = format!("{}{}", ds1, ds2);
    println!("{}", ds3);
}

fn append_string(){
    let mut dyn_str = "Hello".to_string();
    dyn_str = format!("{}{}", dyn_str, ", ");
    dyn_str = format!("{}{}", dyn_str, "world");
    dyn_str = format!("{}{}", dyn_str, "!");
    println!("{}", dyn_str);

    // This is a better way:
    // The function push_str takes a static string and 
    // pushes all its characters to the end of the receiving string.
    let mut dyn_str = "Hello".to_string();
    dyn_str.push_str(", ");
    dyn_str.push_str("world");
    dyn_str.push_str("!");
    println!("{}", dyn_str);

    // More compact form than using fn push_str:
    // The += operator, when applied to a String object, 
    // is equivalent to the push_str function.
    let mut dyn_str = "Hello".to_string();
    dyn_str += ", ";
    dyn_str += "world";
    dyn_str += "!";
    print!("{}", dyn_str);
}

fn append_string_obj_char(){
    let comma = ", ".to_string();
    let world = "world".to_string();
    let excl_point = '!';
    
    let mut dyn_str = "Hello".to_string();
    dyn_str += &comma;
    dyn_str.push_str(&world);
    dyn_str.push(excl_point);
    println!("{}", dyn_str);
}

/// Notice that to pass a dynamic string as an argument of push_str or +=, 
/// it has to be converted to a static string beforehand. 
/// 
/// Such effect is obtained using the & operator.
/// Actually, with such an operator, a reference to a String is obtained, 
///! but any reference to a String can be implicitly converted to a reference to str.
fn implicit_conv_ref_string(){
    let word = "bye".to_string();
    let w1: &str = &word;   // &String ==> &str 
    let w2: &String = &word; // &String === &String
    print!("{} {}", w1, w2);
    This will print: "bye bye".
}

//-------------------------------------------------//

/*

    Please check out the reference about them, and here are some general rules we can follow:

    Stick to String and &str for Unicode text.
    When working with filenames, use std::path::PathBuf and &Path instead.
    When working with binary data that isn’t UTF-8 encoded at all, use Vec<u8> and &[u8] instead.
    When working with environment variable names and command-line arguments in the native form presented by the operating system, use OsString and &OsStr.
    When interoperating with C libraries that use null-terminated strings, use std::ffi::CString and &CStr.

*/
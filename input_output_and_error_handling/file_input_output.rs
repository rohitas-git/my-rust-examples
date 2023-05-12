// In addition to reading and writing to the console, 
// -b in Rust it is rather easy also to read and write both binary and text sequential files.

fn main(){
    create_file();
    read_file();
    // edit_file();
    read_large_file();
}

/// * "create" function tries to create a file named "data.txt" in the current folder of the file system.
/// This function is fallible, and, if it is successful in creating a file, 
/// it returns a file handle to the file just created.
/// 
/// * "write_all" function to write some bytes in the newly create file.
/// 
fn create_file(){
    use std::io::Write;
    let mut file = std::fs::File::create("data.txt").unwrap(); // saves the file handle of newly created file
    file.write_all("\nRohitas is awesome!".as_bytes()).unwrap(); // writes the bytes into the file with its handle
}

/// * "open" function to open an existing file named "data. txt" in the current folder.
/// This function fails if the file does not exist, or if it is not accessible for whatever reason. 
/// If it succeeds, a file handle to such file is assigned to the "file" variable.
/// 
/// * "read_to_string" function on the "file" handle to read all the contents of that file 
/// * into a string variable, passed by reference to a mutable object.
/// 
/// So now you can copy a file into another one. 
/// -n But if a file is huge, it is impossible to load it all into a string before writing it. 
/// It is required to read and write a portion at a time. 
/// However, it is inefficient to read and write small portions.
/// 
fn read_file(){
    use std::io::Read;
    let mut file = std::fs::File::open("data.txt").unwrap();  // save file handle of opened file
    let mut contents = String::new();   
    file.read_to_string(&mut contents).unwrap();   
    print!("{}", contents);
}

fn edit_file(){
    use std::io::{Read, Write};
    let mut file = std::fs::File::open("data.txt").unwrap();
    file.write_all("\n Rohitas is awesome again !!".as_bytes()).unwrap();

    // let mut contents = String::new();   
    // file.read_to_string(&mut contents).unwrap();
    // print!("{}", contents);
}

/// -i Notice that there is no need to close explicitly the files. 
/// As soon as the file handles exit their scopes, the files are automatically closed, 
/// saving and releasing all internal temporary buffers.
/// 
fn read_large_file(){
    use std::io::Read;
    use std::io::Write;

    let mut command_line: std::env::Args = std::env::args(); 
    command_line.next().unwrap();

    let source = command_line.next().unwrap();
    let destination = command_line.next().unwrap();

    let mut file_in = std::fs::File::open(source).unwrap();
    let mut file_out = std::fs::File::create(destination).unwrap(); 
    // 4096-byte buffer is allocated in the stack.
    let mut buffer = [0u8; 4096];
    
    // a loop repeatedly reads a 4096-byte chunk from the source file and writes it to the output file. 
    // The number of bytes read is automatically as many as the length of the buffer. 
    // But if the remaining portion of the file is not long enough, 
    // the read bytes are less than 4096, or even zero.
    loop {
        let nbytes = file_in.read(&mut buffer).unwrap(); 
        file_out.write(&buffer[..nbytes]).unwrap();
        if nbytes < buffer.len() { break; }
    }

}
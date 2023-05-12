/* -------------------- Read lines of strings from a file ------------------- */
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

fn main() -> Result<(), Error> {
    let path = "lines.txt";

    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}

mod read_string_from_file{
    use std::fs::File;
    use std::io::{self, Read};
    
    fn main() -> io::Result<()> {
        let mut file = File::open("input.txt")?;
        let mut content = String::new();
    
        file.read_to_string(&mut content)?;
    
        println!("File content:\n{}", content);
    
        Ok(())
    }
}

/* --------------- Avoid writing and reading from a same file --------------- */
use same_file::Handle;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

fn main() -> Result<(), Error> {
    let path_to_read = Path::new("new.txt");

    let stdout_handle = Handle::stdout()?;
    let handle = Handle::from_path(path_to_read)?;

    if stdout_handle == handle {
        return Err(Error::new(
            ErrorKind::Other,
            "You are reading and writing to the same file",
        ));
    } else {
        let file = File::open(&path_to_read)?;
        let file = BufReader::new(file);
        for (num, line) in file.lines().enumerate() {
            println!("{} : {}", num, line?.to_uppercase());
        }
    }

    Ok(())
}

mod write_data_to_file{
    use std::fs::File;
    use std::io::Write;
    
    fn main() -> std::io::Result<()> {
        let mut file = File::create("output.txt")?;
        file.write_all(b"Hello, World!")?;
    
        Ok(())
    }
}

/* ---------------- Access a file randomly using a memory map --------------- */
// Creates a memory map of a file using memmap and simulates some non-sequential reads from the file. 
// Using a memory map means you just index into a slice rather than dealing with seek to navigate a File.

// The Mmap::map function assumes the file behind the memory map is not being modified at the same time 
// by another process or else a race condition occurs.




use memmap::Mmap;
use std::fs::File;
use std::io::{Write, Error};

fn main() -> Result<(), Error> {
    let file = File::open("content.txt")?;
    let map = unsafe { Mmap::map(&file)? };

    let random_indexes = [0, 1, 2, 19, 22, 10, 11, 29];
    assert_eq!(&map[3..13], b"hovercraft");
    let random_bytes: Vec<u8> = random_indexes.iter()
        .map(|&idx| map[idx])
        .collect();
    assert_eq!(&random_bytes[..], b"My loaf!");
    Ok(())
}

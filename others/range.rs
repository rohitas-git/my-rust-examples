fn main(){
    intro_range();
    pass_seq_to_fn::passing_v3();
}

///! Range:
/// 
/// start..end clause is an expression whose 
/// value can be assigned to a variable for use in for loop statements
/// Such value is called Range.
/// 
/// any range is a concretization of the Range<T> generic type,
/// where T must be an integer type able to represent both extremes of the range
/// 
/// Range struct contains only two fields: start, end
/// 
/// For loops: start (included), end (excluded)
/// 
/// range.len: end-start = no. of iterated values in loop
fn intro_range(){
    let range: std::ops::Range<usize> = 3..8;
    println!(
        "{:?}, {}, {}, {}",
        range, range.start, range.end, range.len()
    );
    for i in range { print!("{}, ", i); }
}

mod cases_for_range
{ 
    #[cfg(illegal)]
    fn _illegal_range(){
        let r1 = 3u8..12i8;
        let r2: std::ops::Range<u32> = -3..12;
        let r3: std::ops::Range<i32> = 3i16..12;
    }
    #[cfg(illegal)]
    fn _warning_range(){
        let _r1 = 3u8..1200; // generate an integer overflow warning for i8 and i32 variable
        let _r2 = 3..5_000_000_000;
    }
    #[cfg(illegal)]
    fn allowed_nonsense(){
        // such absurd ranges cannot be used in a for loop.
        let _r1 = false .. true;
        let _r2 = "hello" .. "world";
        let _r3 = 4.2 .. 7.9;
    }
}

pub mod pass_seq_to_fn{
    #[allow(dead_code)]
    fn passing_v1(){
        // Has many drawbacks: Duplication, Inflexible, Pass-by-value
        fn min(arr: [i32; 8]) -> i32 {
            let mut minimum = arr[0];
            for i in 1..arr.len() {
                if arr[i] < minimum { minimum = arr[i]; }
            }
            minimum
        }
        print!("{}", min([23, 17, 12, 16, 15, 28, 17, 30]));
    }
    // Pass-by-reference for argument
    #[allow(dead_code)]
    fn passing_v2(){
        fn min(arr: &[i32; 8]) -> i32 {
            let mut minimum = arr[0];
            for i in 1..arr.len() {
                if arr[i] < minimum { minimum = arr[i]; }
            }
            minimum
        }
        print!("{}", min(&[23, 17, 12, 16, 15, 28, 17, 30]));
    }
    //receive the request to process just a portion of the array.
    pub fn passing_v3(){
        fn min(arr: &[i32; 8], start: usize, count: usize) -> i32 {
            // Let's assume 'start' is between 0 and 7,
            // and 'count' is between 1 and 8 - start.
            let mut minimum = arr[start];
            for i in start + 1..start + count {
                if arr[i] < minimum { minimum = arr[i]; }
            }
            minimum
        }
        print!("{}", min(&[23, 17, 12, 16, 15, 28, 17, 30], 3, 2));
    }
    /*
    However, two drawbacks remain.
    Consider that our function needs to know only from which memory address to
    start processing, how many items to process, and which is the type of the items of
    the sequence. It is not required to know whether such sequence is a part of a larger
    sequence, and even less where such larger sequence starts and ends.
    In addition, consider that any vector keeps its data in heap-allocated array, and so
    such a function could process it, once it knew where are the items to process.
    */
    //! --> Solution: SLICES
}


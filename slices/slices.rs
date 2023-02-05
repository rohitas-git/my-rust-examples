fn main(){
    pass_seq_to_fn_with_slices::passing_with_slices();
    slicing::example_v1();
    slicing::mutable_slice();
    slicing::mutable_slice_refernce();
    slicing::open_end_ranges_and_slices();
}

// Followed from Ranges
/* 
    Consider that our function needs to know only from which memory address to
    start processing, how many items to process, and which is the type of the items of
    the sequence. 
    It is not required to know whether such sequence is a part of a larger
    sequence, and even less where such larger sequence starts and ends.
    In addition, consider that any vector keeps its data in heap-allocated array, and so
    such a function could process it, once it knew where are the items to process.
*/
/// * Slices Reference
/// 
/// This kind of type is a reference to a slice, or slice reference.
/// 
/// Its generic form is 
/// 
/// "&[T]",
/// where "T" represents any type that can be contained in an array.
/// 
/// “slice” means a sub-sequence of items inside a sequence of items, like an array or a vector buffer.
/// 
/// * Implementation of a slice reference is a pair of values: 
/// 
/// StartAddress:   the address of the first item of the sequence, 
/// Quanity:     the number of items.
/// 
/// Notice that usually we have variables whose type is “slice reference” and rarely “slice”.
/// 
/// * Slice
/// 
/// A slice would have type "[T]", but such type cannot be passed as argument to a function, 
/// as it has a size not defined at compilation time, 
/// and a requirement of the arguments of functions is that they have a compile-time defined size. 
/// Therefore, we can pass to a function only references to slices, not slices.
/// 
pub mod pass_seq_to_fn_with_slices{
    pub fn passing_with_slices(){
        // passing_v2 arr:&[i32;8] changed to &[i32]
        fn min(arr: &[i32]) -> i32 {
            // Let's assume 'arr' is not empty.
            let mut minimum = arr[0];
            for i in 1..arr.len() {
               if arr[i] < minimum { minimum = arr[i]; }
            }
            minimum
        }
        println!("{}", min(&[23, 17, 12, 16, 15, 28, 17, 30]));
    }
}

/// Actually, we already saw something very similar to slices and to slice references: 
/// strings buffers, and static strings.
/// 
/// * undefined-length sequence of bytes 
/// -String buffer: str 
/// -Slice of bytes: [u8]
/// 
/// * (address of beginning, length in bytes)
/// -Static string: &str
/// -Reference to slice of bytes: &[u8]
/// 
/// * (address of beginning, length in bytes, number of bytes used)
/// -Dynamic string: String
/// -Vector or bytes: Vec<u8>
/// 
mod similarties_str_and_slice{}

/// If a reference to an array is passed as argument:
/// 
/// In fact, such an array reference is implicitly converted into a slice reference, 
/// using the array address as slice address, 
/// and the array length as slice length.
/// 
mod implicit_conv_refarray_to_refslice{}

// Using slices, flexibility is much increased
fn final_passing(){
    fn min(arr: &[i32]) -> i32 {
        // Let's assume 'arr' is not empty.
        let mut minimum = arr[0];
        for i in 1..arr.len() {
            if arr[i] < minimum { minimum = arr[i]; }
        }
        minimum
    }
    print!("{} ", min(&[23, 17]));
    print!("{}", min(&vec![55, 22, 33, 44]));
}

/// * Slicing
///
/// forge a slice as a portion of an array or vector, 
/// not necessarily the entire array or vector.
/// 
/// * The operation of taking a slice from an array or a vector is named “slicing”.
/// Notice that, like in the for loop, the upper extreme of the range is excluded for slicing too.
/// 
pub mod slicing{
    pub fn passing_slicing_example_v1(){
        fn min(arr: &[i32]) -> i32 {
            // Let's assume 'arr' is not empty.
            let mut minimum = arr[0];
            for i in 1..arr.len() {
                if arr[i] < minimum { minimum = arr[i]; }
            }
            minimum
        }
        let arr = [23, 17, 12, 16, 15, 2];
        let range = 2..5;
        let slice_ref = &arr[range];
        println!("{}", min(slice_ref));

        // * ALT: print!("{} ", min(&[23, 17, 12, 16, 15, 2][2..5]));
    }

    pub fn example_v1() {
        let arr = [55, 22, 33, 44, 66, 7, 8];
        let v = vec![55, 22, 33, 44, 66, 7, 8];
        let sr1 = &arr[2..5];
        let sr2 = &v[2..5];
        println!("{:?} {:?} {:?} {:?}", sr1, sr2, &sr1[1..2], &sr1[1]);
    }

    pub fn out_of_range_slicing(){
        let arr = [55, 22, 33, 44, 66];
        let _r1 = 4..4; let _a1 = &arr[_r1];
        let _r2 = 4..3; //let _a2 = &arr[_r2];
        let _r3 = -3i32..2; //let _a3 = &arr[_r3];
        let _r4 = 3..8; //let _a4 = &arr[_r4];
    }

    pub fn mutable_slice(){
        // A slice is a portion of another sequence, and 
        // so changing its contents means changing the value of one or more items
        // in the underlying sequence

        let mut arr = [11, 22, 33, 44];
        {   
            //immutable ref to a mutable slice
            let sl_ref = &mut arr[1..3]; // mutable slice: &mut Arr[range]
            println!("{:?}", sl_ref);
            sl_ref[1] = 0;
            println!(" {:?}", sl_ref);
        }
        println!(" {:?}", arr);
    }
    pub fn mutable_slice_refernce(){
        let arr = [11, 22, 33, 44];
        {
            let mut sl_ref = &arr[1..3];
            println!("{:?}", sl_ref);
            sl_ref = &arr[0..1];
            println!(" {:?}", sl_ref);
        }
        println!(" {:?}", arr);
    }

    pub fn open_end_ranges_and_slices(){
        let r1: std::ops::RangeFrom<i32> = 3..;  // maybe used to specify for loop
        let r2: std::ops::RangeTo<i32> = ..12;
        println!("{:?} {:?} {} {}", r1, r2,
        std::mem::size_of_val(&r1),  //only store one i32 obj --> 4 bytes
        std::mem::size_of_val(&r2)); // same as above

        let arr = [11, 22, 33, 44];
        let n = 2;
        let sr1 = &arr[..n];
        let sr2 = &arr[n..];
        println!("RangeTo, RangeFrom{:?} {:?}", sr1, sr2);

        let range: std::ops::RangeFull = ..;
        let a1 = [11, 22, 33, 44];
        let a2 = &a1[range];
        println!("RangeFull:  {} {:?} {:?}", std::mem::size_of_val(&range), a1, a2);
    
    }

}
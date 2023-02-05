    /// * We saw several iterator generators: iter, iter_mut, chars, bytes; 
    /// * and we saw ranges, which are iterators with no need to be created by a generator.
    /// * We saw several iterator adapters: filter, map, enumerate.
    /// * And we saw several iterator consumers: any, all, count, sum, min, max, collect.

fn main(){
    // ANY
    iterator_consumers::any::determin_a_char_with_any("Hello World", 'o');
    iterator_consumers::any::determine_negative_with_any();
    // ALL
    iterator_consumers::all::determine_positive_with_all();
    // COUNT
    iterator_consumers::count::number_with_count();
    // SUM
    iterator_consumers::sum::ex_sum();
    iterator_consumers::sum::ex_sum2();
    iterator_consumers::sum::ex_sum3();
    // MIN-MAX 
    iterator_consumers::min_max_empty::numeric();
    iterator_consumers::min_max_empty::non_numeric();
    // COLLECT
    iterator_consumers::collect::collect_vecs();
    iterator_consumers::collect::collect_string_char_bytes();
    // CHAIN
    iterator_consumers::iterator_chain::chain1_v3();
}

/// * Iterator Consumer:
/// 
/// * Every function that is applied to an iterator but does not return an iterator is called “iterator consumer”, 
/// because it gets data from an iterator but does not put them into another iterator, 
/// and so it “consumes” data, instead of “adapting” data.
/// 
pub mod iterator_consumers{

    /// * Any:
    /// 
    /// The any function, which is in the standard library, is applied to an iterator. 
    /// * Its purpose is determining if a Boolean function (aka “predicate”) is true for any item produced by the iterator.
    /// 
    /// The any function must be applied to an iterator and must receive a closure as an argument. 
    /// It applies that closure to every item received from the iterator, 
    /// and it returns true as soon as the closure returns true on an item, 
    /// or returns false if the closure returns false for all the items.
    /// 
    pub mod any{
        fn determin_a_char(a_string:&str, req_char:char){
            // let s = "Hello, world!";
            // let ch = 'R';
            let mut contains = false;
            for a_char in a_string.chars() {if a_char == req_char {contains = true;}}
            print!("\"{}\" {} '{}'.", a_string, if contains {"contains"} else {"does not contain"}, req_char);
        }

        pub fn determin_a_char_with_any(a_string:&str, req_char:char){
            // let s = "Hello, world!";
            // let ch = 'R';
            println!("\"{}\" {} '{}'.", a_string, if a_string.chars().any(|a_char| a_char == req_char) {"contains"} else {"does not contain"}, req_char);
        }

        pub fn determine_negative_with_any(){
            print!("{} ", [45, 8, 2, 6].iter().any(|n: &i32| -> bool { *n < 0 }));
            println!("{} ", [45, 8, -2, 6].iter().any(|n: &i32| -> bool { *n < 0 }));
        }
    }
    /*
        Notice that while 
        * the any function means a repeated application of the or logical operator, 
        * the all function means a repeated application of the and logical operator.

        Notice also that, following logical rules, if the iterator produces no items, 
        the any function returns false for any closure, and the all function returns true for any closure.
    */
    pub mod all{
        pub fn determine_positive_with_all(){
            print!("{} ", [45, 8, 2, 6].iter().all(|n: &i32| -> bool { *n > 0 }));
            println!("{} ", [45, 8, -2, 6].iter().all(|n: &i32| -> bool { *n > 0 }));
        }
    }
    /// * The count iterator consumer does not get any argument, and it always returns a usize value.
    pub mod count{
        fn number_with_len(){
            // When Iterating for Array, Slice, Vector ... Use Len function of such objects
            // as it is the simplest and fastest way to get their lengths.
            let a_array = [10;8];
            println!("Array length: {}", a_array.len());

            let mut a_vec = vec![2;12];
            a_vec.push(11);
            println!("Vector length: {} and capacity: {} ", a_vec.len(), a_vec.capacity());

            let a_slice = &a_vec[2..6];
            println!("Slice length: {}", a_slice.len());
        }

        /// But if you want to know how many characters there are in a string, 
        /// you must scan it all, 
        /// because the number of chars comprising a string is not stored anywhere, unless you did it.
        pub fn number_with_count(){
            let s = "€èe"; 
            println!("char_count:{} string_len:{}", s.chars().count(), s.len());
        }
    }
    /// * sum: add all the items in iterator to get a single value
    /// 
    /// sum iterator consumer does not get arguments. 
    /// Yet, it requires a type parameter in angle brackets. ==> Type of returned number/value
    /// 
    /// * the sum function is applicable only to iterators that produce addable items. 
    /*
        Notice that while the count function was applicable to any iterator, 
        the sum function is applicable only to iterators that produce addable items. 
        * The statement [3.4].iter().sum::<f64>(); is valid, 
        * while the statement [true].iter().sum::<bool>() is illegal,
        because it is not allowed to sum Booleans.
    */
    pub mod sum{
        pub fn ex_sum(){
            println!("{}", [45, 8, -2, 6].iter().sum::<i32>());
        }
        pub fn ex_sum2(){
            let s: i32 = [45, 8, -2, 6].iter().sum();
            println!("{}", s);
        }
        pub fn ex_sum3(){
            let s: u32 = [].iter().sum();
            println!("{}", s);
        }
    }
    /// If an iterator produces values that can be compared with one another, 
    /// it is possible to get the minimum or the maximum of those values. But there is a problem: the empty sequences.
    /// 
    pub mod min_max_empty{
        pub fn numeric(){
            let arr = [45, 8, -2, 6]; 
            
            match arr.iter().min() {
                Some(arr_min) => println!("Array's min: {} ", arr_min),
                _ => (), 
            }

            match arr.iter().max() {
                Some(arr_max) => println!("Array's max: {} ", arr_max), 
                _ => (),
            }

            match [0; 0].iter().min() { 
                Some(value) => print!("{} ", value), 
                _ => println!("Array is empty: min/max iter_consumer: None"),
            }
        }

        pub fn non_numeric(){
            let arr = ["hello", "brave", "new", "world"]; 
            
            match arr.iter().min() {
                Some(n) => print!("{} ", n),
                _ => (), 
            }

            match arr.iter().max() {
                Some(n) => print!("{} ", n), _ => (),
            }  // will print "brave world"
        }
    }

    /// The any, all, count, sum, min, and max iterator consumers return 
    /// simple information regarding a possibly long sequence of items.
    /// 
    /// * The function is parameterized by the type of the resulting collection, 
    /// because 
    /// * such function can be used to put items in various kinds of collections, 
    /// and it wasn’t clear from the environment that a Vec was needed. 
    /// * Notice that the collect function cannot be used to put the iterated items into 
    /// a static string, an array, or a slice,
    /// * because it needs to allocate the needed space at runtime.
    pub mod collect{

        /// The collect function has created a new Vec<&i32> object, 
        /// and it has pushed into it all the items produced by the iterator.
        pub fn collect_vecs(){
            let arr = [36, 1, 15, 9, 4];
            let v = arr.iter().collect::<Vec<&i32>>(); 
            println!("{:?}", v);

            // In fact, Rust is able to infer the type &i32, 
            // and so it can be replaced by the placeholder _.
            let arr = [36, 1, 15, 9, 4];
            let v = arr.iter().collect::<Vec<_>>(); 
            println!("{:?}", v);

            let arr = [36, 1, 15, 9, 4];
            let v: Vec<_> = arr.iter().collect(); 
            println!("{:?}", v);
        }

        // Also, string characters or strings bytes can be collected into a string or into a vector:
        pub fn collect_string_char_bytes(){
            let s = "Hello";
            println!("Collect into String: {:?}", s.chars().collect::<String>()); 
            println!("Collect into Vec<char>: {:?}", s.chars().collect::<Vec<char>>()); 
            println!("Collect into Vec<u8>: {:?}", s.bytes().collect::<Vec<u8>>()); 
            println!("Collect into Vec<&u8>: {:?}", s.as_bytes().iter().collect::<Vec<&u8>>());
        }
    }

    /// This last version shows 
    /// * a programming pattern that is typical of functional languages: 
    /// * the iterator chain.
    /// 
    /// From a sequence, 
    /// an iterator is created, 
    /// then zero or more iterator adapters are chained, 
    /// then an iterator consumer closes the chain.
    /// 
    /// * Iterator Chain:
    /// * Iterator | Iterator_Generator -> 0 | 1+ Iterator_Adaptor -> 0|1+ Ierator Consumer 
    /// 
    /// * We saw several iterator generators: iter, iter_mut, chars, bytes; and we saw ranges, which are iterators with no need to be created by a generator.
    /// * We saw several iterator adapters: filter, map, enumerate.
    /// * And we saw several iterator consumers: any, all, count, sum, min, max, collect.
    pub mod iterator_chain{

        /// Assume you have an array of numbers, 
        /// and you want to create a vector containing only the positive numbers of such array, 
        /// multiplied by two.
        /// 
        /// 1. Without iterator
        pub fn chain1_v0(){
            let arr = [66, -8, 43, 19, 0, -31]; 
            let mut a_vec = vec![];

            for i in 0..arr.len() {
                if arr[i] > 0 { a_vec.push(arr[i] * 2); } 
            }    
        }
        /// 2. Use an iterator, without using iterator adapters:
        pub fn chain1_v1(){
            let arr = [66, -8, 43, 19, 0, -31]; 
            let mut a_vec = vec![];
            
            for item in arr.iter() {
                if *item > 0 { a_vec.push(*item * 2); } 
            }
            print!("{:?}", a_vec); 
        }
        /// 3. Use an iterator and two iterator adapters, without using an iterator consumer:
        pub fn chain1_v2(){
            let arr = [66, -8, 43, 19, 0, -31]; 
            let mut a_vec = vec![];

            for item in arr
                .iter()
                .filter(|x| **x > 0)
                .map(|x| *x * 2) 
            {
                a_vec.push(item);
            }
            
            print!("{:?}", a_vec);
        }
        /// 4. Use an iterator, two iterator adapters, and an iterator consumer:
        pub fn chain1_v3(){
            let arr = [66, -8, 43, 19, 0, -31]; 

            let a_vec = arr
                .iter()
                .filter(|x| **x > 0)
                .map(|x| *x * 2)
                .collect::<Vec<_>>(); 
            
            println!("{:?}", a_vec);
        }

    }
}
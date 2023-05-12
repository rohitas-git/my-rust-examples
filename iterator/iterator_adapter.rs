fn main(){
    iterator_adapters::filter::filter_negatives();

    iterator_adapters::map::print_doubles_with_map();
    
    iterator_adapters::enumerate::enumerate_seq_index_iter();
}

pub mod iterator_adapters{
    
    /// * Iterator Adapter - Filter :
    /// 
    /// The filter function is in the standard library.
    /// the purpose of this function is “filtering” the iterated sequence, that is,
    /// to discard the items that do not satisfy the criterion implemented by the closure, 
    /// and let pass only the items that satisfy such criterion
    /// 
    /// * The filter function gets an item at a time from the iterator, 
    /// * and invokes the closure once for every item, passing such item as argument. 
    ///   In our example, the current item, which is an integer number, is assigned to the x local variable.
    /// * The closure must return a Boolean that indicates 
    /// * whether the item is accepted (true) or rejected (false) by the filtering. 
    /// * The rejected items are destroyed, while the accepted ones are passed to the surrounding expression.
    ///
    ///  * In fact, the filter function returns an iterator that (when its next function is invoked) produces
    /// * just the items for which the closure returned true.
    /// 
    /// ! Filter function: takes an iterator and return a "transformed" iterator
    /// ! Such iterator "transformer" are called "iterator adapters".
    pub mod filter{
        fn filter_negatives_if_statement(){
            let arr = [66, -8, 43, 19, 0, -31]; 
            for n in arr.iter() {
                if *n < 0 { print!("{} ", n); } 
            }   
        }
        // Why ** required ?
        // -> iter fn produces refr to items of the sequence
        // -> filter fn passes to closure the reference to the item from iterator
        // thus, to access actual item, we req **item_ref_ref
        pub fn filter_negatives(){
            let arr = [66, -8, 43, 19, 0, -31];
            for n in arr.iter() {
                print!("{} ", n);
            }
            println!("");
            for n in arr.iter().filter(|x| **x < 0) {
                print!("{} ", n);
            }
            println!("");
        }
    } 
        
    /// * Iterator Adapter - Map:
    /// The map function is another iterator adapter in the standard library. 
    /// Its purpose is to “transform” the values produced by an iterator into other values.
    /// Differing from the filter function, the value returned by the closure can be of any type. 
    /// Such value represents the transformed value.
    /// 
    /// * Actually, the map function returns a newly created iterator that 
    /// produces all the items returned by the closure received as an argument.
    /// 
    /// While the filter adapter removes some items of the iterated sequence, and it keeps the others unchanged, 
    /// * the map adapter does not remove any items, but it transforms them. 
    /// 
    /// Another difference between them is that while 
    /// filter passes a reference as the argument of its closure,
    /// * map passes a value.
    /// 
    pub mod map{
        fn print_doubles_without_map(){
            let arr = [66, -8, 43, 19, 0, -31];
            for n in arr.iter() {
                print!("{} ", n * 2);
            }
        }

        pub fn print_doubles_with_map(){
            let arr = [66, -8, 43, 19, 0, -31];
            for n in arr.iter() {
                print!("{} ", n);
            }
            println!("");
            for n in arr.iter().map(|x| *x * 2) {
                print!("{} ", n);
            }
            println!("");
        }
    }
        

    /// * Iterator Adapter - enumerate:
    /// 
    /// The enumerate function takes an iterator and returns another iterator. 
    /// * This returned iterator, at each iteration, returns a value of type (usize, &char). 
    /// This tuple has at its first field a counter, 
    /// and as its second field a copy of the item received from the first iterator.
    pub mod enumerate{
        fn access_seq_using_index(){
            let arr = ['a', 'b', 'c'];
            for i in 0..arr.len() {
                print!("{} {}, ", i, arr[i]);
            }
        }

        fn access_seq_using_iter(){
            let arr = ['a', 'b', 'c'];
            for ch in arr.iter() {
                print!("{}, ", ch);
            }
        }

        fn scan_seq_index_iter(){
            let arr = ['a', 'b', 'c']; let mut i = 0;
            for ch in arr.iter() {
                print!("{} {}, ", i, *ch);
                i += 1; 
            }
        }
        
        pub fn enumerate_seq_index_iter(){
            let arr = ['a', 'b', 'c'];
            for (i, ch) in arr.iter().enumerate() {
                print!("{} {}, ", i, *ch);
            }
            println!("");
        }
    }

}

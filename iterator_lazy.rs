fn main(){
    lazy_iterators::active_iterator_adapter();
    lazy_iterators::lazy_iterator_adapter();
}

/// * In computer science, to be “lazy” means trying to do some processing as late as possible. 
/// * Iterator adapters are lazy, as they process data 
/// * only when another function asks them for an item: it can be 
/// another iterator adapter, 
/// or an iterator consumer, 
/// or a for loop, which acts as a consumer. 
/// * If there is no data sink, there is no data access.
pub mod lazy_iterators{

    /// * The runtime operations are the following ones.
    /// The invocation of iter prepares an iterator, but it does not access the array. Let’s name “I” such iterator.
    /// The invocation of filter prepares an iterator, but it does not manage data. Let’s name “F” such iterator.
    /// The invocation of map prepares an iterator, but it does not manage data. Let’s name “M” such iterator.
    /// 
    /// The invocation of collect asks “M” for an item; 
    /// “M” asks “F” for an item; 
    /// “F” asks “I” for an item; 
    /// “I” takes the number 66 from the array 
    /// and passes it to “F”, which prints it, checks whether it is positive, 
    /// and so passes it to “M”, which prints it, doubles it, 
    /// passes it to collect, which then pushes it to the vector.
    /// 
    /// Then, collect, because it has just received Some item and not None, asks “M” for another item, 
    /// and the trip is repeated until the number -8 arrives to “F”, which rejects it as non-positive. 
    /// Indeed -8 is not printed by “M”. 
    /// At this point, “F,” because before it has just received Some item, asks “I” for another item.
    /// The algorithm proceeds in this way until the array is finished. 
    /// When “I” cannot find other items in the array, it sends a None to “F” to indicate there are no more items. 
    /// When “F” receives a None, it sends it to “M”, 
    /// which sends it to collect, which stops asking items, and the whole statement is finished.
    pub fn active_iterator_adapter(){
        let v = [66, -8, 43, 19, 0, -31] 
            .iter()
            .filter(|x| { print!("F{} ", x); **x > 0 })
            .map(|x| { println!("M{} ", x); *x * 2 })
            .collect::<Vec<_>>();
        println!("{:?}", v);
    }

    /// * This does print nothing, because it does nothing. 
    /// * Even the compiler reports the warning unused `std::iter::Map` which must be used: 
    /// * iterator adapters are lazy and do nothing unless consumed.
    /// 
    /// * In computer science, to be “lazy” means trying to do some processing as late as possible. 
    /// 
    /// * Iterator adapters are lazy, as they process data only when another function asks them for an item: 
    /// * it can be another iterator adapter, or an iterator consumer, or a for loop, which acts as a consumer. 
    /// 
    /// ! If there is no data sink, there is no data access.
    pub fn lazy_iterator_adapter(){
        let v = [66, -8, 43, 19, 0, -31]
            .iter()
            .filter(|x| { print!("F{} ", x); **x > 0 })
            .map(|x| { print!("M{} ", x); *x * 2 });
        println!("{:?}", v);
    }
}
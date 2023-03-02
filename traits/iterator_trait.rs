// interesting standard library trait is "Iterator".
fn get_third(r: std::ops::Range<u32>) -> Option<u32> { 
    if r.len() >= 3 { Some(r.start + 2) } 
    else { None }
}

fn get_third(s: &[f64]) -> Option<f64> { 
    if s.len() >= 3 { Some(s[2]) } 
    else {None}
}


/// But using iterators,
/// it should be possible to write a generic function that, given an iterator, returns its third produced item, 
/// if it can produce at least three items, or nothing, if it cannot produce enough items.
/// 
// An iterator is every type that implements the "Iterator" standard library trait.
// It is enough to bound the function parameter type to the "Iterator" trait.
fn get_third<Iter, Item>(mut iterator: Iter) -> Option<Item> 
where Iter: std::iter::Iterator {

}
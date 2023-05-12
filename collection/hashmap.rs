/* -------------------------------------------------------------------------- */
/*                                   HashMap                                  */
/* -------------------------------------------------------------------------- */
// * https://doc.rust-lang.org/std/collections/struct.HashMap.html

// The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function, 
// which determines how it places these keys and values into memory.
// 
// *Use a HashMap when:
// You want to associate arbitrary keys with an arbitrary value.
// You want a cache.
// You want a map, with no extra functionality.
// 
// hash maps store their data on the heap.
// hash maps are homogeneous.
// each unique key can only have one value associated with it at a time (but not vice versa)
// 
// *By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables
// This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it.
// 
// Used for storing key-value pairs using a hashtable
// Only unique keys are allowed
// Hashmap doesn't guarantee the ordering of elements
// 
// The get method returns an Option<&V> or None if there is no value of that key
// The or_insert method returns a mutable reference (&mut V) to the value for the specified key.
// The return value of the entry method is an enum called Entry that represents a value that might or might not exist. 

mod access_values{
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
}

mod hashmap_ownership{
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}

mod UpdatingHashMaps{

    mod overwrite{
        // If we insert a key and a value into a hash map 
        // and then insert that same key with a different value, the value associated with that key will be replaced.

        use std::collections::HashMap;

        let mut scores = HashMap::new();
    
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
    
        println!("{:?}", scores); //25
    }

    mod add_key_only_if_not_present{
        // It’s common to check whether a particular key already exists in the hash map with a value 
        // then take the following actions: 
        // if the key does exist in the hash map, the existing value should remain the way it is. 
        // If the key doesn’t exist, insert it and a value for it.

        // Hash maps have a special API for this called entry that takes the key you want to check as a parameter. 
        // The return value of the entry method is an enum called Entry that represents a value that might or might not exist.

        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
    
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
    
        println!("{:?}", scores); // {"Yellow": 50, "Blue": 10}    
    }

    mod update_based_on_old_value{
        // Another common use case for hash maps is to look up a key’s value and then update it based on the old value. 
        use std::collections::HashMap;

        let text = "hello world wonderful world";
    
        let mut map = HashMap::new();
    
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
     
        println!("{:?}", map); // {"world": 2, "hello": 1, "wonderful": 1}
    }
}

/* --------------------------------- Example -------------------------------- */

mod book_review_map{
    use std::collections::HashMap;

    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<String, String>` in this example).
    let mut book_reviews = HashMap::new();
    
    // Review some books.
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );
    
    // Check for a specific one.
    // When collections store owned values (String), they can still be
    // queried using references (&str).
    if !book_reviews.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.",
                 book_reviews.len());
    }
    
    // oops, this review has a lot of spelling mistakes, let's delete it.
    book_reviews.remove("The Adventures of Sherlock Holmes");
    
    // Look up the values associated with some keys.
    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{book}: {review}"),
            None => println!("{book} is unreviewed.")
        }
    }
    
    // Look up the value for a key (will panic if the key is not found).
    println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);
    
    // Iterate over everything.
    for (book, review) in &book_reviews {
        println!("{book}: \"{review}\"");
    }
}

mod from_array{
    // A HashMap with a known list of items can be initialized from an array:

    use std::collections::HashMap;

    let solar_distance = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);
}

mod entry_api{
    #![allow(unused)]
    fn main() {
        use std::collections::HashMap;
        
        let mut letters = HashMap::new();
        
        for ch in "a short treatise on fungi".chars() {
            letters.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
        }
        
        assert_eq!(letters[&'s'], 2);
        assert_eq!(letters[&'t'], 3);
        assert_eq!(letters[&'u'], 1);
        assert_eq!(letters.get(&'y'), None);
        
        println!("{:?}", letters);
    }
}

mod with_custom_keys{
    // * The easiest way to use HashMap with custom key type is to derive Eq and Hash. We must also derive PartialEq.

    use std::collections::HashMap;

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Viking {
        name: String,
        country: String,
    }
    
    impl Viking {
        /// Creates a new Viking.
        fn new(name: &str, country: &str) -> Viking {
            Viking { name: name.to_string(), country: country.to_string() }
        }
    }
    
    // Use a HashMap to store the vikings' health points.
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);
    
    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{viking:?} has {health} hp");
    }
}

mod snippets{
    {
        use std::collections::HashMap;
        use std::collections::hash_map::RandomState;
        
        let s = RandomState::new();
        let mut map = HashMap::with_hasher(s);
        map.insert(1, 2);
    }
    {
        use std::collections::HashMap;
        let map: HashMap<i32, i32> = HashMap::with_capacity(100);
        assert!(map.capacity() >= 100);
    }
    {
        use std::collections::HashMap;
        let mut map: HashMap<&str, i32> = HashMap::with_capacity(10);
    }
    {
        use std::collections::HashMap;
        use std::collections::hash_map::RandomState;
        
        let s = RandomState::new();
        let mut map = HashMap::with_capacity_and_hasher(10, s);
        map.insert(1, 2);
    }
    {
        use std::collections::HashMap;

        let map = HashMap::from([
            ("a", 1),
            ("b", 2),
            ("c", 3),
        ]);
        
        for key in map.keys() {
            println!("{key}");
        }
        // Performance
        // In the current implementation, iterating over keys takes O(capacity) time instead of O(len) because it internally visits empty buckets too.
    }
    {
        use std::collections::HashMap;

        let map = HashMap::from([
            ("a", 1),
            ("b", 2),
            ("c", 3),
        ]);
        
        let mut vec: Vec<&str> = map.into_keys().collect();
        // The `IntoKeys` iterator produces keys in arbitrary order, so the
        // keys must be sorted to test them against a sorted array.
        vec.sort_unstable();
        assert_eq!(vec, ["a", "b", "c"]);
    }
    {
        use std::collections::HashMap;

        let map = HashMap::from([
            ("a", 1),
            ("b", 2),
            ("c", 3),
        ]);
        
        for val in map.values() {
            println!("{val}");
        }
    }
    mod iter{
        use std::collections::HashMap;

        let map = HashMap::from([
            ("a", 1),
            ("b", 2),
            ("c", 3),
        ]);
        
        for (key, val) in map.iter() {
            println!("key: {key} val: {val}");
        }
    }
    mod drain{
        use std::collections::HashMap;

        let mut a = HashMap::new();
        a.insert(1, "a");
        a.insert(2, "b");

        for (k, v) in a.drain().take(1) {
            assert!(k == 1 || k == 2);
            assert!(v == "a" || v == "b");
        }

        assert!(a.is_empty());
    }


}
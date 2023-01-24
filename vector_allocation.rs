// Two object structures in case of vector
// 1. Fixed size header -- Stack allocated
// 2. Variable length buffer -- Heap allocated

// Vec obj stores 3 sub-ojects in its header
// 1. pointer-to heap-allocated buffer
// 2. capacity of such buffer as number of items, which is a usize nubmber
// 3. length of vector as number of items in-store <= capacity

// Therefore, Header = 
// 3 * 8 -> 24 bytes in any 64-bit system
// 3 * 4 -> 12 bytes in any 32-bit system

// After length exceeds capacity, vector needs to deallocate current buffer and allocate & copy into a new buffer
// with more capacity 

fn len_vs_capacity(){
    println!("Compare Len vs Capacity");
    let mut v = vec![0; 0];
    println!("{} {}", v.len(), v.capacity());
    v.push(11);
    println!("{} {}", v.len(), v.capacity());
    v.push(22);
    println!("{} {}", v.len(), v.capacity());
    v.push(33);
    println!("{} {}", v.len(), v.capacity());
    v.push(44);
    println!("{} {}", v.len(), v.capacity());
    v.push(55);
    println!("{} {} \n", v.len(), v.capacity());
}

fn capacity_change(){
    println!("Capacity Changes");
    let mut v = vec![0; 0];
    let mut prev_capacity = std::usize::MAX;
    for i in 0..1_000 {
        let cap = v.capacity();
        if cap != prev_capacity {
                println!("index:{}   len:{}   cap:{}", i, v.len(), cap);
                prev_capacity = cap;
            }
            v.push(1);    
    }
}

fn main(){
    len_vs_capacity();

    capacity_change();
}
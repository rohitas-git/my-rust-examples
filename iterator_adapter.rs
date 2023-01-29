

fn main(){


}

pub mod iterator_adapters{
    fn filter_negatives_if_statement(){
        let arr = [66, -8, 43, 19, 0, -31]; 
        for n in arr.iter() {
            if *n < 0 { print!("{} ", n); } 
        }   
    }
    fn filter_negatives_iter_filter(){
        let arr = [66, -8, 43, 19, 0, -31];
        for n in arr.iter().filter(|x| **x < 0) {
            print!("{} ", n);
        }
    }

}

pub mod iterator_consumers{
}

pub mod lazy_processing_iterator_chains{
}

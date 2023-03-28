/* -------------------------------------------------------------------------- */
/*                                 repetitions                                */
/* -------------------------------------------------------------------------- */

/// () => {} is the entry for a macro rule. 
/// We can have many rules to match for in a single macro.
/// 
/// $(),* 
/// , - Separator [This will be the character that will separate the patterns, allowing us to have repetitions.]
/// * will repeat pattern inside $()

macro_rules! yo{
    ( $( $name:expr ),* ) => {
        $( println!("Yo {}!", $name); )*
    };
}

fn main(){
    yo!["John","Finn","Santa"];
}
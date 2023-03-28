/// Putting pattern inside $(),* allows repetitions
/// Code in $()* will be expanded in Transcriber for each match

use std::collections::HashMap;

macro_rules! map {

    ( $($key:expr => $value:expr),* ) =>
    {{
        let mut hm = HashMap::new();
        $( hm.insert($key, $value); )*
        hm
    }}

}

fn main(){

    let user = map!(
    "name" => "Tom",
    "gender" => "Boy"
    );

    println!("User {:?}", user);
}
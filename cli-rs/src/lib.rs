use node_bindgen::derive::node_bindgen;
use tslink::tslink;


/// add two integer
#[tslink]
#[node_bindgen]
fn sum(first: i32, second: i32) -> i32 {
    println!("👋🏼 Hi");
    first + second
}
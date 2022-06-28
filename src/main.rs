mod lang;

use serde::{Serialize};

#[derive(Serialize)]
struct Context {
    name: String,
}

fn main() {
    println!("hello world!");
}

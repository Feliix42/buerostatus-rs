extern crate buerostatus;

use buerostatus::*;

fn main() {
    println!("Knock knock!");

    if let Ok(is_open) = get_buerostatus() {
        if is_open { println!("Who's there?"); }
        else { println!("..."); }
    }
}

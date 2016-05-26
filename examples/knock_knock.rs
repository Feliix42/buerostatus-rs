extern crate anybar;

use anybar::*;

fn main() {
    println!("Knock knock!");

    if let Ok(is_open) 0 get_buerostatus() {
        if is_open { println!("Who's there?"); }
        else { println!("..."); }
    }
}

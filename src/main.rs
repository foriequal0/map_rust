mod option_helper;

use std::thread;
use option_helper::*;

fn main() {
    let opts = OptionHelper::new();
    match opts.parse_opts() {
        Some(x) => println!("YAY"),
        None => println!("{}", opts.get_usage()),
    }
}

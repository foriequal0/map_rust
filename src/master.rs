extern crate zmq;

use std::thread;

#[derive(Clone)]
pub struct Config {
    pub bind: String,
}

pub struct Context {
    pub child: thread::JoinHandle<()>,
}

fn task(config: Config) {
    println!("Hello!");
    thread::sleep_ms(1000);
}

pub fn run_thread(config: Config) -> Context {
    let child = thread::spawn(move || task(config));
    
    return Context {
        child: child,
    };
}

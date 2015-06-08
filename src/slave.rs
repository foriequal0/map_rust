extern crate zmq;

use std::thread;

#[derive(Clone)]
pub struct Config {
    pub connect: String,
    pub workers: u32,
}

pub struct Context {
    pub childs: Vec<thread::JoinHandle<()>>,
}

fn task(config: Config) {
    println!("World!");
    thread::sleep_ms(1000);
}

pub fn run_thread(config: Config) -> Context {
    let childs: Vec<_> = (0..config.workers)
        .map(|_| {
            let x = config.clone();
            thread::spawn(move || task(x))
        })
        .collect();

    return Context {
        childs: childs,
    };
}

extern crate zmq;

use std::thread;
use std::string::ToString;

#[derive(Clone)]
pub struct Config {
    pub id: u32,
    pub connect: String,
    pub workers: u32,
}

pub struct Context {
    pub childs: Vec<thread::JoinHandle<()>>,
}

fn task(config: Config) {
    let mut context = zmq::Context::new();
    //TODO : Error handling
    let mut requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect(&config.connect).is_ok());

    let mut msg = zmq::Message::new().unwrap();

    let str = config.id.to_string().into_bytes();
    requester.send(&str, 0);
    
}

pub fn run_thread(config: Config) -> Context {
    let childs: Vec<_> = (0..config.workers)
        .map(|id| {
            let x = {
                let mut tmp = config.clone();
                tmp.id = id;
                tmp
            };
            thread::spawn(move || task(x))
        })
        .collect();

    return Context {
        childs: childs,
    };
}

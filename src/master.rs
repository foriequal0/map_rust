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
    let mut context = zmq::Context::new();
    // TODO: Error handling.
    let mut responder = context.socket(zmq::REP).unwrap();

    responder.bind(&config.bind).is_ok();

    let mut msg = zmq::Message::new().unwrap();
    let mut counter = 0;
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Check {}", msg.as_str().unwrap());
        responder.send_str("World", 0).unwrap();
        counter += 1;
        if(counter == 4) {
            break;
        }
    }
}

pub fn run_thread(config: Config) -> Context {
    let child = thread::spawn(move || task(config));
    
    return Context {
        child: child,
    };
}

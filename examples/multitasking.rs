extern crate map_rust;

use map_rust::*;
use map_rust::join_all::*;

fn main() {
    let master_config = master::Config {
        bind: "tcp://*:5555".to_string(),
    };

    let slave_config = slave::Config {
        connect: "tcp://localhost:5555".to_string(),
        workers: 4,
        id: 0,
    };
    
    let master = master::run_thread(master_config);
    let slave = slave::run_thread(slave_config);

    master.child.join().unwrap();
    slave.childs.join_all();
}

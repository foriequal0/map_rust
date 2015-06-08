mod option_helper;
mod master;
mod slave;

use std::thread;
use option_helper::*;

trait JoinAll {
    fn join_all(self);
}

impl JoinAll for Vec<thread::JoinHandle<()>> {
    fn join_all(self) {
        for v in self {
            v.join()
        }
    }
}

fn main() {
    let master_config = master::Config {
        bind: "a".to_string(),
    };

    let slave_config = slave::Config {
        connect: "b".to_string(),
        workers: 4,
    };
    
    let master = master::run_thread(master_config);
    let slave = slave::run_thread(slave_config);

    master.child.join().unwrap();
    slave.childs.join_all();
}

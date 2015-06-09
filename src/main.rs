mod option_helper;
mod master;
mod slave;

use std::thread;
use option_helper::*;

trait JoinAll<T> {
    fn join_all(self) -> Vec<thread::Result<T>>;
}

impl<T, C: IntoIterator> JoinAll<T> for C
    where C::Item: Into<thread::JoinHandle<T>> {
    fn join_all(self) -> Vec<thread::Result<T>>{
        let all:Vec<_> = self.into_iter()
            .map(move |v| v.into().join())
            .collect();
        return all;
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

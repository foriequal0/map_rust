extern crate getopts;

use std::env;
use self::getopts::{Options, Fail};
use std::option::Option;
use std::result::Result;
use std::ffi::{OsStr, OsString};
use std::convert::AsRef;

#[derive(Eq, PartialEq)]
pub enum ExecOption {
    Master { bind_to: String },
    Slave { connect_to: String },
    MultiThread { slaves: u32 },
    Help,
}

fn build_opts() -> Options {
    let mut opts = Options::new();
    
    opts.optopt("m", "master", "run as a master node", "BIND");
    opts.optopt("s", "slave", "run as a slave node", "CONNECT");
    opts.optopt("l", "local", "run both a master and slaves in a local machine", "SLAVES");
    opts.optflag("h", "help", "print this help menu");

    opts
}

macro_rules! try_o {
    ($expr:expr) => (match $expr {
        Some(val) => val,
        None => {
            return None;
        }
    })
}

pub fn try_parse_opts<C: IntoIterator>(args: C) -> Option<ExecOption>
    where C::Item: AsRef<str>
{
    let opts = build_opts();

    let args_string:Vec<_> = args.into_iter().map(|s| s.as_ref().to_string()).collect();
    let args_osstr:Vec<_> = args_string.iter().map(|s| OsString::from(s)).collect();

    let matches = try_o!(opts.parse(args_osstr).ok());
    if matches.opt_present("h") {
        return Some(ExecOption::Help);
    }

    let master_exist = matches.opt_present("m");
    let slave_exist = matches.opt_present("s");

    if matches.opt_present("l") {
        if master_exist || slave_exist {
            return None;
        }
        
        let slaves = try_o!(matches.opt_str("l"));
        let slaves_num:u32 = try_o!(slaves.parse().ok());
        
        return Some(ExecOption::MultiThread {
            slaves: slaves_num,
        });
    } else {

        if master_exist && slave_exist {
            return None;
        } 
        
        if master_exist {
            let bind = try_o!(matches.opt_str("m"));
            return Some(ExecOption::Master {
                bind_to: bind,
            });
        } else if slave_exist {
            let connect = try_o!(matches.opt_str("s"));
            return Some(ExecOption::Slave {
                connect_to: connect,
            });
        }

        return None;
    }
}

pub fn get_usage(program: &str, opts: Options) -> String {
    let brief = format!("Usage: {} [options]", program);
    format!("{}", opts.usage(&brief))
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn parse_should_correct() {
        assert!(try_parse_opts(&["-h"])
                == Some(ExecOption::Help));
        assert!(try_parse_opts(&["-l", "3"])
                == Some(ExecOption::MultiThread { slaves: 3 }));
        assert!(try_parse_opts(&["-m", "tcp://*:8888"])
                == Some(ExecOption::Master { bind_to: "tcp://*:8888".to_string() }));
        assert!(try_parse_opts(&["-s", "tcp://localhost:8888"])
                == Some(ExecOption::Slave { connect_to: "tcp://localhost:8888".to_string() }));
    }

    #[test]
    fn parse_should_fail() {
        assert!(try_parse_opts(&["-l"]) == None); 
        assert!(try_parse_opts(&["-l", "2", "-m", "tcp://"]) == None);
    }
}

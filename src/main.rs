extern crate itertools;

use std::string::String;
use std::borrow::Borrow;
use itertools::Itertools;

trait Map {
    fn map(&self, record: &str) -> Vec<(String, String)>;
}

trait Reduce {
    fn reduce(&self, key: &str, records: &[&str]) -> Result<String, String>;
}

struct MapReduce<'a> {
    map: &'a Map,
    reduce: &'a Reduce,
}

impl<'a> MapReduce<'a> {
    fn run(&self, records: &[&str]) -> Vec<(String, String)> {
        let mapped = records.iter()
            .flat_map(|r| self.map.map(r));

        let sorted = {
            let mut mapped_vector: Vec<_> = mapped.collect();
            mapped_vector.sort_by(|x, y| x.0.cmp(&y.0));
            mapped_vector
        };

        let grouped = sorted.iter().group_by(|x| &x.0);

        let reduced = grouped
            .map(|key| {
                let (key, kvlist) = key;
                let values: Vec<_> = kvlist.iter().map(|x| x.1.borrow()).collect();
                let res = self.reduce.reduce(&key, &values);
                (key, res)
            })
            .filter(|x| if let &(_,Ok(_)) = x { true } else { false })
            .map(|x| match x {
                (key, Ok(res)) => return (key.to_string(), res),
                _ => (String::new(), String::new()),
            });
        return reduced.collect();
    }
}

struct MapTask;

impl Map for MapTask {
    fn map(&self, record: &str) -> Vec<(String, String)>{
        let words = record.split(' ');
        
        return words.map(|word| (word.to_string(), "1".to_string())).collect();
    }
}

struct ReduceTask;

impl Reduce for ReduceTask {
    fn reduce(&self, key: &str, records: &[&str]) -> Result<String, String> {
        return Ok(records.iter().count().to_string());
    }
}

fn main() {
    let map = MapTask;
    let reduce = ReduceTask;

    let map_reduce = MapReduce {
        map: &map,
        reduce: &reduce
    };

    println!("{:?}", map_reduce.run(&["this is word", "this is long word"]));
}

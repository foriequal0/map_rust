use std::thread;

pub trait JoinAll<T> {
    fn join_all(self) -> Vec<thread::Result<T>>;
}

impl<T, C: IntoIterator> JoinAll<T> for C
    where C::Item: Into<thread::JoinHandle<T>>
{
    fn join_all(self) -> Vec<thread::Result<T>> {
        let all:Vec<_> = self.into_iter()
            .map(move |th| th.into().join())
            .collect();
        return all;
    }
}

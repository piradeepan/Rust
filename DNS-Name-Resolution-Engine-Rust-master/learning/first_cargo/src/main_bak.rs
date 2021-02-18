//use std::any::type_name;
use std::thread;
use std::time::Duration;
use std::any::type_name;
use crossbeam::channel::bounded;
//use std::sync::{Arc, Mutex};
use crossbeam::queue::ArrayQueue;
// accept an int32 value
// from a channel

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let q = ArrayQueue::new(20);
    assert_eq!(q.push(10), Ok(()));
    assert_eq!(q.push(20), Ok(()));
    assert_eq!(q.push(30), Ok(()));
    assert_eq!(q.push(40), Ok(()));
    assert_eq!(q.push(50), Ok(()));
    assert_eq!(q.push(60), Ok(()));
    
    let temp_q = q.pop();
    // println!("{?}", q.pop());
    // println!("{:#?}", q.pop());
    // println!("{:#?}", q.pop());
    let mut threads = vec![];
    
    let (s, r) = bounded(3);

    threads.push(thread::spawn(move || {
        s.send(1).unwrap();
        s.send(2).unwrap();
        s.send(3).unwrap();
        drop(s); // Disconnect the channel.
    }));
    
    let v: Vec<_> = r.iter().collect();

    assert_eq!(v, [1, 2, 3]);

    println!("{}", type_of(threads));
    thread::sleep(Duration::from_secs(1));
    assert_eq!(r.recv(), Ok(1));
  

}
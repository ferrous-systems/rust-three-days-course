// TODO: Make this example easier and shorter

use futures::channel::oneshot;
use std::sync::mpsc;

let (tx1, rx1) = oneshot::channel::<i32>();
let (tx2, rx2) = mpsc::channel();

let t = thread::spawn(|| tx1.send(1).unwrap());
rx1.map_ok(move |x| tx2.send(x)).run_in_background();

assert_eq!(1, rx2.recv().unwrap());
t.join().unwrap();
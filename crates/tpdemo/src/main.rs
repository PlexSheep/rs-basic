use std::sync::{Arc, Mutex};

use threadpool::ThreadPool;

struct Fish(i32);

fn main() {
    let thing: Arc<Mutex<Fish>> = Arc::new(Mutex::new(Fish(0)));

    let tp = ThreadPool::new(20);

    while thing.lock().unwrap().0 < 200 {
        let thing = thing.clone();
        tp.execute(move || {
            do_thing(thing);
        });
    }
}

fn do_thing(thing: Arc<Mutex<Fish>>) {
    println!("blubb");
    #[allow(deprecated)]
    std::thread::sleep_ms(1000);
    thing.lock().unwrap().0 += 1;
}

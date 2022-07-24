use futures::executor::block_on;

use std::thread;
use std::time;

pub fn futures_launch() {
    block_on(async {
        loop {
            println!("futures_launch======>");
            thread::sleep(time::Duration::from_millis(1000))
        }
    });
}

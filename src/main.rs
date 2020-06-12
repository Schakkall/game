//rustc -o hello.rs hello

use std::thread;
use std::time::Duration;

fn test(i: u64) {
	thread::sleep(Duration::from_millis(i * 123));   
	println!("It is the thread! {}", i); 
	   
}

fn main() {
    thread::spawn(|| {	
		test(10);
	});

    thread::spawn(|| {
		test(20);		
	});

    thread::spawn(|| {
		test(30);
	});

	thread::sleep(Duration::from_millis(10000));
}
// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish
use std::thread;
use std::thread::spawn;

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1100));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1200));
    "!"
}

fn main() {
    let hello_handle = thread::spawn(|| msg_hello());

    let thread_handle = thread::spawn(|| msg_thread());

    let excited_handle = thread::spawn(|| msg_excited());

    match hello_handle.join() {
        Ok(data) => println!("{}", data),
        Err(_) => ()
    }

    match thread_handle.join() {
        Ok(data) => println!("{}", data),
        Err(_) => ()
    }

    match excited_handle.join() {
        Ok(data) => println!("{}", data),
        Err(_) => ()
    }
}

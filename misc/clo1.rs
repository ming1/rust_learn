use std::thread;

fn create_thread(f: fn()) {
    let c = move || {
        f();
    };

    let handle = thread::spawn(c);

    // Wait for the thread to finish execution
    handle.join().unwrap();

    let handle = thread::spawn(c);

    // Wait for the thread to finish execution
    handle.join().unwrap();
}

fn fn_ptr() {
    println!("{}", "Hello from fn callback");
}

fn main() {
    // Define a variable
    let message = "Hello from the main thread!";

    // Define a closure that captures the 'message' variable
    let closure = move || {
        println!("{}", message);
    };

    create_thread(fn_ptr);
}

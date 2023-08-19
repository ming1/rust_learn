use std::thread;

fn main() {
    // Define a closure
    let add_five = |x| x + 5;

    // Spawn a new thread and pass the closure to it
    let handle = thread::spawn(move || {
        // Inside the new thread, we can use the passed closure
        let result = add_five(10);
        println!("Result in thread: {}", result);
    });

    // Spawn a new thread and pass the closure to it
    let handle1 = thread::spawn(move || {
        // Inside the new thread, we can use the passed closure
        let result = add_five(10);
        println!("Result in thread: {}", result);
    });

    handle.join().unwrap();
    handle1.join().unwrap();

    // Note: The closure 'add_five' goes out of scope here and is dropped.
    // Any captured variables in the closure are also dropped as they are moved to the new thread.
}

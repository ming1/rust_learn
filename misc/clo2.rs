// Function that takes a closure of the defined type as an argument
fn execute_closure(closure: dyn Fn(i32) + Send + Sync + 'static) {
    let handle = std::thread::spawn(closure(5));

    // Wait for the thread to finish execution
    handle.join().unwrap();
}

fn main() {
    // Define a closure that matches the closure type
    let my_closure = |x| println!("{}", x + 5);

    // Call the function and pass the closure as an argument
    execute_closure(my_closure);
}

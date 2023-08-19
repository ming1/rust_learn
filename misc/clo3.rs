use std::thread;

fn run_in_thread<F>(closure: F)
where
    F: Fn() + Send + Sync + 'static,
{
    for _i in 0..2 {
        let c = move || closure();
        thread::spawn(move || {
            c();
        });
    }
}

fn main() {
    // Create a closure
    let my_closure = || {
        for i in 0..5 {
            println!("Thread: {}", i);
        }
    };

    // Pass the closure to the run_in_thread function
    run_in_thread(my_closure);

    // The main thread continues its execution here
    println!("Main thread continues...");
}

use std::thread;

fn run_in_threads<F>(closure: F, num_threads: usize)
where
    F: FnMut() + Send + Sync + Clone + 'static,
{
    //    let handles: Vec<_> = (0..num_threads)
    //        .map(|_| {
    //            let closure_clone = closure.clone(); // Clone the closure for each thread
    //            thread::spawn(move || {
    //                closure_clone();
    //            })
    //        })
    //        .collect();

    let mut handles = Vec::new();
    for q in 0..num_threads {
        let closure_clone = closure.clone(); // Clone the closure for each thread
        handles.push(std::thread::spawn(move || {
            closure_clone();
        }));
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    // Create a closure
    let my_closure = || {
        println!("Thread {:?}", thread::current().id());
    };

    // Specify the number of threads to spawn
    let num_threads = 3;

    // Pass the closure and number of threads to the function
    run_in_threads(my_closure, num_threads);

    // The main thread continues its execution here
    println!("Main thread continues...");
}

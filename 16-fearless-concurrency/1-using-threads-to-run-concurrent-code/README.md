# Using threads to run code simultaneously

A Rust program is run as a `process` by our Operating System. Within this program, we can have **parts that run independently and/or simultaneously**. Each of these running parts is called a `thread`. Splitting our program among multiple threads can improve performance, but also, we need to deal with some problems. We don’t have any guarantee about the order in which these threads will run and finish. Problems -

- `Race conditions` - where threads are accessing a resource in an inconsistent order leading to an unpredictable outcome.
- `Deadlocks` - Multiple threads stuck waiting for each other to finish. Due to this, all those threads are blocked and cannot continue execution.

Creating a new thread -
```rust
thread::spawn(
	// passing a closure, containing the code to execute in this new thread
	| | {
		// code that will run in the worker thread
	}
)
```

> When the main thread is finished executing, all the other threads created by the program are forcefully shutdown, regardless of whether they finished executing or not.

We can `prevent premature shutdown of child threads` in this manner -
```rust
fn main( ) {
    let workerThread= thread::spawn(
        | | {
            //* code that will run in this worker thread */
        }
    );

    //* code that will run in the main thread */

	// this will block the main thread until the worker thread finishes execution
    workerThread.join( )
        .unwrap( );
}
```

## Using move closures with threads

If the worker thread wants to use a value from the parent thread, we need to add the `move` keyword with the closure being passed as the argument. This **moves the ownership of that value from the parent thread to the worker thread**. Here is an example -
```rust
fn main( ) {
    let vector= vec![ 1,2,3 ];

    let workerThread= thread::spawn(
        move | | {
            // ownership of `vector` moved to the worker thread
            println!("the vector is - {:?}", vector);
        }
    );

    // this will block the main thread until the worker thread finishes execution
    workerThread.join( )
        .unwrap( );
}
```

We need to move the ownership of *vector* from the main to the worker thread, because in this way, *vector* **cannot be dropped by the main thread while the worker thread is still using it**.
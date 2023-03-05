# Shared state concurrency
Previously, we saw how multiple threads can communicate with each other using channels. There is another way - using shared data.
Channels in Rust are based on `single ownership model`. First the value is **owned by the worker-thread, then the ownership is transferred to the channel and then to the consumer thread**.
Shared memory concurrency is based on the `multiple ownership model`.

## Mutexes
Mutex stands for `mutual exclusion`. A mutex allows only one thread to access a value at a given point of time (It is similar to `AtomicReference` in Java). To access the data in a mutex, a **thread must first signal that it wants access** by asking to acquire the mutex’s lock. The `lock` is a data structure that is part of the mutex that **keeps track of who currently has exclusive access** to the data.
Mutexes are more difficult to use, since we need to remember 2 rules -
- We must acquire the lock before using the data
- The must release the lock when we are done using the data, so that other threads can access that data.

But Rust's type system and memory management rules, enforces us to follow these 2 rules.
Here is how it happens -
```rust
let mutex= Mutex::new(
    5 // this is the data that the mutex holds
);

//* `lock` can return error if the mutex is already locked */
// we need to call `lock`, otherwise we won't get reference to the shared data
let mut reference= mutex.lock( )
    .unwrap( );
*reference= 6;

println!("modified value of the shared data - {:?}", mutex);
```
The call to lock returns a `smart pointer` called `MutexGuard`, **wrapped in a LockResult** that we handled with the call to unwrap. The MutexGuard smart pointer implements Deref to point at our inner data; the smart pointer also has a **Drop implementation that releases the lock automatically when a MutexGuard goes out of scope**.

Example of using mutex to share data between multiple threads -
```rust
fn main( ) {
    let counterMutex= Mutex::new(0);
    let mut workerThreads= vec![ ];

    for _ in 0..10 {
        let workerThread= thread::spawn(
            move | | {
                let mut counterReference= counterMutex.lock( )
                    .unwrap( );

                *counterReference += 1;
            }
        );
        workerThreads.push(workerThread);
    }

    for workerThread in workerThreads {
        workerThread.join( )
            .unwrap( );
    }

    let finalCounterValue= *counterMutex.lock( )
        .unwrap( );
    println!("final value of the counter - {}", finalCounterValue);
}
```
But wait, this code won't work. It will give error - in the first iteration of the for loop, the ownership of `counterMutex` was moved to the first worker thread spawned. So, how do we solve this problem?

By using `Rc` - smart pointer allowing multiple ownership of a value? Unfortunately, Rc is not safe to share across threads. When Rc manages the reference count, it adds to the count for each call to clone and subtracts from the count when each clone is dropped. It **doesn't take any measures to prevent concurrency related problems like race conditions**.

Here comes `Arc<T>` - which prevents concurrency related problems from happening.
```rust
fn main( ) {
    let counterMutex=
        Arc::new( // allows multiple ownership of the mutex
            Mutex::new(0)
        );
    let mut workerThreads= vec![ ];

    for _ in 0..10 {
        let counterMutex= Arc::clone(&counterMutex);
        let workerThread= thread::spawn(
            move | | {
                let mut counterReference= counterMutex.lock( )
                    .unwrap( );

                *counterReference += 1;
            }
        );
        workerThreads.push(workerThread);
    }

    for workerThread in workerThreads {
        workerThread.join( )
            .unwrap( );
    }

    let finalCounterValue= *counterMutex.lock( )
        .unwrap( );
    println!("final value of the counter - {}", finalCounterValue);
}
```

You might wonder - why do we have something like Rc then? Why not use Arc in all situations? This is because of the performance drop that Arc brings. This is why, in case of single threaded scenarios, we just use Rc to allow multiple ownership of a data.

> Mutex comes with the `possibility of creating deadlocks`
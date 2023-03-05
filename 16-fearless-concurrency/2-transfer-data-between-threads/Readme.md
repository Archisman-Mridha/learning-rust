# Using message passing to transfer data between threads

One increasingly popular approach to ensuring safe concurrency is `message passing`, where threads communicate by **sending each other messages pointing to some data**. Rust implements message passing between threads using `channels`.
A channel has 2 parts - a `transmitter` (data is put here by a thread) and a `receiver` (data is consumed from here by a thread). The channel is said to be **closed if the transmitter or receiver is dropped**.
The way Rust implements a channel - mpsc (`multiple producers, single consumer`). This means that there can be multiple transmitters but only a single receiver for a channel.

Here is an example of a channel with a single producer and a single consumer -
```rust
fn main( ) {
    let (transmitter, receiver)= channel( );

    let workerThread= thread::spawn(
        move | | {
            let message= "hi".to_string( );

            //* `send` can return an error if the receiver part of the channel has been dropped */
            transmitter.send(message)
                .unwrap( );
        }
    );

    //* `recv` blocks the main thread until a message is received */
    // `recv` can also return an error if the transmitter part of the channel is closed
    let message= receiver.recv( )
        .unwrap( );
    println!("message recived from the worker thread - {}", message);

    workerThread.join( )
        .unwrap( );
}
```

Instead of `recv` we can use the `try_recv` method which **doesn't block the parent thread** and **immediately returns a message if available**.

## Channels and ownership transferrence
Here is an example of how, **ownership rules and channels combine in Rust to prevent concurrency related runtime problems**. Consider this code -
```rust
fn main( ) {
    let (transmitter, receiver)= channel( );

    let workerThread= thread::spawn(
        move | | {
            let message= "hi".to_string( );

            transmitter.send(message)
                .unwrap( );

            /*
                The compiler will give an error here. Because `message` is sent to the channel and now can be consumed by another
                thread. So there can be situations, where this thread and the other thread can try to perform operations on
                `message` simulataneously thus leading to a race condition.
                When we send `message` to the channel, this worker thread loses the ownership.
            */
            println!("message sent to the channel - {:?}", message);
        }
    );

    let message= receiver.recv( )
        .unwrap( );

    workerThread.join( )
        .unwrap( );
}
```

## Multiple producer and single consumer
Here is how to create a channel with multiple transmitters and a single receiver -
```rust
fn main( ) {
    let (transmitterA, receiver)= channel( );
    let transmitterB= transmitterA.clone( ); // creating another transmitter

    thread::spawn(
        move | | {
            let message= "hi from worker-thread A".to_string( );

            //* `send` can return an error if the receiver part of the channel has been dropped */
            transmitterA.send(message)
                .unwrap( );
        }
    );

    thread::spawn(
        move | | {
            let message= "hi from worker-thread B".to_string( );

            transmitterB.send(message)
                .unwrap( );
        }
    );

    for message in receiver {
        println!("message received from the channel - {}", message);}
}
```
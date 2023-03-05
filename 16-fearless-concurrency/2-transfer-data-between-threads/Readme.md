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
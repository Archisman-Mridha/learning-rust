#![allow(non_snake_case)]

use std::{sync::mpsc::channel, thread};

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
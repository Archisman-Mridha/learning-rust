#![allow(non_snake_case)]

use std::{sync::mpsc::channel, thread};

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
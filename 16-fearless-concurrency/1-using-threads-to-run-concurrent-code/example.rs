#![allow(non_snake_case)]

use std::thread;

fn main( ) {
    let vector= vec![ 1,2,3 ];

    let workerThread= thread::spawn(
        move | | {
            println!("the vector is - {:?}", vector);
        }
    );

    // this will block the main thread until the worker thread finishes execution
    workerThread.join( )
        .unwrap( );
}
#![allow(non_snake_case)]

use std::{sync::{Mutex, Arc}, thread};

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
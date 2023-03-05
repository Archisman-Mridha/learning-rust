use std::thread;

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
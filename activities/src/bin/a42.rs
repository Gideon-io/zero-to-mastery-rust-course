// Topic: Implementing Iterator
//
// Summary:
// A game uses a scoring system that includes a score multiplier.
// The multiplier starts at 1 and increases by 1 each iteration.
// The amount the multiplier increases each iteration can be
// adjusted through in-game powerups.
//
// Example multiplier progression:
// 1, 2, 3, (+1 powerup obtained), 5, 7, 9, ...
//
// Requirements:
// * Write a program that uses an iterator to generate a score multiplier
// * The iterator must start at 1 and increase by 1 each iteration
//   * It must be possible to increase the per-iteration amount through powerups
//
// Notes:
// * Use the .next() method to advance the iterator to confirm it works correctly
// * Only the Iterator trait needs to be implemented for this activity
use crossbeam_channel::unbounded;
use std::thread;

enum WorkerMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

enum MainMsg {
    SumResult(i64),
    WorkerQuit,
}
fn main() {
    let (worker_tx,worker_rx) = unbounded(); //worker channel created with 1 receiving and send end
    let (main_tx,main_rx) = unbounded();

    let worker = thread::spawn(move || loop {  //creating a worker thread here via spawn
                                        match worker_rx.recv() {
                                            Ok(msg) => match msg {
                                                WorkerMsg::PrintData(msg) => println!("Worker: {}", msg),
                                                WorkerMsg::Sum(a,b) => {
                                                    println!("Worker summing");
                                                    main_tx.send(MainMsg::SumResult(a + b));
                                                    ()
                                                }
                                                WorkerMsg::Quit => {
                                                println!("Worker thread initiating termination");
                                                main_tx.send(MainMsg::WorkerQuit);
                                                break;
                                                }
                                            }
                                            Err(_) => {
                                                println!("Worker Disconnected");
                                                main_tx.try_send(MainMsg::WorkerQuit);
                                                break;
                                            }
                                        }
    });
                              
    worker_tx.send(WorkerMsg::PrintData("Hello from main".to_string())).unwrap();
    worker_tx.send(WorkerMsg::Sum(5,6)).unwrap();
    worker_tx.send(WorkerMsg::Quit).unwrap();
    //drop(s); //just to initiate the disconnected message in the Err arm      

    while let Ok(msg) = main_rx.recv() {  //recv is blocked so it will wait until a message comes along. if there is and its Ok perfrom the match
        match msg {
            MainMsg::SumResult(answer) => println!("main answer: {}", answer),
            MainMsg::WorkerQuit => println!("Main has quit")
        }
    }      

    worker.join().unwrap();
    

}

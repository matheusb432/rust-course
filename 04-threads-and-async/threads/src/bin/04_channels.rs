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
    // NOTE Creating a worker thread that will enable bidirectional communication
    let (worker_tx, worker_rx) = unbounded();
    let (main_tx, main_rx) = unbounded();
    let worker = thread::spawn(move || loop {
        match worker_rx.recv() {
            Ok(msg) => match msg {
                WorkerMsg::PrintData(d) => println!("Worker: {d:?}"),
                WorkerMsg::Sum(x, y) => {
                    println!("Worker: summing...");
                    // NOTE Sending a message to the main thread
                    main_tx.send(MainMsg::SumResult(x + y)).unwrap();
                }
                WorkerMsg::Quit => {
                    println!("Worker: terminating...");
                    main_tx.send(MainMsg::WorkerQuit).unwrap();
                    break;
                }
            },
            Err(_) => {
                println!("Worker: disconnected");
                // ? try_send won't block the thread so that the thread can be terminated
                main_tx.try_send(MainMsg::WorkerQuit).unwrap();
                break;
            }
        }
    });

    worker_tx
        .send(WorkerMsg::PrintData("hello from main".to_owned()))
        .unwrap();
    worker_tx.send(WorkerMsg::Sum(4, 2)).unwrap();
    worker_tx.send(WorkerMsg::Quit).unwrap();

    // NOTE Configuring the main thread to receive messages from the worker thread
    // ? Even if the worker already terminated, the main thread will still receive the channel messages
    while let Ok(msg_from_worker) = main_rx.recv() {
        match msg_from_worker {
            MainMsg::SumResult(res) => println!("Main: answer = {res}"),
            MainMsg::WorkerQuit => println!("Main: worker terminated"),
        }
    }

    worker.join().unwrap();
}

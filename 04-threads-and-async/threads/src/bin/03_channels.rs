use crossbeam_channel::unbounded;
use std::thread;

enum ThreadMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

fn main() {
    let (sender, receiver) = unbounded::<ThreadMsg>();
    // NOTE This thread will enable unidirectional communication from `main` to `handle`
    let handle = thread::spawn(move || loop {
        match receiver.recv() {
            Ok(msg) => match msg {
                ThreadMsg::PrintData(d) => println!("{d:?}"),
                ThreadMsg::Sum(x, y) => println!("{}+{}={}", x, y, x + y),
                ThreadMsg::Quit => {
                    println!("thread terminating");
                    break;
                }
            },
            Err(_) => {
                println!("disconnected");
                break;
            }
        }
    });

    sender
        .send(ThreadMsg::PrintData("hello from main".to_owned()))
        .expect("panic!");
    sender.send(ThreadMsg::Sum(4, 2)).expect("panic!");
    sender.send(ThreadMsg::Quit).expect("panic!");
    // ? Dropping the sender will trigger the Err result in the `handle` thread
    // drop(sender);

    handle.join().expect("panic!");
}

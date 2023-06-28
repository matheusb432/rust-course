// Topic: Arc, Mutex, and Threads
//
// Summary:
// Modify the existing multi-threaded program to include a global
// counter shared among the threads. The counter should increase
// by 1 whenever a worker completes a job.
//
// Requirements:
// * 1. The total number of jobs completed must be displayed
//      at the end of the program.
// * 2. Use Arc & Mutex to share the total count among threads.
//      * Arc is in the standard library
//      * Mutex is in the parking_lot crate
//
// Notes:
// * Ensure following crates are added to your Cargo.toml file:
//   - crossbeam-channel
//   - parking_lot

use crossbeam_channel::{unbounded, Receiver, Sender};
use parking_lot::Mutex;
use std::{
    collections::VecDeque,
    sync::Arc,
    thread::{self, JoinHandle},
    time::Duration,
};

/// Job given to workers.
#[derive(Clone)]
enum Job {
    Print(String),
    Sum(isize, isize),
}

/// Message sent to workers.
#[derive(Clone)]
enum Message {
    AddJob(Job),
    Quit,
}

struct Worker<M> {
    tx: Sender<M>,
    _rx: Receiver<M>,
    handle: JoinHandle<()>,
}

impl Worker<Message> {
    fn add_job(&self, job: Job) {
        self.tx
            .send(Message::AddJob(job))
            .expect("failed to add job");
    }
    fn join(self) {
        self.handle.join().expect("failed to join thread");
    }
    fn send_msg(&self, msg: Message) {
        self.tx.send(msg).expect("failed to send message");
    }
}

/// Create a new worker to receive jobs.
fn spawn_worker(job_counter: Arc<Mutex<usize>>) -> Worker<Message> {
    let (tx, rx) = unbounded();
    // We clone the receiving end here so we have a copy to give to the
    // thread. This allows us to save the `tx` and `rx` into the Worker struct.
    let rx_thread = rx.clone();
    let handle = thread::spawn(move || {
        // VecDeque allows us to get jobs in the order they arrive.
        let mut jobs = VecDeque::new();

        // Outer loop is so we can have a brief delay when no jobs are available.
        loop {
            let mut jobs_count = 0;

            // * Inner loop continuously processes jobs until no more are available.
            loop {
                // Get the next job.
                while let Some(job) = jobs.pop_front() {
                    match job {
                        Job::Print(msg) => println!("{}", msg),
                        Job::Sum(lhs, rhs) => println!("{}+{}={}", lhs, rhs, lhs + rhs),
                    }
                    // ? More efficient to increment a local variable so less locks are necessary
                    // * However, this makes it so shared_jobs_count only has the latest data when the worker quits
                    jobs_count += 1;
                }

                if let Ok(msg) = rx_thread.try_recv() {
                    match msg {
                        Message::AddJob(job) => {
                            jobs.push_back(job);
                            continue;
                        }
                        Message::Quit => {
                            // ? 2.
                            let mut jc = job_counter.lock();
                            // ? Dereferencing the lock to increment it to the completed jobs before quitting the thread
                            *jc += jobs_count;

                            println!("Worker: jc: {:?} | Terminating thread...", *jc);
                            return;
                        }
                    }
                } else {
                    break;
                }
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    Worker {
        tx,
        _rx: rx,
        handle,
    }
}

fn main() {
    let jobs = vec![
        Job::Print("msg 1".to_owned()),
        Job::Sum(2, 2),
        Job::Print("msg 2".to_owned()),
        Job::Sum(4, 4),
        Job::Print("msg 3".to_owned()),
        Job::Sum(1, 1),
        Job::Print("msg 4".to_owned()),
        Job::Sum(10, 10),
        Job::Print("msg 5".to_owned()),
        Job::Sum(3, 4),
        Job::Print("msg 6".to_owned()),
        Job::Sum(9, 8),
        Job::Print("msg 7".to_owned()),
        Job::Sum(1, 2),
        Job::Print("msg 8".to_owned()),
        Job::Sum(9, 1),
    ];
    // NOTE Creating an Arc to share data between threads
    let shared_job_counter = Arc::new(Mutex::new(0usize));

    let jobs_sent = jobs.len();

    let mut workers = vec![];
    for _ in 0..4 {
        // NOTE every worker will have access to a pointer to the same data in memory via Arc
        let worker = spawn_worker(shared_job_counter.clone());
        workers.push(worker);
    }

    let mut worker_ring = workers.iter().cycle();
    for job in jobs.into_iter() {
        // Get next worker
        let worker = worker_ring.next().expect("failed to get worker");
        worker.add_job(job);
    }

    // Ask all workers to quit.
    for worker in &workers {
        worker.send_msg(Message::Quit);
    }

    // Wait for workers to terminate.
    for worker in workers {
        worker.join();
    }

    println!("Jobs sent: {}", jobs_sent);

    // ? 1.
    println!("Jobs completed: {:?}", shared_job_counter.lock());
}

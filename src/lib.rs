use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    // threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
// struct Job;
type Job = Box<dyn FnBox + Send + 'static>;
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        // let mut threads = Vec::with_capacity(size);
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // threads.push(thread::spawn(|| {}));
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
trait FnBox {
    fn call_box(self: Box<Self>);
}
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} get a job; executing", id);
            // (*job)()
            job.call_box()
        });

        Worker { id, thread }
    }
}

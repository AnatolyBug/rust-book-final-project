use std::thread;
use std::sync::{Arc, Mutex, mpsc};
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
// make type look prettier
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl ThreadPool {
    /// Create a new thread pool
    /// # Panics
    /// Will panic is the size is 0
    pub fn new(size: usize) -> Self {
        assert!(size>0);
        // this can hold size number of items
        let mut workers = Vec::with_capacity(size);

        let(sender, reciever) = mpsc::channel(); 
        // <Job> because Rust is smart enough to deduce from ThreadPool def

        let reciever = Arc::new(Mutex::new(reciever));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        };

        ThreadPool{workers, sender}
    }
    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static, 
    // FnOnce thread for running a request will only execute the closure once,
    // and we pass the argument from execute to spawn
    // Send to transfer closure from one thread to another 
    // 'static because we don't know how long the thread will live
    {
        let job: Job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            println!("Worker {} is listening", id);
            //don't use while let, since unlock is based on lifetime of Mutex
            // let drop any temp values on RHS immidiately, while
            // while let, if let, match do not drop until the end of the block
            // remember lock blocks the (child) thread untill recieving a message
            let job = reciever.lock()
            .unwrap()
            .recv()
            .unwrap();
            println!("Worker {} got a job; executing", id);
            job();    
        } );
        Worker{id, thread}
    }
}
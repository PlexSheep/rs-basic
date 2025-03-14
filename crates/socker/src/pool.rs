use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[allow(dead_code)]
struct WorkerThread {
    handle: thread::JoinHandle<()>,
    id: usize,
}

impl WorkerThread {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> WorkerThread {
        let handle = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });
        WorkerThread { handle, id }
    }
}

/// Shares tasks over multiple worker threads
pub struct ThreadPool {
    /// executes tasks assigned by the instructor
    #[allow(dead_code)]
    workers: Vec<WorkerThread>,
    /// sends instructions to the workers
    sender: mpsc::Sender<Job>,
}

/// Something for the workers to do
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, String> {
        if size == 0 {
            return Err("cannot build a thread pool with size 0!".to_string());
        }
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(WorkerThread::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    /// schedules the given job for execution
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // now it's getting crazy
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

use std::thread::JoinHandle;
use std::thread;
use std::sync::mpsc;

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

pub struct Worker{
    id: usize,
    thread: JoinHandle<()>,
}

struct Job{}

impl ThreadPool{
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) ->ThreadPool {
        assert!(size > 0);
        let (sender, reciever) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            workers.push(Worker::new(id));
        }

        ThreadPool{
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce(),
            F: Send + 'static,
    {

    }
}

impl Worker{
    fn new(id: usize)-> Worker {
        Worker{
            id,
            thread: thread::spawn(|| {}),
        }
    }
}
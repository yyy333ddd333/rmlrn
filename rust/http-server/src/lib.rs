use std::thread::JoinHandle;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

pub struct Worker{
    id: usize,
    thread: JoinHandle<()>,
}

type Job = Box<FnBox + Send + 'static>;

trait FnBox{
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F{
    fn call_box(self: Box<Self>){
        (*self)();
    }
}

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

        let reciever = Arc::new(Mutex::new(reciever));
        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&reciever)));
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
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Worker{
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>)-> Worker {
        Worker{
            id,
            thread: thread::spawn(move || {
                loop{
                    //let (tx, rx) = mpsc::channel();
                    //let job = rx.recv();
                    let job = reciever.lock().unwrap().recv().unwrap();
                    println!("Worker {} got a job; executing.", id);
                    job.call_box();
                }
            }),
        }
    }
}

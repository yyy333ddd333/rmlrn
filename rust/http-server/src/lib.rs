use std::thread::JoinHandle;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

pub struct Worker{
    id: usize,
    thread: Option<JoinHandle<()>>,
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

enum Message {
    NewJob(Job),
    Terminate,
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
        let message = Message::NewJob(job);
        self.sender.send(message).unwrap();
    }
}

impl Worker{
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>)-> Worker {
        Worker{
            id,
            thread: Some(thread::spawn(move || {
                loop{
                    //let (tx, rx) = mpsc::channel();
                    //let job = rx.recv();
                    let message = reciever.lock().unwrap().recv().unwrap();//blocked by the method recv(), wait for sth.
                    
                    match message {
                        Message::NewJob(job) => {
                            println!("Worker {} got a job; executing.", id);
                            job.call_box();
                        },
                        Message::Terminate => {
                            println!("Worker {} was told to terminate.", id);
                            break;
                        }
                    }
                }
            })),
        }
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        println!("Sending terminate message to all workers.");

        for _ in & self.workers {
            self.sender.send(Message::Terminate).unwrap();//send是异步方法，不会block当前程序
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers{
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        } 
    }
}

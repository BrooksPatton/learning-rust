use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

trait FnBox {
  fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
  fn call_box(self: Box<F>) {
    (*self)()
  }
}

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

impl ThreadPool {
  /// Panics!
  /// 
  /// Panics when provided number of total threads is less than 1
  pub fn new(total_threads: usize) -> ThreadPool {
    assert!(total_threads > 0);

    let mut workers = Vec::with_capacity(total_threads);
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    for id in 0..total_threads {
      workers[id] = Worker::new(id, Arc::clone(&receiver));
    }

    ThreadPool {
      workers,
      sender,
    }
  }

  pub fn execute<F>(&self, closure: F) 
    where
      F: FnOnce() + Send + 'static
  {
    let job = Box::new(closure);

    self.sender.send(job).unwrap(); 
  }
}

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    Worker {
      id,
      thread: thread::spawn(move || {
        loop {
          let job = receiver.lock().unwrap().recv().unwrap();

          println!("Worker id {} got a job, executing!", id);

          job.call_box();
        }
      }),
    }
  }
}

type Job = Box<dyn FnBox + Send + 'static>;
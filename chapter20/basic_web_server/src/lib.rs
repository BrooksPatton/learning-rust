use std::thread;

pub struct ThreadPool {
  threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
  /// Panics!
  /// 
  /// Panics when provided number of total threads is less than 1
  pub fn new(total_threads: usize) -> ThreadPool {
    assert!(total_threads > 0);

    let threads = Vec::with_capacity(total_threads);

    // for _ in 0..total_threads {
    //   thread::
    // }

    ThreadPool {
      threads
    }
  }

  pub fn execute<F>(&self, closure: F) 
    where
      F: FnOnce() + Send + 'static
  {

  }
}
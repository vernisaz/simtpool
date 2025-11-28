use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

pub struct Worker {
    #[allow(dead_code)]
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            #[cfg(trace)]
            eprintln!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() && let Err(e) = thread.join() { 
                eprintln!{"Thread join error: {e:?}"} 
            }
        }
    }
}

impl  Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::Builder::new().name(format!{"pooled thread {id}"}).spawn(move || loop {
            // using while or if let will block other threads from an execution
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    #[cfg(trace)]
                    eprintln!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    eprintln!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        }).ok();

        Worker { id, thread }
    }
}

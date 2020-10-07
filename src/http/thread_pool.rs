use std::{
    io,
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() -> io::Result<()> + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F) -> ()
    where
        F: FnOnce() -> io::Result<()> + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .send(Message::NewJob(job))
            .expect("Error sending new job message");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        debug!("Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender
                .send(Message::Terminate)
                .expect("Error sending terminate message");
        }

        debug!("Shutting down all workers.");
        for worker in &mut self.workers {
            debug!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    debug!("Worker {} got a job; executing.", id);
                    match job() {
                        Err(e) => error!("Worker {} got an error {}", id, e),
                        Ok(()) => debug!("Worker {} finsihed job successfully", id),
                    }
                }
                Message::Terminate => {
                    debug!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}

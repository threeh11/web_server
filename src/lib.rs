use tokio::sync::mpsc;
use tokio::task;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    receiver: mpsc::Receiver<Job>,
}

impl Worker {
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        Worker { id, receiver }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        println!("Worker {} dropped", self.id);
    }
}

impl Worker {
    async fn run(mut self) {
        while let Some(job) = self.receiver.recv().await {
            println!("Worker {} received job", self.id);
            job();
        }
    }
}
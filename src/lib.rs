use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Создает новый ThreadPool
    ///
    /// size - количество пулов.
    ///
    /// # Panics
    ///
    /// `new` - функция вызывает панику, когда колчиество пулов равно нулю
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Мы создаём новый канал, используя функцию mpsc::channel;
        // mpsc означает несколько производителей, один потребитель (multiple producer, single consumer)'
        // https://doc.rust-lang.ru/book/ch16-02-message-passing.html

        // Щас будет большой пример
        // use std::sync::mpsc;
        // use std::thread;
        //
        // fn main() {
        //     let (tx, rx) = mpsc::channel();
        //
        //     thread::spawn(move || {
        //         let val = String::from("hi");
        //         tx.send(val).unwrap();
        //     });
        // }
        // Опять же, мы используем thread::spawn для создания нового потока,
        // а затем используем move для перемещения tx в замыкание, чтобы порождённый поток владел tx .
        // Порождённый поток должен владеть передатчиком, чтобы иметь возможность отправлять сообщения
        // через канал. Передатчик имеет метод send , который принимает значение, которое мы хотим отправить.
        // Метод send возвращает тип Result<T, E> , поэтому, если получатель уже удалён и отправить значение некуда,
        // операция отправки вернёт ошибку. В этом примере мы вызываем unwrap для паники в случае ошибки.
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        // with_capacity - если обычный вектор не знает сколько выделять памяти, то
        // используя with_capacity - мы можем точно сказать сколько памяти нам надо под вектор
        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        // Мб потом на uuid переделать, надо будет погуглить
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

//Когда пул удаляется, все наши потоки должны объединиться (join), чтобы убедиться, что они завершают свою работу
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Уронил воркер {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    /// Создает новый Worker
    /// экземпляр Worker содержит id и поток, порождённый с пустым замыканием
    ///
    /// id - идентификатор воркера.
    ///
    /// # Panics ...
    ///

    // Надо будет потом это в Result обернуть, т.к для создания потока может не хватить системных ресурсов
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Воркер {id} приступил к обработке запроса");

                    job();
                }
                Err(_) => {
                    println!("Ворке {id} упал; офается.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
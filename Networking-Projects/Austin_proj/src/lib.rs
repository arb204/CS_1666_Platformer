use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    //Threadpool holds a vector of worker structs, which manage threads.
    workers: Vec<Worker>,

    //Holds onto the sending side of the channel.
    sender: mpsc::Sender<Message>
}

//Job is a box that holds these traits and lifetimes.
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    //Function to create a new Thread pool
    pub fn new(size: usize) -> ThreadPool {

        //Panics if size is 0 or less. 0 threads makes no sense.
        assert!(size > 0);

        //Creates a new channel and stores the sender and reciever sides of the channel respectively.
        let (sender, receiver) = mpsc::channel();

        //Initialize the receiver to be an Arc pointer so that it can be shared across threads.
        //Add a mutex so that only one worker can get a job from the receiver at a time.
        let receiver = Arc::new(Mutex::new(receiver));

        //Initialize a vector with number of threads specified
        let mut workers = Vec::with_capacity(size);

        //For loop which will be used to create threads (Difficult to create threads that wait for code that they will exec later)
        for id in 0..size {
            //Creates a new worker that will manage a thread
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    //A function which takes in a closure an processes it one time.
    //Send is used to transfewr the closure from one thread to another.
    //The F type parameter also gets a 'static lifetime because we do not know how long the thread will take to execute.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        {
            //Creates a new job instance from the closure we get and packages that job into a box.
            let job = Box::new(f);

            //Sends the job down the channel to be picked up by a reciever in the workers.
            self.sender.send(Message::NewJob(job)).unwrap();
        }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        //Sends a message to all workers to terminate and stop accepting new requests.
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        
        //Iterates through all the workers in the thread pool
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            //Takes a thread from the worker, and leaves none in its place.
            //If it is already none then it is cleaned up so we do nothing in that case.
            if let Some(thread) = worker.thread.take() {
                //Joins the thread
                thread.join().unwrap();
            }
            //Joins the threads of the workers.
            //worker.thread.join().unwrap();
        }
    }
}

//Struct that monitors the execution of certain threads.
struct Worker {
    //ID of the worker
    id: usize, 
    //Thread that the worker manages the execution of. 
    //Is now an option so that if a worker doesn't have a thread it has a none value. 
    //We can also use take to get the value out of the some variant. (How we will join the threads)
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    //Creates a new worker
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        //Spawns a thread for that worker, the thread executes nothing as of right now
        let thread = thread::spawn(move || loop {

            //lock locks the mutex on the reciever.
            //recv is called to receive a job from the channel. 
            //recv blocks if there is no job so the thread will wait for a job to become available.
            let message = receiver.lock().unwrap().recv().unwrap(); //unlocks at the end of the type function

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    //Allows thread to execute the job which would have previously been passed into spawn
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    //Breaks out of the loop
                    break;
                }
            } 
        });

        //Returns a worker with the inputted id and its corresponding created thread
        Worker {
            id, 
            thread: Some(thread),
        }
    }
}
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    //Threadpool holds a vector of worker structs, which manage threads.
    workers: Vec<Worker>,

    //Holds onto the sending side of the channel.
    sender: mpsc::Sender<Job>
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
            self.sender.send(job).unwrap();
        }
}

//Struct that monitors the execution of certain threads.
struct Worker {
    //ID of the worker
    id: usize, 
    //Thread that the worker manages the execution of.
    thread: thread::JoinHandle<()>,
}

impl Worker {
    //Creates a new worker
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        //Spawns a thread for that worker, the thread executes nothing as of right now
        let thread = thread::spawn(move || loop {

            //lock locks the mutex on the reciever.
            //recv is called to receive a job from the channel. 
            //recv blocks if there is no job so the thread will wait for a job to become available.
            let job = receiver.lock().unwrap().recv().unwrap(); //unlocks at the end of the type function

            println!("Worker {} got a job, and is executing.", id);

            //Allows thread to execute the job which would have previously been passed into spawn
            job();
        });

        //Returns a worker with the inputted id and its corresponding created thread
        Worker {id, thread}
    }
}
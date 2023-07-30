use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};

static ATOM: AtomicUsize = AtomicUsize::new(0);

pub struct Envelope
{
    sender : mpsc::Sender<String>,
    message: String
}


fn start_senders(max_senders: u32, depth: usize)
    -> (mpsc::Receiver<Envelope>, Vec<thread::JoinHandle<()>>)
{
    let (sender, receiver) = mpsc::sync_channel(depth);

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    for j in 0u32..max_senders
    {
        let my_sender = sender.clone();
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            let mut i = 0u32;
            let mut min_elapsed = u128::MAX;
            let mut max_elapsed = 0u128;
            let mut total_elapsed = 0u128;
            loop {
                ATOM.fetch_add(1, Ordering::SeqCst);
                let s : String = String::new() + "Thread_" + &j.to_string() + "_" + &i.to_string();

                // Create a response channel
                let (resp_sender, resp_receiver) = mpsc::channel::<String>();
                let envelope = Envelope{sender:resp_sender, message:s};
                let now = Instant::now();
                if my_sender.send(envelope).is_err() {
                    println!("Thread_{} detected receiver stop on i = {} with elapsed min/max/total = {} / {} / {} micros ",j,i,min_elapsed,max_elapsed, total_elapsed);
                    break;
                }
                let elapsed = now.elapsed().as_micros();
                if resp_receiver.recv().is_err()
                {
                    println!("Thread_{} detected receiver stop on i = {} with elapsed min/max/total = {} / {} / {} micros ",j,i,min_elapsed,max_elapsed, total_elapsed);
                    break;
                }
                total_elapsed += elapsed;
                if elapsed > max_elapsed
                {
                    max_elapsed = elapsed;
                }
                if elapsed < min_elapsed
                {
                    min_elapsed = elapsed;
                }
                i += 1;
            }
        });

        handles.push(handle);
    }

    (receiver, handles)
}

fn start_logger(envelopes : Receiver<Envelope>, depth: usize )
    -> ()
{
    let mut i = 0usize;
    for envelope in envelopes
    {
        if envelope.sender.send("ok".to_string()).is_err()
        {
            println!("Loggerdetected receiver stop on i = {}",i);
            break;
        }
        println!("{}",envelope.message);
        i += 1;
        if i > depth
        {
            break;
        }
    }
}
fn main() {
    let depth = 50000;
    let max_senders = 10u32;
    let (messages, handles) = 
    start_senders(max_senders, depth);
    
    start_logger(messages, depth);

    for handle in handles
    {
        handle.join().unwrap();
    }

    println!("Total loops for all threads is {}",ATOM.load(Ordering::SeqCst));
}

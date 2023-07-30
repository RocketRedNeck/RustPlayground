use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Instant;

fn start_senders(max_senders: u32, depth: usize)
    -> (mpsc::Receiver<String>, Vec<thread::JoinHandle<()>>)
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
                let s : String = String::new() + "Thread_" + &j.to_string() + "_";
                let now = Instant::now();
                if my_sender.send(s + &i.to_string()).is_err() {
                    println!("Thread_{} detected receiver stop on i = {} with elapsed min/max/total = {} / {} / {} micros ",j,i,min_elapsed,max_elapsed, total_elapsed);
                    break;
                }
                let elapsed = now.elapsed().as_micros();
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

fn start_logger(messages : Receiver<String> )
    -> ()
{
    let mut i = 0u32;
    for message in messages
    {
        println!("{}",message);
        i += 1;
        if i > 100000
        {
            break;
        }
    }
}
fn main() {
    let (messages, handles) = 
    start_senders(10, 100000);
    
    start_logger(messages);

    for handle in handles
    {
        handle.join().unwrap();
    }
}

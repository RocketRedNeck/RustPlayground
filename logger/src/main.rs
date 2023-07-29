use std::io;
use std::sync::mpsc::{self, Receiver};
use std::thread;

fn start_senders(max_senders: u32)
    -> (mpsc::Receiver<String>, Vec<thread::JoinHandle<()>>)
{
    let (sender, receiver) = mpsc::sync_channel(10000);

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    for j in 0u32..max_senders
    {
        let my_sender = sender.clone();
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            let mut i = 0u32;
            loop {
                let s : String = String::new() + "Thread_" + &j.to_string() + "_";
                if my_sender.send(s + &i.to_string()).is_err() {
                    println!("Thread_{} detected receiver stop on i = {}",j,i);
                    break;
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
        if i > 50000
        {
            break;
        }
    }
}
fn main() {
    let (messages, handles) = start_senders(20u32);
    start_logger(messages);

    for handle in handles
    {
        handle.join().unwrap();
    }
}

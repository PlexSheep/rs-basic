//! Example on how to use std::mpsc with 2 threads

use std::{
    io::Write,
    str::FromStr,
    sync::{mpsc, Arc, Barrier},
    thread,
};

// put anything into Results
use anyhow::Result;

// simulate a complex datatype with special meaning being sent somewhere
#[derive(Clone, Debug)]
struct Message {
    payload: String,
}

// be able to print the Message
impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message {{{}}}", self.payload)
    }
}

// make a new Message from a string
impl std::str::FromStr for Message {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self {
            payload: s.to_string(),
        })
    }
}

// printer thread function, will print any message received via the mpsc channel
fn printer(receiver: mpsc::Receiver<Message>, barrier: Arc<Barrier>) -> Result<()> {
    let mut stdout = std::io::stdout();

    // run as long as we can receive something (err always means all senders have been dropped)
    while let Ok(msg) = receiver.recv() {
        println!("{msg}");
        stdout.flush()?;
        barrier.wait(); // done with printing, the main thread can continue
    }
    Ok(())
}

fn main() -> Result<()> {
    // channel that can be split across threads to send values between them
    // mpsc means Multi-producer, single-consumer
    let (sender, receiver) = mpsc::channel();

    // we need to wait for the printer thread to be done before we print the prompt "> " to stdout,
    // otherwise we mix up the prints.
    //
    // We put it into an Arc, so that we can share it across threads.
    let barrier = Arc::new(Barrier::new(
        2, /* number of threads, continue when this many are waiting */
    ));
    let barrier_printer = barrier.clone(); // second one for the printer, this contains a reference
                                           // to our original barrier. (An Arc is a special kind of reference)

    // we spawn a thread and give it something to run
    let _handle = thread::spawn(|| printer(receiver, barrier_printer).expect("printer error"));

    let mut msg; // we store our messages here
    let mut buf = String::new(); // we put the contents of the stdin here
    let stdin = std::io::stdin(); // we read user input from here
    let mut stdout = std::io::stdout(); // we need this to flush explicitly

    // do this forever
    loop {
        buf.clear(); // we want an empty buf at the start

        // print a prompt while staying in the same line
        print!("> ");
        stdout.flush()?; // make sure that the stdout gets printed now instead of waiting for a
                         // newline (stdout flushes automatically at newlines)

        let _ = stdin.read_line(&mut buf)?; // read the user input

        // check for special inputs
        if buf == "\n" {
            // enter
            continue;
        } else if buf.to_lowercase() == "exit\n" || buf.is_empty() {
            // exit or ctrl-d
            break;
        }

        buf = buf.replace('\n', ""); // we don't need the newline, just accept the user input when
                                     // the user presses return
                                     // if the user input was empty start anew

        // convert the user input into a message (we could also just send a String, but this
        // simulates a more complex behavior)
        msg = Message::from_str(&buf).unwrap();

        // send the `Message` to the printer thread
        sender.send(msg)?;

        // wait until the printer is done printing the message, so we dont mix
        // stdout prints (we use print instead of println)
        barrier.wait();
    }
    Ok(())
}

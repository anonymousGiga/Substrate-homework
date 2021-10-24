use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

//This is the handle function which can delt the request from client
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    //Create a buffer
    let mut buf = [0; 512];
    //We can read 1000 times from the stream
    for _ in 0..1000 {
        //Read data from the stream
        let bytes_read = stream.read(&mut buf)?;
        //If there is no data can read from stream, then we can return
        if bytes_read == 0 {
            //Return
            return Ok(());
        }

        //Write the content which read from the stream to the stream.
        stream.write(&buf[..bytes_read])?;
        //Sleep 1 second.
        thread::sleep(time::Duration::from_secs(1));
    }
    //Return
    Ok(())
}

fn main() -> io::Result<()> {
    // Create a tcp socket server, listening for connections.
    // This server bind "127.0.0.1:8080"
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    // Create a vector which will fill handler thread.
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    //Dealt the incoming from the accepts connections of the tcp server
    for stream in listener.incoming() {
        //Because the stream is a Result, so need get the value from the Result.
        let stream = stream.expect("failed");
        //Create a thread to handle the request from client and return the thread handler.
        let handle = thread::spawn(move || {
            //This is the handler function
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        //Push the thread handler to thread_vec, then we can wait the thread close
        thread_vec.push(handle);
    }
    //Wait all the threads close
    for handle in thread_vec {
        //Wait thread close and match the Result
        match handle.join() {
            //If close thread ok
            Ok(_) => {
                println!("Close the thread success!");
            }
            //If close thread error
            Err(_) => {
                println!("Some error when close the thread!");
            }
        }
    }
    //Return
    Ok(())
}

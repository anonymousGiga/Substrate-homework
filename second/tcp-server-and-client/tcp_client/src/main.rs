use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<()> {
    //Create a stream which connect 127.0.0.1:8080(Server address)
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    //We can send request 10 times
    for _ in 0..10 {
        //Send data to Server
        //Create a param which will store the data input from terminal
        let mut input = String::new();
        //Read data from terminal input to param input
        io::stdin().read_line(&mut input).expect("Failed to read");
        //Write the data to stream
        stream.write(input.as_bytes()).expect("Failed to write");

        //Then we will read from server
        //Create a reader
        let mut reader = BufReader::new(&stream);
        //Create a buffer which will fill the receive data
        let mut buffer: Vec<u8> = Vec::new();
        //Read data from stream to buffer
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Failed to read into buffer");
        //Print the read result
        println!("read  from server: {}", str::from_utf8(&buffer).unwrap());
        //Print a empty lin
        println!("");
    }
    //Return
    Ok(())
}

extern crate rand ;

use std::net::{TcpListener, TcpStream} ;
use std::thread ;
use rand::{thread_rng , Rng} ;
use std::time::Duration ;
use std::io::{Read,Write, Error};


fn client_handler(mut stream:TcpStream)->Result<(),Error>{
    let mut buff:[u8;512 ] = [0; 512 ] ;

    loop{
        let bytes_read:usize = stream.read(&mut buff)? ;
        if bytes_read == 0 {return Ok(());}
        let sleep = Duration::from_secs(*thread_rng().choose(&[01,2,3,4,5]).unwrap());
        println!("Sleeping for {:?} before replaying", sleep);
        std::thread::sleep(sleep);
        let bytes_written:usize = stream.write(&buff[..bytes_read])?;
        println!("This many bytes where written back into this stream {:?}  bytes", bytes_written) ;
    }

}


fn main() {
    let socket:&str = "0.0.0.0:9999" ;
    let listener:TcpListener = TcpListener::bind(socket).expect("Could not bind socket" ) ;

    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                thread::spawn(
                    move || {
                                client_handler(stream)
                                .unwrap_or_else(|error| eprintln!("{:?}",error));
                            });
            },
            Err(e) => eprintln!("failed: {}", e)
        }
    }

}

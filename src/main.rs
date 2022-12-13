use std::str;
use std::process;
use std::thread;
use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() {
    let host = "mtl.wserv.org";
    let port = 6112;

    let mut stream = TcpStream::connect((host, port)).unwrap();
    let mut input_stream = stream.try_clone().unwrap();

    input_stream.write("C1\r\nACCT WarRecon\r\nPASS PASSWORD\r\nHOME KoG\r\nLOGIN\r\n".as_bytes());
    input_stream.flush();

    // let handler =
    thread::spawn(move || {
        let mut client_buffer = [0u8; 1024];

        loop {
            match input_stream.read(&mut client_buffer) {
                Ok(n) => {
                    if n == 0 {
                        process::exit(0);
                    }
                    else
                    {
                        io::stdout().write(&client_buffer).unwrap();
                        io::stdout().flush().unwrap();
                    }
                },
                Err(error) => println!("{}", error.to_string()),
            }
        }
    });

    let output_stream = &mut stream;
    let mut user_buffer = String::new();

    loop {
        io::stdin().read_line(&mut user_buffer).unwrap();

        output_stream.write(user_buffer.as_bytes()).unwrap();
        output_stream.flush().unwrap();
    }
}

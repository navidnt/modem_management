use std::os::unix::net::UnixDatagram;
use std::time::Duration;
use std::thread;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::process;

#[derive(Serialize, Deserialize)]
struct Message {
    sender_id: u32,
    message_id: u8,
    message_text: String,
}

fn create_message(sender_id: u32, message_id: u8, message_text: &str) -> Result<String> {
    let m = Message {
        sender_id,
        message_id,
        message_text: message_text.to_string(),
    };
    let j = serde_json::to_string(&m)?;
    Ok(j)
}


fn main() {


    let sender_thread = thread::spawn(move|| {
        thread::sleep(Duration::from_millis(80));
        
        let sender_id = process::id();
        let sock = UnixDatagram::unbound().unwrap();
        
        let path = "/data/local/tmp/sock1";
        let mut message_id = 0;
        loop {
            message_id = message_id + 1;
            let message_text = "at";
            let message = create_message(sender_id, message_id, message_text).unwrap();
            // Send the message to the socket
            match sock.send_to(message.as_bytes(), path) {
                Ok(_) => println!("Message sent successfully"),
                Err(err) => eprintln!("Error sending message: {}", err),
            }
            thread::sleep(Duration::from_millis(5000));
        }
    });

    let receiver_thread = thread::spawn(move|| {

        let sender_id = process::id();
        let sender_id_string = sender_id.to_string();
	    let path = "/data/local/tmp/sock".to_string() + &sender_id_string;
        let sock = match UnixDatagram::bind(&path) {
            Ok(sock) => sock,
            Err(e) => {
                println!("Couldn't bind: {:?}", e);
                return
            }
        };
    
        loop{
            
            println!("Connecting to socket {}", path);
            
    
            let mut buf = [0; 100];
            sock.recv(buf.as_mut_slice()).expect("recv function failed");
            for i in 0..100 {
                if buf[i] != 0 {
                    print!("{}", buf[i] as char);
                }
            }
            //fs::remove_file("/data/local/tmp/sock2").expect("Couldn't remove file");
            thread::sleep(Duration::from_millis(50));
        }

    });
    sender_thread.join().unwrap();
    receiver_thread.join().unwrap();
}

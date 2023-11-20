use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::{Duration};
use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::os::unix::net::UnixDatagram;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};


static CONT: AtomicBool = AtomicBool::new(true);
static IS_WAITING: AtomicBool = AtomicBool::new(false);

extern "C" fn handle_interrupt(_sig: libc::c_int) { // 1
    CONT.store(false, Ordering::SeqCst);
    println!("Sorry we didn't get the chance to finish");
    //exit(0);
}


#[derive(Serialize, Deserialize, Debug)]
struct Message {
    sender_id: u32,
    message_id: u8,
    message_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    sender_id: u32,
    message_id: u8,
    response_text: String,
}

fn create_response(sender_id: u32, message_id: u8, response_text: &str) -> Result<String> {
    let r = Response {
        sender_id,
        message_id,
        response_text: response_text.to_string(),
    };
    let j = serde_json::to_string(&r)?;
    Ok(j)
}


fn main() {
    let _c_str = Arc::new("");
    //fs::remove_file("/home/navid/kavoshcom/attach_modules/socket_folder/sock").expect("Couldn't remove file");
    unsafe { 
        libc::signal(libc::SIGINT, handle_interrupt as libc::sighandler_t); // 2
    }

    //let portname = CString::new("/dev/ttyUSB0").unwrap();
    

    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let (tx_sender_id, rx_sender_id) = mpsc::channel();
    let (tx_message_id, rx_message_id) = mpsc::channel();

    let thread1 = thread::spawn(move|| {        // thread1: unix socket listen and send to channel1
        
        let sock = match UnixDatagram::bind("/data/local/tmp/sock1") {
            Ok(sock) => sock,
            Err(e) => {
                println!("Couldn't bind: {:?}", e);
                return
            }
        };

        while CONT.load(Ordering::SeqCst) == true {
            println!("Thread 1 running...");
            
		
            let mut buf = [0; 1024];
	    sock.recv(buf.as_mut_slice()).expect("recv function failed");
 	    let mut command = String::new();
		
            for i in 0..1024{
  	    	if buf[i] != 0 {
			print!("{}", buf[i] as char);
			let added: String = format!("{}", buf[i] as char);
			let ss: &str = added.as_str();
			command.push_str(ss); 
		}
	    }
	    println!("");
	    tx1.send(command).unwrap();
        
            
            
            //fs::remove_file("/data/local/tmp/sock1").expect("Couldn't remove file");

           

            thread::sleep(Duration::from_secs(1));

        }


    });


    let thread2  = thread::spawn(move||{           // thread2: receive from channel1 and send to sock1
        

        while CONT.load(Ordering::SeqCst) == true {
            println!("Thread 2 running...");
            



            while IS_WAITING.load(Ordering::SeqCst) == true{
                thread::sleep(Duration::from_millis(200));
            }
            let received = rx1.recv().unwrap();
            println!("Thread 2: Got from channel 1: {}", received);
            
            //Parse JSON
            /****/
            let v: Value = serde_json::from_str(received.as_ref()).unwrap();
            println!("sender id: {} message_id: {} message_text: {}", v["sender_id"], v["message_id"], v["message_text"]);
            
	        let sock3 = UnixDatagram::unbound().unwrap();
	        sock3.send_to(v["message_text"].as_str().unwrap().as_bytes(), "/data/local/tmp/sock3").expect("send_to function failed");
            tx_message_id.send(v["message_id"].as_u64().unwrap() as u8).unwrap();
            tx_sender_id.send(v["sender_id"].as_u64().unwrap() as u32).unwrap();
            /****/

            IS_WAITING.store(true, Ordering::SeqCst);
	        //thread::sleep(Duration::from_millis(400));
            thread::sleep(Duration::from_secs(1));   
        }
    });
    
    let thread3 = thread::spawn(move || {           // thread3: receive from sock2 and send to channel2
        
        
        let sock4 = match UnixDatagram::bind("/data/local/tmp/sock4") {
            Ok(sock4) => sock4,
            Err(e) => {
                println!("Couldn't bind: {:?}", e);
                return
            }
        };

        while CONT.load(Ordering::SeqCst) == true {
            println!("Thread 3 running...");
            while IS_WAITING.load(Ordering::SeqCst) == false{
                thread::sleep(Duration::from_millis(200));
            }
                

            let mut resp = String::new();
            let mut buf1 = [0; 100];
            sock4.recv(buf1.as_mut_slice()).expect("recv function failed");
            let sender_id = rx_sender_id.recv().unwrap();
            let message_id = rx_message_id.recv().unwrap();

            print!("Thread 3: received from UnixSocket: ");
            for i in 0..100 {
                if buf1[i] != 0 {
                    print!("{}", buf1[i] as char);
                    let added: String = format!("{}", buf1[i] as char);
                    let ss: &str = added.as_str();
                    resp.push_str(ss);
                }
            }
            println!("");
            println!("Thread 3: received: length: {} message: {}", resp.len(), resp); 
                    
            tx2.send(create_response(sender_id, message_id, resp.as_str()).unwrap()).unwrap();
            //fs::remove_file("/data/local/tmp/sock4").expect("Couldn't remove file");
            IS_WAITING.store(false, Ordering::SeqCst);
            thread::sleep(Duration::from_secs(1));
        }
       // }

    });

    let thread4 = thread::spawn(move || {           // thread4: receive from channel2 and send to UnixSocket
        
        // ToDo: implement
        while CONT.load(Ordering::SeqCst) == true {
            println!("Thread 4 running...");
            let received = rx2.recv().unwrap();
            println!("Got from channel 2: {}", received);

            //Parse JSON
            /****/
            let v: Value = serde_json::from_str(received.as_ref()).unwrap();
            println!("sender id: {} message_id: {} message_text: {}", v["sender_id"], v["message_id"], v["message_text"]);
            let sender_id = v["sender_id"].as_u64().unwrap() as u32;
            let sender_id_string = sender_id.to_string();
	        /*let sock3 = UnixDatagram::unbound().unwrap();
	        sock3.send_to(v["message_text"].as_str().unwrap().as_bytes(), "/data/local/tmp/sock3").expect("send_to function failed");
            tx_message_id.send(v["message_id"].as_u64().unwrap() as u8).unwrap();
            tx_sender_id.send(v["sender_id"].as_u64().unwrap() as u32).unwrap();*/


            /****/
            let path = "/data/local/tmp/sock".to_string() + &sender_id_string;
            let sock = UnixDatagram::unbound().unwrap();
            sock.send_to(received.as_bytes(), &path).expect("send_to function failed");
            thread::sleep(Duration::from_secs(1));
        }   

    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    thread4.join().unwrap();

    
}


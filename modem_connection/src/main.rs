use std::thread;
use std::time::{Duration};
use std::sync::{mpsc, Arc};
//use std::io::stdin;
//use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::os::unix::net::UnixDatagram;
use std::fs;
//use std::ffi::CString;
//use std::os::unix::io::RawFd;
//use std::io::{/*Write,*/ Result};
//use libc::{termios, TCSANOW, B115200, O_RDWR, c_void/*, O_NOCTTY*/};

static CONT: AtomicBool = AtomicBool::new(true);
static IS_WAITING: AtomicBool = AtomicBool::new(false);

extern "C" fn handle_interrupt(_sig: libc::c_int) { // 1
    CONT.store(false, Ordering::SeqCst);
    println!("Sorry we didn't get the chance to finish");
    //exit(0);
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
            

            let mut buf = [0; 100];
            sock.recv(buf.as_mut_slice()).expect("recv function failed");
            let mut command = String::new();

            print!("Thread 1: received from UnixSocket 1: ");
            for i in 0..100 {
                if buf[i] != 0 {
                    print!("{}", buf[i] as char);
                    let added: String = format!("{}", buf[i] as char);
                    let ss: &str = added.as_str();
                    command.push_str(ss);
                }
            }
            println!("");
        
            
            
            //fs::remove_file("/data/local/tmp/sock1").expect("Couldn't remove file");

            tx1.send(command).unwrap();

            thread::sleep(Duration::from_secs(1));

        }


    });


    let thread2  = thread::spawn(move||{           // thread2: receive from channel1 and send to sock1
        
        // ToDo: implement

        /*while CONT.load(Ordering::SeqCst) == true {
            for received in rx1{
                println!("Got from channel 1: {}", received);
                while IS_WAITING.load(Ordering::SeqCst) == true{
                    thread::sleep(Duration::from_millis(200));
                }
                send_to_serial(fd, received);
                IS_WAITING.store(true, Ordering::SeqCst);    
            }
            thread::sleep(Duration::from_secs(1));
        }   */
        while CONT.load(Ordering::SeqCst) == true {
            println!("Thread 2 running...");
            /*for received in rx1{
                if CONT.load(Ordering::SeqCst) == false {
                    break;
                }
                println!("Got from channel 1: {}", received);
                while IS_WAITING.load(Ordering::SeqCst) == true{
                    thread::sleep(Duration::from_millis(200));
                }
                let cstring = CString::new(received).expect("CString::new failed");

                set_serial_attributes(fd).expect("set attributes failed");
                let _bytes_written = unsafe { libc::write(fd, cstring.as_ptr() as *const _, cstring.as_bytes_with_nul().len()) };
                IS_WAITING.store(true, Ordering::SeqCst);
                thread::sleep(Duration::from_secs(1));

                let sock3 = UnixDatagram::unbound().unwrap();
                sock3.send_to(received.as_bytes(), "/data/local/tmp/sock3").expect("send_to function failed");
                IS_WAITING.store(true, Ordering::SeqCst);
                thread::sleep(Duration::from_secs(1));
            }*/



            while IS_WAITING.load(Ordering::SeqCst) == true{
                thread::sleep(Duration::from_millis(200));
            }
            let received = rx1.recv().unwrap();
            println!("Thread 2: Got from channel 1: {}", received);

	        let sock3 = UnixDatagram::unbound().unwrap();
	        sock3.send_to(received.as_bytes(), "/data/local/tmp/sock3").expect("send_to function failed");
            IS_WAITING.store(true, Ordering::SeqCst);
	        //thread::sleep(Duration::from_millis(400));
            thread::sleep(Duration::from_secs(1));   
        }
    });
    
    let thread3 = thread::spawn(move || {           // thread3: receive from sock2 and send to channel2
        
        /*while CONT.load(Ordering::SeqCst) == true {
            
            if IS_WAITING.load(Ordering::SeqCst) {
                unsafe { libc::read(fd, c_str.as_ptr() as *mut c_void, 100) };
                IS_WAITING.store(false, Ordering::SeqCst);
                println!("Bytes Received from serial port: {}", c_str.len());
                let buf = c_str.as_bytes();
                tx2.send(buf).unwrap();
            }
            
            
            thread::sleep(Duration::from_secs(1));
        }*/
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
           
            tx2.send(resp).unwrap();
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
            let sock = UnixDatagram::unbound().unwrap();
            sock.send_to(received.as_bytes(), "/data/local/tmp/sock2").expect("send_to function failed");
            thread::sleep(Duration::from_secs(1));
        }   
        /*for received in rx2{
            println!("Thread 4 running...");
            if CONT.load(Ordering::SeqCst) == false {
                break;
            }
            println!("Got from channel 2: {}", received);
            let sock = UnixDatagram::unbound().unwrap();
            sock.send_to(received.as_bytes(), "/home/navid/kavoshcom/attach_modules/socket_folder/sock2").expect("send_to function failed");
            thread::sleep(Duration::from_secs(1));
        }*/
        


    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    thread4.join().unwrap();

    /*let sender = thread::spawn(move||{


        while CONT.load(Ordering::SeqCst) == true {
            let mut line = String::new();
            println!("next command:");
            stdin().read_line(&mut line).unwrap();
            tx1.send(line).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });


    let receiver = thread::spawn(move||{
        for received in rx1{
            if CONT.load(Ordering::SeqCst) == false {
                break;
            }
            println!("Got: {}", received);
            thread::sleep(Duration::from_secs(1));
        }
    });
*/
    //sender.join().unwrap();
    //receiver.join().unwrap();

    
}


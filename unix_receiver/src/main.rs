use std::time::{Duration};
use std::os::unix::net::UnixDatagram;
use std::thread;
//use std::env;
//use std::process;


//const PATH_FILE_IMEIs: &str = "/data/local/tmp/MainApp/.imei.ini";


/*struct k400_modem_t {

}


struct k400_first_message_t {
    IMEIs: [&'static str; 2],
    modems: [k400_modem_t; 11],   
}



fn arg_parser(args: &[String]) {
    if args.len() != 3 {
        eprintln!("usage: {} <loglevel> <modemORethernet>", &args[0]);
        process::exit(1);
    }
}


*/

fn main() {
/* 
    let args: Vec<String> = env::args().collect();
    arg_parser(&args);
*/

    let sock = match UnixDatagram::bind("/data/local/tmp/sock2") {
        Ok(sock) => sock,
        Err(e) => {
            println!("Couldn't bind: {:?}", e);
            return
        }
    };

    loop{
        
        println!("Connecting to socket...");
        

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
}

use std::os::unix::net::UnixDatagram;
use std::time::{Duration};
use std::thread;




fn main() {

    thread::sleep(Duration::from_millis(80));
    let sock = UnixDatagram::unbound().unwrap();
    sock.send_to(b"at", "/data/local/tmp/sock1").expect("send_to function failed");
    thread::sleep(Duration::from_millis(70));

/*
    let sock = UnixDatagram::unbound().unwrap();
    sock.send_to(b"AT+CPSI?", "/data/local/tmp/sock1").expect("send_to function failed");
    thread::sleep(Duration::from_millis(70));

    let sock = UnixDatagram::unbound().unwrap();
    sock.send_to(b"AT+CREG?", "/data/local/tmp/sock1").expect("send_to function failed");
    thread::sleep(Duration::from_millis(70));

    let sock = UnixDatagram::unbound().unwrap();
    sock.send_to(b"AT", "/data/local/tmp/sock1").expect("send_to function failed");
    thread::sleep(Duration::from_millis(70));

    let sock = UnixDatagram::unbound().unwrap();
    sock.send_to(b"AT", "/data/local/tmp/sock1").expect("send_to function failed");
    thread::sleep(Duration::from_millis(70));
*/

}

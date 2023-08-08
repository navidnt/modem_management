use std::os::unix::net::UnixDatagram;
use std::time::{Duration};
use std::thread;




fn main() {

    thread::sleep(Duration::from_millis(80));
    let sock = UnixDatagram::unbound().unwrap();
    sock.send_to(b"Hi\n", "/data/local/tmp/sock1").expect("send_to function failed");
    thread::sleep(Duration::from_millis(70));

    let sock = UnixDatagram::unbound().unwrap();
    sock.send_to(b"How\n", "/data/local/tmp/sock1").expect("send_to function failed");
    thread::sleep(Duration::from_millis(70));

    let sock = UnixDatagram::unbound().unwrap();
    sock.send_to(b"are\n", "/data/local/tmp/sock1").expect("send_to function failed");
    thread::sleep(Duration::from_millis(70));

    let sock = UnixDatagram::unbound().unwrap();
    sock.send_to(b"you\n", "/data/local/tmp/sock1").expect("send_to function failed");
    thread::sleep(Duration::from_millis(70));

    let sock = UnixDatagram::unbound().unwrap();
    sock.send_to(b"doing?\n", "/data/local/tmp/sock1").expect("send_to function failed");
    thread::sleep(Duration::from_millis(70));


}

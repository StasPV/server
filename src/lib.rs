use std::thread;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub fn get_local_ip() -> Result<IpAddr, std::io::Error>{
    let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    let local_addr = socket.local_addr()?;
    Ok(local_addr.ip())    
}
pub struct ThreadPool{
    workers: Vec<Worker>,
}

impl ThreadPool{
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        for id in 0..size{
            workers.push(Worker::new(id));
        }
        ThreadPool{
            workers
        }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static{

    }
}

struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker{
        let thread = thread::spawn(||{});
        Worker{
            id,
            thread,
        }
    }
}
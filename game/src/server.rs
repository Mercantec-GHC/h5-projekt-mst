use std::{
    io::{self, BufReader, Read, Write},
    net::TcpStream,
    slice::{from_raw_parts, from_raw_parts_mut},
    thread,
    time::Duration,
};

#[repr(C)]
enum ReqTy {
    Subscribe = 0,
}

#[repr(C)]
struct ReqHeader {
    ty: ReqTy,
}

#[repr(C)]
pub struct Measurement {
    pub angle: f64,
}

pub struct Server {
    stream: TcpStream,
}

impl Server {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            stream: TcpStream::connect("127.0.0.1:8889")?,
        })
    }

    pub fn subscribe(&mut self, cb: impl Fn(Measurement) -> ()) -> io::Result<()> {
        let req = ReqHeader {
            ty: ReqTy::Subscribe,
        };
        let req_as_bytes =
            unsafe { from_raw_parts((&req as *const _) as *const u8, size_of::<ReqHeader>()) };
        self.stream.write_all(req_as_bytes)?;

        loop {
            let mut packet = Measurement { angle: 0.0 };
            let packet_as_bytes = unsafe {
                from_raw_parts_mut(&mut packet as *mut _ as *mut u8, size_of::<Measurement>())
            };
            self.stream.read_exact(packet_as_bytes)?;
            cb(packet);
        }
    }
}

use std::{
    io::{Read, Write},
    net::TcpStream,
};

struct Server;
struct UnregisteredServer(TcpStream);
struct RegisteredServer(TcpStream);

impl Server {
    pub fn bind(addr: &str) -> UnregisteredServer {
        UnregisteredServer(TcpStream::connect(addr).unwrap())
    }
}

impl UnregisteredServer {
    pub fn register(mut self, id: u64) -> Result<RegisteredServer, UnregisteredServer> {
        self.0.write(b"register ").unwrap();
        self.0.write(id.to_string().as_bytes()).unwrap();
        self.0.write(b"\n").unwrap();

        let mut buf: [u8; 2] = [0, 0];
        self.0.read(&mut buf).unwrap();
        match &buf {
            b"OK" => Ok(RegisteredServer(self.0)),
            b"NO" => Err(self),
            _ => panic!("server error"),
        }
    }
}

struct Data {
    sway: f64,
}

struct Pump<'a>(&'a mut TcpStream, [u8; 4], usize);

impl<'a> Pump<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        Self(stream, [0, 0, 0, 0], 0)
    }
}

impl<'a> Iterator for Pump<'a> {
    type Item = std::io::Result<Data>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut bytes = self.0.bytes();
            let Some(byte) = bytes.next() else {
                break None;
            };
            let byte = match byte {
                Ok(byte) => byte,
                Err(err) => break Some(Err(err)),
            };
            self.1[self.2] = byte;
            self.2 += 1;
            if self.2 == 4 {
                let sway = u32::from_le_bytes(self.1) as f64;
                let sway = (sway - 64_000_000.0) / 2.0;
                break Some(Ok(Data { sway }));
            }
        }
    }
}

impl RegisteredServer {
    fn pump(&mut self) -> Pump<'_> {
        Pump::new(&mut self.0)
    }
}

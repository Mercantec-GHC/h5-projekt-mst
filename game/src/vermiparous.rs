use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

struct Server(TcpStream);

struct Data {
    x: f64,
    y: f64,
    z: f64,
    a: f64,
    b: f64,
    c: f64,
}

impl Server {
    pub fn bind(addr: &str) -> Server {
        Server(TcpStream::connect(addr).unwrap())
    }
    fn get_fallible(data: &[f64], idx: usize) -> Result<f64, String> {
        data.get(idx)
            .map(|idx| *idx)
            .ok_or_else(|| format!("protocol error: {idx}"))
    }
    pub fn read(&mut self) -> Result<Data, String> {
        let mut reader = BufReader::new(&mut self.0);
        let mut data = String::new();
        reader
            .read_line(&mut data)
            .map_err(|err| format!("io error: {}", err.to_string()))?;
        let data = data
            .split(",")
            .map(|x| x.parse::<f64>())
            .collect::<Result<Vec<f64>, _>>();
        let data = data.map_err(|err| format!("protocol error: {}", err.to_string()))?;

        Ok(Data {
            x: Self::get_fallible(&data, 0)?,
            y: Self::get_fallible(&data, 1)?,
            z: Self::get_fallible(&data, 2)?,
            a: Self::get_fallible(&data, 3)?,
            b: Self::get_fallible(&data, 4)?,
            c: Self::get_fallible(&data, 5)?,
        })
    }
}

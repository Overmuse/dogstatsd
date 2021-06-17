use crate::metric::Metric;
use std::io::Error;
use std::net::{ToSocketAddrs, UdpSocket};

pub struct Client<'a> {
    socket: UdpSocket,
    target: &'a str,
}

impl<'a> Client<'a> {
    pub fn new<T: ToSocketAddrs>(host_address: &T, target_address: &'a str) -> Result<Self, Error> {
        Ok(Self {
            socket: UdpSocket::bind(host_address)?,
            target: target_address,
        })
    }

    pub fn send<I, T>(&self, metric: Metric<I, T>) -> Result<usize, Error>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        self.socket.send_to(metric.to_bytes().as_ref(), self.target)
    }
}

impl Default for Client<'_> {
    fn default() -> Self {
        Self {
            socket: UdpSocket::bind("127.0.0.1:0").unwrap(),
            target: "127.0.0.1:8125",
        }
    }
}

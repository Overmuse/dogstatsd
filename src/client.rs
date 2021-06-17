use crate::metric::Metric;
use std::io::Error;
use tokio::net::{ToSocketAddrs, UdpSocket};

pub struct Client<'a> {
    socket: UdpSocket,
    target: &'a str,
}

impl<'a> Client<'a> {
    pub async fn new<T: ToSocketAddrs>(
        host_address: &T,
        target_address: &'a str,
    ) -> Result<Client<'a>, Error> {
        Ok(Self {
            socket: UdpSocket::bind(host_address).await?,
            target: target_address,
        })
    }

    pub async fn send<I, T>(&self, metric: Metric<'_, I, T>) -> Result<usize, Error>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        self.socket
            .send_to(metric.to_bytes().as_ref(), self.target)
            .await
    }
}

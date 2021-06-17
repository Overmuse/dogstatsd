use crate::Metric;
use std::io::Error;
use tokio::net::{ToSocketAddrs, UdpSocket};

pub struct Client {
    socket: UdpSocket,
}

impl Client {
    pub async fn new<T: ToSocketAddrs>(
        host_address: T,
        target_address: &str,
    ) -> Result<Client, Error> {
        let socket = UdpSocket::bind(host_address).await?;
        socket.connect(target_address).await?;
        Ok(Self { socket })
    }

    pub async fn send<'a>(&self, metric: Metric<'a>) -> Result<(), Error> {
        let bytes = metric.into_bytes();
        self.socket.send(&bytes).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_client() {
        let udp_receiver = UdpSocket::bind("127.0.0.1:8125").await.unwrap();
        let client = Client::new("127.0.0.1:1234", "127.0.0.1:8125")
            .await
            .unwrap();
        client.send(Metric::increase("test")).await.unwrap();
        udp_receiver.connect("127.0.0.1:1234").await.unwrap();
        let mut bytes_received: usize = 0;
        let mut buf = [0; 8];
        while bytes_received < 8 {
            bytes_received += udp_receiver.recv_from(&mut buf).await.unwrap().0;
        }
        assert_eq!(&buf, b"test:1|c");
    }
}

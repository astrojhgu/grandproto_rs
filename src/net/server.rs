use super::super::msg_def::msg::TrendMsg;

use super::codec::MsgDecoder;
use std;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;
//use tokio::codec::Decoder;
use tokio::net::{UdpSocket as TUdpSocket};
//use tokio_util::udp::UdpFramed;
//use futures::stream::StreamExt;
use std::future::Future;
use futures::future::Ready;
use futures::executor::block_on;


pub struct TrendServer {
    socket: UdpSocket,
    handlers: Vec<Box<dyn FnMut(&TrendMsg, std::net::SocketAddr) -> () + Send + Sync>>,
}

impl TrendServer {
    pub fn register_handler(
        &mut self,
        h: Box<dyn FnMut(&TrendMsg, std::net::SocketAddr) -> () + Send + Sync>,
    ) {
        self.handlers.push(h);
    }

    pub fn run(&mut self) {
        loop {
            let mut buf = vec![0_u8; 65536];
            let (s, addr) = self.socket.recv_from(&mut buf[..]).unwrap();
            assert!(s <= buf.len());
            unsafe { buf.set_len(s) };
            //eprintln!("{}", buf.len());

            if let Some(ref msg) = TrendMsg::from_byte_vec(buf) {
                for h in &mut self.handlers {
                    h(&msg, addr);
                }
            }
        }
    }

    pub fn wait_for(&mut self, dt: Option<Duration>) -> Option<TrendMsg> {
        let mut buf = vec![0_u8; 65536];
        self.socket
            .set_read_timeout(dt)
            .expect("set timeout failed");
        if let Ok((s, _addr)) = self.socket.recv_from(&mut buf[..]) {
            assert!(s <= buf.len());
            unsafe { buf.set_len(s) };
            //println!("{}", buf.len());
            TrendMsg::from_byte_vec(buf)
        } else {
            None
        }
    }

    pub fn new(addr: SocketAddr) -> Self {
        TrendServer {
            socket: UdpSocket::bind(&addr)
                .unwrap_or_else(|_| panic!("bind to addr {} failed", addr)),
            handlers: Vec::new(),
        }
    }
}

/*
pub fn create_async_server(
    addr: SocketAddr,
    handler: impl FnMut(Result<(TrendMsg, SocketAddr), std::io::Error>)->Ready<()>
)->impl Future<Output=()>
 {
    println!("port={}", addr.port());
    UdpFramed::new(
        block_on(TUdpSocket::bind(&addr)).expect("binding failed"),
        MsgDecoder {},
    )
    //.for_each(|(msg, _socket)| { Ok(())})
    .for_each(handler)
}
*/
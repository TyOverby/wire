use bchannel::{self, channel};
use std::net::{ToSocketAddrs, UdpSocket, SocketAddr};
use std::io::Result as IoResult;
use std::thread;
use std::marker::PhantomData;
use unreliable_message::network::AddrsContainer;
use unreliable_message::msgqueue::CompleteMessage;

use unreliable_message as unre;
use bincode;
use serialize;


pub struct Sender<T> {
    backing: bchannel::Sender<(Vec<u8>, AddrsContainer), unre::UnrError>,
    _phantom: PhantomData<T>
}

impl <T: serialize::Encodable> Sender<T> {
    fn new(channel: bchannel::Sender<(Vec<u8>, AddrsContainer), unre::UnrError>) -> Sender<T> {
        Sender {
            backing: channel,
            _phantom: PhantomData
        }
    }

    pub fn send<A: ToSocketAddrs>(&self, object: &T, addrs: A) -> bincode::EncodingResult<()> {
        let encoded = try!(bincode::encode(object, bincode::SizeLimit::Infinite));
        // TODO: make error returning way more general.
        let _ = self.backing.send((encoded, AddrsContainer::from_to_sock(addrs).unwrap()));
        Ok(())
    }

    pub fn close(self) {  }
}

pub type Receiver<T> = bchannel::Receiver<T, unre::UnrError>;

pub fn bind<A: ToSocketAddrs, I, O>(addr: A) -> IoResult<(Sender<I>,  Receiver<(SocketAddr, O)>)>
where A: ToSocketAddrs, I: serialize::Encodable, O: serialize::Decodable + Send + 'static {
    let message_size = 1024;
    let sock_1 = try!(UdpSocket::bind(addr));
    let sock_2 = try!(sock_1.try_clone());

    let back_send = unre::Sender::from_socket(sock_1, message_size, 1);
    let back_recv = unre::Receiver::from_socket(sock_2, message_size);

    let (in_s, in_r) = channel();
    let (out_s, out_r) = channel();

    thread::spawn(move || {
        let in_r = in_r;
        let mut back_send = back_send;
        loop {
            if back_send.is_queue_empty() && in_r.is_closed() {
                break;
            }

            for (bytes, from) in in_r.iter() {
                back_send.enqueue(bytes, from);
            }

            back_send.send_one();
            thread::sleep_ms(2);
        }
    });

    thread::spawn(move || {
        let out_s = out_s;
        let mut back_recv = back_recv;
        loop {
            match back_recv.poll() {
                Ok((from, CompleteMessage(id, bytes))) => {
                    match bincode::decode(&bytes[..]) {
                        // TODO: better error handling
                        Ok(obj) => out_s.send((from, obj)),
                        Err(e) => {
                            out_s.error(unre::UnrError::DecodingError(e));
                            break;
                        }
                    };
                }
                Err(e) => {
                    out_s.error(e);
                    break;
                }
            }
        }
    });

    Ok((Sender::new(in_s), out_r))
}

use bchannel::{self, channel};
use std::net::{ToSocketAddrs, UdpSocket, SocketAddr};
use std::collections::HashSet;
use std::io::Result as IoResult;
use std::thread;
use std::marker::PhantomData;
use unreliable_message::network::AddrsContainer;
use unreliable_message::msgqueue::CompleteMessage;

use unreliable_message as unre;
use bincode;
use serialize;

use bincode::rustc_serialize::{
    EncodingResult,
};

pub struct Sender<T> {
    backing: bchannel::Sender<(Vec<u8>, AddrsContainer), unre::UnrError>,
    _phantom: PhantomData<T>
}

impl <T> Clone for Sender<T> {
    fn clone(&self) -> Sender<T> {
        Sender {
            backing: self.backing.clone(),
            _phantom: PhantomData
        }
    }
}

impl <T: serialize::Encodable> Sender<T> {
    fn new(channel: bchannel::Sender<(Vec<u8>, AddrsContainer), unre::UnrError>) -> Sender<T> {
        Sender {
            backing: channel,
            _phantom: PhantomData
        }
    }

    pub fn send<A: ToSocketAddrs>(&self, object: &T, addrs: A) -> EncodingResult<()> {
        let encoded = try!(bincode::rustc_serialize::encode(object, bincode::SizeLimit::Infinite));
        // TODO: make error returning way more general.
        let _ = self.backing.send((encoded, AddrsContainer::from_to_sock(addrs).unwrap()));
        Ok(())
    }

    pub fn close(self) {  }
}

pub type Receiver<T> = bchannel::Receiver<T, unre::UnrError>;

pub fn bind<I, O, A: ToSocketAddrs>(addr: A) -> IoResult<(Sender<I>,  Receiver<(SocketAddr, O)>)>
where A: ToSocketAddrs, I: serialize::Encodable, O: serialize::Decodable + Send + 'static {
    let addrs_clonable = try!(AddrsContainer::from_to_sock(addr));

    let mut whitelist = HashSet::new();
    whitelist.extend(try!(addrs_clonable.to_socket_addrs()));

    let message_size = 1024;
    let sock_1 = try!(UdpSocket::bind(addrs_clonable.clone()));
    let sock_2 = try!(sock_1.try_clone());


    let back_send = unre::Sender::from_socket(sock_1, message_size, 1);
    let back_recv = unre::Receiver::from_socket(sock_2, message_size, None,
        unre::network::ReceiverFilter::Whitelist(whitelist));

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
                if back_send.enqueue(bytes, from).is_err() {
                    break;
                }
            }

            if back_send.send_one().is_err() {
                break;
            }
            thread::sleep(::std::time::Duration::from_millis(2));
        }
    });

    thread::spawn(move || {
        let out_s = out_s;
        let mut back_recv = back_recv;
        loop {
            match back_recv.poll() {
                Ok((from, CompleteMessage(_id, bytes))) => {
                    match bincode::rustc_serialize::decode(&bytes[..]) {
                        // TODO: better error handling
                        Ok(obj) => {
                            if out_s.send((from, obj)).is_err() {
                                break;
                            }
                        },
                        Err(e) => {
                            let _ = out_s.error(unre::UnrError::DecodingError(e));
                            break;
                        }
                    };
                }
                Err(e) => {
                    let _ = out_s.error(e);
                    break;
                }
            }
        }
    });

    Ok((Sender::new(in_s), out_r))
}

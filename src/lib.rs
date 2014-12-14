#![feature(unsafe_destructor)]

extern crate bincode;
extern crate serialize;
extern crate bchannel;

use std::io::net::tcp::{TcpStream, TcpListener, TcpAcceptor};
use std::io::{IoResult, IoError, BufferedReader, Listener, Acceptor, TimedOut};
use std::task::spawn;
use std::rc::{is_unique, Rc};
use std::sync::Mutex;

use serialize::{Decodable, Encodable};
use bincode::{EncoderWriter, DecoderReader};
pub use bchannel::{Sender, Receiver};
use bchannel::channel;

#[deriving(Clone)]
pub struct OutStream<T> {
    // wrap a mutex around a tcpstream so that we can get
    // writes from multiple threads.
    tcp_stream: Rc<Mutex<TcpStream>>,
}

impl <'a, T> OutStream<T>
where T: Encodable<EncoderWriter<'a, TcpStream>, IoError> {
    pub fn send(&mut self, m: &T) -> IoResult<()> {
        let mut stream = self.tcp_stream.lock();
        bincode::encode_into(m, stream.deref_mut())
    }
    pub fn send_all<'a, I: Iterator<&'a T>>(&mut self, mut i: I) ->
    Result<(), (&'a T, I, IoError)> {
        loop {
            match i.next() {
                None => return Ok(()),
                Some(x) => {
                    match self.send(x) {
                        Ok(()) => {},
                        Err(e) => return Err((x, i, e))
                    }
                }
            }
        }
    }
    pub fn close(self) {}
}

#[unsafe_destructor]
impl <T> Drop for OutStream<T> {
    fn drop(&mut self) {
        if is_unique(&self.tcp_stream) {
            let _ = self.tcp_stream.lock().close_write();
        }
    }
}

/// Connect to a server and open a send-receive pair.  See `upgrade` for more
/// details.
pub fn connect<'a, 'b, I, O>(ip: &str, port: u16) ->
IoResult<(Receiver<I, IoError>, OutStream<O>)>
where I: Send + Decodable<DecoderReader<'a, BufferedReader<TcpStream>>, IoError>,
      O: Encodable<EncoderWriter<'b, TcpStream>, IoError> {
    let path = format!("{}:{}", ip, port);
    Ok(upgrade(try!(TcpStream::connect(path[]))))
}

/// Starts listening for connections on this ip and port.
/// Returns:
/// * A receiver of Tcp stream objects.  It is recommended that you `upgrade`
///   these.
/// * A TcpAcceptor.  This can be used to close the listener from outside of the
///   listening thread.
pub fn listen(ip: &str, port: u16) ->
IoResult<(Receiver<TcpStream, IoError>, TcpAcceptor)> {
    let tcpl = try!(try!(TcpListener::bind((ip, port))).listen());
    let (sx, rx) = channel();

    let mut tcpl2 = tcpl.clone();
    spawn(proc() {
        loop {
            if sx.is_closed() {
                break;
            }
            match tcpl2.accept() {
                Ok(stream) => {
                    if sx.send(stream).is_err() {
                        break;
                    }
                }
                Err(IoError{kind: TimedOut, ..}) => {
                    continue;
                }
                Err(e) => {
                    let _  = sx.error(e);
                    break;
                }
            }
        }
    });
    Ok((rx, tcpl))
}

/// Upgrades a TcpStream to a Sender-Receiver pair that you can use to send and
/// receive objects automatically.  If there is an error decoding or encoding
/// values, that respective part is shut down.
pub fn upgrade<'a, 'b, I, O>(stream: TcpStream) ->
(Receiver<I, IoError>, OutStream<O>)
where I: Send + Decodable<DecoderReader<'a, BufferedReader<TcpStream>>, IoError>,
      O: Encodable<EncoderWriter<'b, TcpStream>, IoError> {
    (upgrade_reader(stream.clone()), upgrade_writer(stream))
}

fn upgrade_writer<'a, T>(stream: TcpStream) -> OutStream<T>
where T: Encodable<EncoderWriter<'a, TcpStream>, IoError> {
    OutStream {
        tcp_stream: Rc::new(Mutex::new(stream))
    }
}

fn upgrade_reader<'a, T>(stream: TcpStream) -> Receiver<T, IoError>
where T: Send + Decodable<DecoderReader<'a, BufferedReader<TcpStream>>, IoError> {
    let (in_snd, in_rec) = channel();

    spawn(proc() {
        let mut buffer = BufferedReader::new(stream);
        loop {
            match bincode::decode_from(&mut buffer) {
                Ok(a) => {
                    // Try to send, and if we can't,
                    // then the channel is closed.
                    if in_snd.send(a).is_err() {
                        break;
                    }
                },
                // if we can't decode, close the stream with an error.
                Err(e) => {
                    let _ = in_snd.error(e);
                    break;
                }
            }
        }
        let mut s1 = buffer.into_inner();
        let _ = s1.close_read();
    });
    in_rec
}

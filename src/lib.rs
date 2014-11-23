#![feature(if_let)]

extern crate bincode;
extern crate serialize;
extern crate bchannel;

use std::io::net::tcp::{TcpStream, TcpListener, TcpAcceptor};
use std::io::{IoResult, IoError, BufferedReader, Listener, Acceptor, TimedOut};
use std::task::spawn;

use serialize::{Decodable, Encodable};
use bincode::{EncoderWriter, DecoderReader};
use bchannel::{Sender, Receiver, channel};

/// Connect to a server and open a send-receive pair.  See `upgrade` for more
/// details.
pub fn connect<'a, 'b, I, O>(ip: &str, port: u16) ->
IoResult<(Receiver<I, IoError>, Sender<O, IoError>)>
where I: Send + Decodable<DecoderReader<'a, BufferedReader<TcpStream>>, IoError>,
      O: Send + Encodable<EncoderWriter<'b, TcpStream>, IoError> {
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
(Receiver<I, IoError>, Sender<O, IoError>)
where I: Send + Decodable<DecoderReader<'a, BufferedReader<TcpStream>>, IoError>,
      O: Send + Encodable<EncoderWriter<'b, TcpStream>, IoError> {
    let (in_snd, in_rec) = channel();
    let (out_snd, out_rec) = channel();

    let s1 = stream.clone();
    let s2 = stream.clone();

    // "in" thread.
    spawn(proc() {
        let mut buffer = BufferedReader::new(s1);
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
        let mut s1 = buffer.unwrap();
        let _ = s1.close_read();
    });

    // "out" thread.
    spawn(proc() {
        let mut s2 = s2;
        loop {
            if let Some(obj) = out_rec.recv_block() {
                match bincode::encode_into(&obj, &mut s2) {
                    Ok(()) => {},
                    Err(_) => {
                        // Erroring on encoding means we should just close.
                        break;
                    }
                }
            } else {
                // No more messages from this.
                break;
            }
        }
        let _ = s2.close_write();
    });

    (in_rec, out_snd)
}

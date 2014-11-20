extern crate bincode;
extern crate serialize;

use std::io::net::tcp::{TcpStream, TcpListener};
use std::io::{IoResult, IoError, BufferedReader, Listener, Acceptor};
use std::comm::{Sender, Receiver, channel};
use std::task::spawn;

use serialize::{Decodable, Encodable};

use bincode::{EncoderWriter, DecoderReader};

pub fn connect<'a, 'b, I, O>(ip: &str, port: u16) -> IoResult<(Receiver<I>, Sender<O>)>
where I: Send + Decodable<DecoderReader<'a, BufferedReader<TcpStream>>, IoError>,
      O: Send + Encodable<EncoderWriter<'b, TcpStream>, IoError> {
    let path = format!("{}:{}", ip, port);
    Ok(upgrade(try!(TcpStream::connect(path[]))))
}

pub fn listen(ip: &str, port: u16) -> IoResult<Receiver<TcpStream>> {
    let tcpl = try!(try!(TcpListener::bind((ip, port))).listen());
    let (sx, rx) = channel();
    spawn(proc() {
        let mut tcpl = tcpl;
        loop {
            let stream = tcpl.accept().unwrap();
            sx.send(stream);
        }
    });
    Ok(rx)
}

pub fn upgrade<'a, 'b, I, O>(stream: TcpStream) -> (Receiver<I>, Sender<O>)
where I: Send + Decodable<DecoderReader<'a, BufferedReader<TcpStream>>, IoError>,
      O: Send + Encodable<EncoderWriter<'b, TcpStream>, IoError> {
    let (in_snd, in_rec) = channel();
    let (out_snd, out_rec) = channel();

    let s1 = stream.clone();
    let s2 = stream.clone();

    // "in" thread.
    spawn(proc() {
        let mut s1 = BufferedReader::new(s1);
        loop {
            match bincode::decode_from(&mut s1) {
                Ok(a) => in_snd.send(a),
                Err(_) => return
            }
        }
    });
    // "out" thread.
    spawn(proc() {
        let mut s2 = s2;
        loop {
            let obj = out_rec.recv();
            match bincode::encode_into(&obj, &mut s2) {
                Ok(()) => {},
                Err(_) => return
            }
        }
    });

    (in_rec, out_snd)
}

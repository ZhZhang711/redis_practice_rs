use std::{string::FromUtf8Error, io::Cursor, io::Read};

use bytes::{Buf, Bytes};

pub enum FrameError {
    DecodeFailed,
    Incomplete,
}


impl From<FromUtf8Error> for FrameError {
    fn from(_: FromUtf8Error) -> Self {
        FrameError::DecodeFailed
    }
}

pub enum Frame {
    SET(String, String),
    GET(String),
    HSET(String, String, String),
    HGET(String, String),
    HGETALL(String),
}

impl Frame {
    pub fn parse_frame<T: Buf + AsRef<[u8]>>(buffer: &mut T) -> Result<Frame, FrameError> {
        let mut cur = &mut Cursor::new(&buffer);
        Frame::check_frame(&mut cur)?;
        let mut s = vec![0u8; cur.position() as usize - 1];
        buffer.advance(1);
        buffer.copy_to_slice(&mut s);
        let s = String::from_utf8(s)?;
        let mut chunk = s.split_whitespace();
        match chunk.next() {
            Some("SET") => {
                let tmp = chunk.take(2).collect::<Vec<_>>();
                if tmp.len() < 2 {
                    Err(FrameError::Incomplete)
                } else {
                    Ok(Frame::SET(tmp[0].to_string(), tmp[1].to_string()))
                }
            }
            Some("GET") => {
                if let Some(k) = chunk.next() {
                    Ok(Frame::GET(k.to_string()))
                } else {
                    Err(FrameError::Incomplete)
                }
            }
            Some("HSET") => {
                let tmp = chunk.take(3).collect::<Vec<_>>();
                if tmp.len() < 3 {
                    Err(FrameError::Incomplete)
                } else {
                    Ok(Frame::HSET(tmp[0].to_string(), tmp[1].to_string(), tmp[2].to_string()))
                }
            }
            _ => panic!()
        }
    }

    fn check_frame<T: AsRef<[u8]>>(buffer: &mut Cursor<T>) -> Result<(), FrameError> {
        if !buffer.has_remaining() {
            return Err(FrameError::Incomplete);
        }
        let sz = buffer.get_u8() as usize;
        if !buffer.remaining() < sz {
            return Err(FrameError::Incomplete);
        }
        buffer.advance(sz);
        Ok(())
    }
}

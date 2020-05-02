use crate::message::Message;
use std::io::Result;

pub trait Storage {
    fn write(msgs: Vec<Message>);
    fn read_at(topic: &str, offset: u64);
    fn read_from_to(topic: &str, from: u64, to: u64);
}

pub struct Rocksdb {

}

impl Storage for Rocksdb {
    fn write(msgs: Vec<Message>) {
        unimplemented!()
    }

    fn read_at(topic: &str, offset: u64) {
        unimplemented!()
    }

    fn read_from_to(topic: &str, from: u64, to: u64) {
        unimplemented!()
    }
}
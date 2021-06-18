use crate::Tag;
use bytes::{Bytes, BytesMut};

enum Type<'a> {
    Count(isize),
    Increase,
    Decrease,
    Gauge(&'a str),
    Histogram(&'a str),
    Distribution(&'a str),
    Set(&'a str),
}

pub struct Metric<'a> {
    frame_type: Type<'a>,
    message: &'a str,
    tags: Vec<Tag>,
}

impl<'a> Metric<'a> {
    fn new(frame_type: Type<'a>, message: &'a str) -> Self {
        Self {
            frame_type,
            message,
            tags: vec![],
        }
    }

    pub fn increase(message: &'a str) -> Self {
        Self::new(Type::Increase, message)
    }

    pub fn decrease(message: &'a str) -> Self {
        Self::new(Type::Decrease, message)
    }

    pub fn count(count: isize, message: &'a str) -> Self {
        Self::new(Type::Count(count), message)
    }

    pub fn gauge(value: &'a str, message: &'a str) -> Self {
        Self::new(Type::Gauge(value), message)
    }

    pub fn histogram(value: &'a str, message: &'a str) -> Self {
        Self::new(Type::Histogram(value), message)
    }

    pub fn distribution(value: &'a str, message: &'a str) -> Self {
        Self::new(Type::Distribution(value), message)
    }

    pub fn set(value: &'a str, message: &'a str) -> Self {
        Self::new(Type::Set(value), message)
    }

    pub fn add_tag<T: ToString>(mut self, tag: T) -> Self {
        self.tags.push(Tag::Single(tag.to_string()));
        self
    }

    pub fn add_key_value<K: ToString, V: ToString>(mut self, key: K, val: V) -> Self {
        self.tags
            .push(Tag::KeyValue(key.to_string(), val.to_string()));
        self
    }

    pub(crate) fn into_bytes(self) -> Bytes {
        let mut buf;
        match self.frame_type {
            Type::Count(count) => {
                buf = BytesMut::with_capacity(3 + self.message.len() + 8);
                buf.extend_from_slice(self.message.as_bytes());
                buf.extend_from_slice(b":");
                buf.extend_from_slice(count.to_string().as_bytes());
                buf.extend_from_slice(b"|c");
            }
            Type::Increase => {
                buf = BytesMut::with_capacity(self.message.len() + 4);
                buf.extend_from_slice(self.message.as_bytes());
                buf.extend_from_slice(b":1|c");
            }
            Type::Decrease => {
                buf = BytesMut::with_capacity(self.message.len() + 5);
                buf.extend_from_slice(self.message.as_bytes());
                buf.extend_from_slice(b":-1|c");
            }
            Type::Gauge(val) => {
                buf = BytesMut::with_capacity(3 + self.message.len() + val.len());
                buf.extend_from_slice(self.message.as_bytes());
                buf.extend_from_slice(b":");
                buf.extend_from_slice(val.as_bytes());
                buf.extend_from_slice(b"|g");
            }
            Type::Histogram(val) => {
                buf = BytesMut::with_capacity(3 + self.message.len() + val.len());
                buf.extend_from_slice(self.message.as_bytes());
                buf.extend_from_slice(b":");
                buf.extend_from_slice(val.as_bytes());
                buf.extend_from_slice(b"|h");
            }
            Type::Distribution(val) => {
                buf = BytesMut::with_capacity(3 + self.message.len() + val.len());
                buf.extend_from_slice(self.message.as_bytes());
                buf.extend_from_slice(b":");
                buf.extend_from_slice(val.as_bytes());
                buf.extend_from_slice(b"|d");
            }
            Type::Set(val) => {
                buf = BytesMut::with_capacity(3 + self.message.len() + val.len());
                buf.extend_from_slice(self.message.as_bytes());
                buf.extend_from_slice(b":");
                buf.extend_from_slice(val.as_bytes());
                buf.extend_from_slice(b"|s");
            }
        }
        let mut tags_iter = self.tags.into_iter();
        let mut next_tag = tags_iter.next();

        if next_tag.is_some() {
            buf.extend_from_slice(b"|#");
        }

        while next_tag.is_some() {
            let tag = next_tag.unwrap();
            match tag {
                Tag::Single(value) => {
                    buf.extend_from_slice(value.as_bytes());
                }
                Tag::KeyValue(key, value) => {
                    buf.extend_from_slice(key.as_bytes());
                    buf.extend_from_slice(b":");
                    buf.extend_from_slice(value.as_bytes());
                }
            }
            next_tag = tags_iter.next();

            if next_tag.is_some() {
                buf.extend_from_slice(b",");
            }
        }

        buf.freeze()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_into_bytes() {
        assert_eq!(Metric::increase("test").into_bytes().as_ref(), b"test:1|c");
        assert_eq!(Metric::decrease("test").into_bytes().as_ref(), b"test:-1|c");
        assert_eq!(Metric::count(2, "test").into_bytes().as_ref(), b"test:2|c");
        assert_eq!(
            Metric::gauge("1.2", "test").into_bytes().as_ref(),
            b"test:1.2|g"
        );
        assert_eq!(
            Metric::histogram("1.2", "test").into_bytes().as_ref(),
            b"test:1.2|h"
        );
        assert_eq!(
            Metric::distribution("1.2", "test").into_bytes().as_ref(),
            b"test:1.2|d"
        );
        assert_eq!(
            Metric::set("1.2", "test").into_bytes().as_ref(),
            b"test:1.2|s"
        );

        let val = String::from("b");
        assert_eq!(
            Metric::increase("test")
                .add_key_value("a", val)
                .add_tag("c")
                .into_bytes()
                .as_ref(),
            b"test:1|c|#a:b,c"
        );
    }
}

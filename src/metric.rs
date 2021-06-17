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

pub struct Metric<'a, I, T>
where
    I: IntoIterator<Item = T>,
    T: AsRef<str>,
{
    frame_type: Type<'a>,
    message: &'a str,
    tags: I,
}

impl<'a, I, T> Metric<'a, I, T>
where
    I: IntoIterator<Item = T>,
    T: AsRef<str>,
{
    pub fn increase(message: &'a str, tags: I) -> Self {
        Self {
            frame_type: Type::Increase,
            message,
            tags,
        }
    }

    pub fn decrease(message: &'a str, tags: I) -> Self {
        Self {
            frame_type: Type::Decrease,
            message,
            tags,
        }
    }

    pub fn count(count: isize, message: &'a str, tags: I) -> Self {
        Self {
            frame_type: Type::Count(count),
            message,
            tags,
        }
    }

    pub fn gauge(value: &'a str, message: &'a str, tags: I) -> Self {
        Self {
            frame_type: Type::Gauge(value),
            message,
            tags,
        }
    }

    pub fn histogram(value: &'a str, message: &'a str, tags: I) -> Self {
        Self {
            frame_type: Type::Histogram(value),
            message,
            tags,
        }
    }

    pub fn distribution(value: &'a str, message: &'a str, tags: I) -> Self {
        Self {
            frame_type: Type::Distribution(value),
            message,
            tags,
        }
    }

    pub fn set(value: &'a str, message: &'a str, tags: I) -> Self {
        Self {
            frame_type: Type::Set(value),
            message,
            tags,
        }
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
            buf.extend_from_slice(next_tag.unwrap().as_ref().as_bytes());
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
        let v: &[&str] = &[];
        assert_eq!(
            Metric::increase("test", v).into_bytes().as_ref(),
            b"test:1|c"
        );
        assert_eq!(
            Metric::decrease("test", v).into_bytes().as_ref(),
            b"test:-1|c"
        );
        assert_eq!(
            Metric::count(2, "test", v).into_bytes().as_ref(),
            b"test:2|c"
        );
        assert_eq!(
            Metric::gauge("1.2", "test", v).into_bytes().as_ref(),
            b"test:1.2|g"
        );
        assert_eq!(
            Metric::histogram("1.2", "test", v).into_bytes().as_ref(),
            b"test:1.2|h"
        );
        assert_eq!(
            Metric::distribution("1.2", "test", v).into_bytes().as_ref(),
            b"test:1.2|d"
        );
        assert_eq!(
            Metric::set("1.2", "test", v).into_bytes().as_ref(),
            b"test:1.2|s"
        );
        assert_eq!(
            Metric::increase("test", &["a:b", "c"])
                .into_bytes()
                .as_ref(),
            b"test:1|c|#a:b,c"
        );
    }
}

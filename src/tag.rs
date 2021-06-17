use std::borrow::Cow;

pub enum Tag<'a> {
    Single(Cow<'a, str>),
    KeyValue(Cow<'a, str>, Cow<'a, str>),
}

impl<'a> From<&'a str> for Tag<'a> {
    fn from(s: &'a str) -> Tag<'a> {
        Tag::Single(Cow::Borrowed(s))
    }
}

impl<'a> From<(&'a str, &'a str)> for Tag<'a> {
    fn from(s: (&'a str, &'a str)) -> Tag<'a> {
        Tag::KeyValue(Cow::Borrowed(s.0), Cow::Borrowed(s.1))
    }
}

impl<'a> From<Tag<'a>> for Cow<'a, str> {
    fn from(tag: Tag<'a>) -> Cow<'a, str> {
        match tag {
            Tag::Single(single) => single,
            Tag::KeyValue(key, value) => {
                let mut out = key.to_string();
                out.push(':');
                out.push_str(value.as_ref());
                Cow::Owned(out)
            }
        }
    }
}

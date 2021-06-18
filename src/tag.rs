pub enum Tag {
    Single(String),
    KeyValue(String, String),
}

impl<'a> From<Tag> for String {
    fn from(tag: Tag) -> String {
        match tag {
            Tag::Single(single) => single,
            Tag::KeyValue(mut key, value) => {
                key.push(':');
                key.push_str(value.as_ref());
                key
            }
        }
    }
}

use std::collections::HashMap;
use crate::http::ParseError;
use crate::http::request::get_next_word;

#[derive(Debug)]
pub struct Headers<'buf_lifetime> {
    data: HashMap<&'buf_lifetime str, &'buf_lifetime str>,
}

impl<'buf_lifetime> From<&'buf_lifetime str> for Headers<'buf_lifetime> {
    fn from(s: &'buf_lifetime str) -> Self {
        let null_terminator: String = '\0'.to_string();
        let mut data = HashMap::new();

        for sub_str in s.split('\n') {

            if let Some(index) = sub_str.find(' ') {
                let key = sub_str[..index].trim();
                let val = sub_str[index + 1..].trim();

                if key == null_terminator || val == null_terminator {
                    break;
                }

                data.insert(key, val);
            }
        }
        dbg!(&data);

        Headers {
            data,
        }
    }
}
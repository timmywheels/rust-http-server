use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf_lifetime> {
    data: HashMap<&'buf_lifetime str, Value<'buf_lifetime>>,
}

#[derive(Debug)]
pub enum Value<'buf_lifetime> {
    Single(&'buf_lifetime str),
    Multiple(Vec<&'buf_lifetime str>)
}

impl<'buf_lifetime> QueryString<'buf_lifetime> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf_lifetime> From<&'buf_lifetime str> for QueryString<'buf_lifetime> {
    fn from(s: &'buf_lifetime str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        *existing = Value::Multiple(vec![prev_val, val]);
                    },
                    Value::Multiple(vec) => vec.push(val)
                })
                .or_insert(Value::Single(val));

        }
        dbg!(&data);
        QueryString {
            data
        }
    }
}
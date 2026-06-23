use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl <'buf> QueryString<'buf> {
    
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for pair in s.split('&') {
            let mut key_value = pair.splitn(2, '=');
            if let Some(key) = key_value.next() {
                let value = key_value.next().unwrap_or("");
                data.entry(key)
                    .and_modify(|existing| {
                        match existing {
                            Value::Single(existing_value) => {
                                *existing = Value::Multiple(vec![existing_value, value]);
                            }
                            Value::Multiple(vec) => {
                                vec.push(value);
                            }
                        }
                    })
                    .or_insert(Value::Single(value));
            }
        }

        QueryString { data }
    }
}
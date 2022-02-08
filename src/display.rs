use serde_json::Value;

pub trait DisplayJson {
    fn json_stdout(&self, key: &str);
}

impl DisplayJson for serde_json::Value {
    fn json_stdout(&self, key: &str) {
        fn operations(value: &Value) {
            match value {
                Value::Null => println!("Null"),
                Value::Bool(x) => println!("{}", x),
                Value::Number(x) => println!("{}", x),
                Value::String(x) => println!("{}", x),
                Value::Array(x) => x.iter().map(operations).collect::<()>(),
                Value::Object(x) => x.values().map(operations).collect::<()>(),
            };
        }
        operations(&self[key])
    }
}

use serde_json::Value;
use std::hash::{Hash, Hasher};

pub fn hash_f64<H: Hasher>(f: &f64, state: &mut H) {
    f.to_bits().hash(state);
}

pub fn hash_opt_f64<H: Hasher>(opt: &Option<f64>, state: &mut H) {
    if let Some(f) = opt {
        true.hash(state);
        hash_f64(f, state);
    } else {
        false.hash(state);
        hash_f64(&0.0, state);
    }
}

pub fn hash_serde_json_value<H: Hasher>(v: &Value, state: &mut H) {
    match v {
        Value::Null => {
            0.hash(state);
        }
        Value::Bool(b) => {
            1.hash(state);
            b.hash(state);
        }
        Value::Number(n) => {
            2.hash(state);
            if let Some(i) = n.as_i64() {
                i.hash(state);
            } else if let Some(u) = n.as_u64() {
                u.hash(state);
            } else if let Some(f) = n.as_f64() {
                hash_f64(&f, state);
            } else {
                panic!("Unknown JSON number type");
            }
        }
        Value::String(s) => {
            3.hash(state);
            s.hash(state);
        }
        Value::Array(a) => {
            4.hash(state);
            for v in a {
                hash_serde_json_value(v, state);
            }
        }
        Value::Object(o) => {
            5.hash(state);
            for (k, v) in o {
                k.hash(state);
                hash_serde_json_value(v, state);
            }
        }
    }
}

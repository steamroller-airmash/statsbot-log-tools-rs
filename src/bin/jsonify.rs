
extern crate statslib;
extern crate serde;
extern crate serde_json;

use statslib::*;

use serde::Serializer;
use serde::ser::SerializeSeq;

use std::io::{BufReader, BufRead, stdout};
use std::fs::File;
use std::env::args;
use std::collections::HashMap;

use self::RecordValue::*;
use serde_json::{Value, Number};

fn serialize_record<'a>(r: &RecordValue<'a>) -> Value {
    match r {
        &Str(s) => Value::String(s.to_owned()),
        &Int(i) => Value::Number(i.into()),
        &Num(n) => Value::Number(Number::from_f64(n).unwrap()),
        Array(x) => {
            let mut vals = vec![];
            for v in x {
                vals.push(serialize_record(v));
            }
            Value::Array(vals)
        }
    }
}

fn main() {
	let args: Vec<String> = args().collect();

	if args.len() < 2 {
		println!("Usage: jsonify <logfile>");
	}

	let file = File::open(args[1].clone()).unwrap();
    let out = stdout();
    let mut ser = serde_json::Serializer::new(out);
    let mut seq = ser.serialize_seq(None).unwrap();

	for line in BufReader::new(file).lines() {
		let ln = line.unwrap();
		let result = parse(&ln);

		if result.is_err() {
			//println!("{}", result.unwrap_err().message);
			continue;
		}
        let result = result.unwrap();
        let mut message: HashMap<&str, Value> = HashMap::default();

        message.clear();

        message.insert("c", Value::String(result.tag.to_owned()));
        
        for (k, v) in result.entries {
            message.insert(k, serialize_record(&v));
        }

        seq.serialize_element(&message).unwrap();
	}

    seq.end().unwrap();
}
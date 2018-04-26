
extern crate statslib;

use statslib::*;

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env::args;
use std::vec::Vec;
use std::collections::HashMap;

fn main() {
	let args: Vec<String> = args().collect();

	if args.len() < 2 {
		println!("Usage: avgping <logfile>");
		return;
	}

	let file = File::open(args[1].clone()).unwrap();

	let mut players: HashMap<i64, String> = HashMap::new();

	for line in BufReader::new(file).lines() {
		let ln = line.unwrap();
		let result = parse(&ln);

		if result.is_err() {
			//println!("{}", result.unwrap_err().message);
			continue;
		}
		let record = result.unwrap();

		if record.tag == "PLAYER_NEW" {
			let id = match record.entries["id"] {
				RecordValue::Int(val) => val,
				_ => panic!("id was not an int")
			};
			let name = match record.entries["name"] {
				RecordValue::Str(val) => val,
				_ => panic!("name was not a string")
			};
		
			players.insert(id, name.to_string());
		}
	}

	let mut names: Vec<(i64, String)> = players.into_iter().collect();

	names.sort_by(|&(a, _), &(b, _)| a.cmp(&b));

	for (id, name) in names {
		println!("{}: {}", id, name);
	}
}
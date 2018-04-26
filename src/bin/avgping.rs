
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

	let mut pings: HashMap<i64, (i64, i64)> = HashMap::new();

	for line in BufReader::new(file).lines() {
		let ln = line.unwrap();
		let result = parse(&ln);

		if result.is_err() {
			//println!("{}", result.unwrap_err().message);
			continue;
		}
		let record = result.unwrap();

		if record.tag == "PLAYER_DETAILED_SCORE" {
			let id = match record.entries["id"] {
				RecordValue::Int(val) => val,
				_ => panic!("id was not an int")
			};
			let ping = match record.entries["ping"] {
				RecordValue::Int(val) => val,
				_ => panic!("ping was not an int")
			};
		
			if !pings.contains_key(&id) {
				pings.insert(id, (ping, 1));
			}
			else {
				(*pings.get_mut(&id).unwrap()).0 += ping;
				(*pings.get_mut(&id).unwrap()).1 += 1;
			}
		}
	}

	for (id, &(ping, cnt)) in pings.iter() {
		println!("{}: {}", id, ping / cnt);
	}
}
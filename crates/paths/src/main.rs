use std::{env::current_dir, path::PathBuf};
use structopt::{self, StructOpt};

use rusqlite::{Connection, Result};

use libs::{paths::get_state_dir, queries};

#[derive(Debug, StructOpt)]
#[structopt(name = "Pathman options", about = "All of the options for pathman")]
pub struct Opt {
	#[structopt()]
	dir: Option<String>,
	#[structopt(short, long)]
	add: Option<PathBuf>,
	#[structopt(short, long)]
	dump: bool,
	#[structopt(long)]
	clear_history: bool,
	#[structopt(short, long)]
	init: bool,
}

fn main() -> Result<()> {
	let opt = Opt::from_args();

	let state_dir = get_state_dir().unwrap();
	let mut db_conn = Connection::open(state_dir.join("pathman.db"))?;
	queries::init(&mut db_conn)?;

	if let Some(dir) = opt.dir {
		match queries::dir::find(&db_conn, &dir) {
			Ok(result) => println!("{}", &result),
			Err(_) => println!("{}", &dir),
		};
	} else if let Some(dir_path) = opt.add {
		let dir_path = current_dir().unwrap().join(dir_path);

		if let Ok(normalized_dir) = dir_path.canonicalize() {
			let normalized_dir = normalized_dir.to_str().unwrap();
			queries::dir::upsert(&db_conn, normalized_dir)?;
		}
	} else if opt.clear_history {
		queries::dir::clear_history(&db_conn)?;
	} else if opt.dump {
		let dump = queries::dir::get_dump(&db_conn)?;
		for dump_row in dump {
			println!("{:#?}", dump_row);
		}
	}

	Ok(())
}
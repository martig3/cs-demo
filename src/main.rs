use std::{fs::File, io::BufReader};

use demo::{Error, EventHandler, events::*, parse_dem_file};

#[derive(Default)]
struct Handler {}

impl EventHandler for Handler {
    fn on_string_cmd(&self, event: &CNETMsg_StringCmd) {
        println!("{:?}", event);
    }
}

fn main() -> Result<(), Error> {
    let file = File::open("example.dem")?;
    let mut reader = BufReader::new(file);

    let dispatcher = Handler::default();
    parse_dem_file(&mut reader, &dispatcher)?;

    Ok(())
}

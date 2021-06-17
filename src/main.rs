use std::{fs::File, io::BufReader};

use demo::{Error, events::*, parse_dem_file};

struct NoOpHandler;
impl EventHandler for NoOpHandler {}
impl UserMessageEventHandler for NoOpHandler {
    fn on_damage(&self, event: &CCSUsrMsg_Damage) -> Result<(),Error> {
        println!("Damage {:?}", event);
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let file = File::open("example.dem")?;
    let mut reader = BufReader::new(file);

    let dispatcher = UserMessageDecoder(NoOpHandler);
    parse_dem_file(&mut reader, &dispatcher)?;

    Ok(())
}

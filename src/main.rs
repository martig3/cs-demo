use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;
use std::iter::FromIterator;
use std::{fs::File, io::BufReader};

use demo::{events::*, parse_dem_file, Error};

#[derive(Debug)]
struct PlayerHurt {
    user_id: Option<i32>,
    armor: Option<i32>,
    weapon: Option<String>,
    hitgroup: Option<i32>,
    dmg_health: Option<i32>,
    dmg_armor: Option<i32>,
    health: Option<i32>,
    attacker: Option<u64>,
}

#[derive(Default)]
struct NoOpHandler {
    descriptors: RefCell<Option<Vec<CSVCMsg_GameEventList_descriptor_t>>>,
    player_hurt_events: RefCell<Vec<PlayerHurt>>,
}

#[derive(Default)]
struct GameEventHandler {}

impl EventHandler for NoOpHandler {
    fn on_game_event_list(&self, event: &CSVCMsg_GameEventList) -> Result<(), Error> {
        *self.descriptors.borrow_mut() = Some(Vec::from_iter(
            event.get_descriptors().into_iter().map(|d| d.to_owned()),
        ));
        Ok(())
    }

    fn on_game_event(&self, event: &CSVCMsg_GameEvent) -> Result<(), Error> {
        let event_id = event.get_eventid();
        // println!("{:?}", event);
        let event_keys = event.get_keys();
        if let Some(descriptors) = &*self.descriptors.borrow() {
            if let Some(descriptors) = descriptors.get(event_id as usize) {
                let descriptor_name = descriptors.get_name();
                let descriptor_keys = descriptors.get_keys();
                let mut map = HashMap::with_capacity(descriptor_keys.len());
                for (i, v) in descriptor_keys.iter().enumerate() {
                    map.insert(v.get_name(), event_keys[i].clone());
                }
                if descriptor_name == "player_hurt" {
                    let ph = PlayerHurt {
                        user_id: Some(map["userid"].get_val_short()),
                        weapon: Some(map["weapon"].get_val_string().to_string()),
                        hitgroup: Some(map["hitgroup"].get_val_short()),
                        dmg_health: Some(map["dmg_health"].get_val_short()),
                        armor: Some(map["dmg_armor"].get_val_short()),
                        dmg_armor: Some(map["dmg_armor"].get_val_short()),
                        health: Some(map["health"].get_val_short()),
                        attacker: Some(map["attacker"].get_val_short().try_into().unwrap()),
                    };
                    println!("{:?}", &ph);
                    self.player_hurt_events.borrow_mut().push(ph);
                }
            }
        }
        Ok(())
    }
}

impl UserMessageEventHandler for NoOpHandler {}

fn main() -> Result<(), Error> {
    let file = File::open("test.dem")?;
    let mut reader = BufReader::new(file);

    let dispatcher = UserMessageDecoder(NoOpHandler::default());
    parse_dem_file(&mut reader, &dispatcher)?;
    println!("size: {}", &dispatcher.0.player_hurt_events.borrow().len());

    Ok(())
}

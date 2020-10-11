extern crate serde_json;

use std::fs;
use std::io;

use serde::de::DeserializeOwned;
use serde_json::Map;
use serde_json::Value as JsonValue;

use crate::datasets::model::BasicMon;
use crate::datasets::model::Jokes;
use crate::datasets::model::Logos;
use crate::datasets::model::MonVec;
use crate::datasets::model::PickupLines;
use crate::datasets::model::Roasts;
use crate::datasets::model::WaifuData;
use crate::datasets::model::YoDataset;


fn read_file(path: String) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn parsed_json<T: DeserializeOwned>(path: String) -> T {
    let res = match read_file(String::from(&path)) {
        Ok(result) => result,
        Err(error) => panic!("Problem opening the file named {}: {:?}", path, error),
    };
    let resjs: T = match serde_json::from_str(&res) {
        Ok(result) => result,
        Err(error) => panic!("Error with{}", error),
    };
    resjs
}

pub fn logodata() -> Logos {
    let logo_data: Logos = parsed_json::<Logos>(String::from("src/data/logos.json"));
    logo_data
}

pub fn yomamadata() -> YoDataset {
    let post_data: YoDataset = parsed_json::<YoDataset>(String::from("src/data/jokes.json"));
    post_data
}

pub fn jokedata() -> Jokes {
    let post_data: Jokes = parsed_json::<Jokes>(String::from("src/data/shortjokes.json"));
    post_data
}

pub fn waifudata() -> WaifuData {
    let wd: WaifuData = parsed_json::<WaifuData>(String::from("src/data/finalwaifu.json"));
    wd
}

pub fn linegen() -> PickupLines {
    let lines: PickupLines = parsed_json::<PickupLines>(String::from("src/data/pickuplines.json"));
    lines
}

pub fn mondata() -> MonVec {
    let pokemon: JsonValue = parsed_json::<JsonValue>(String::from("src/data/pokemons.json"));
    let obj: Map<String, JsonValue> = pokemon.as_object().unwrap().clone();
    let mut vec = Vec::new();
    for item in obj.values() {
        let top: BasicMon = match serde_json::from_str(&item.to_string()) {
            Ok(res) => res,
            Err(error) => panic!("{}", error),
        };
        vec.push(top)
    }
    let final_mons: MonVec = MonVec { list: vec };
    final_mons
}

pub fn roasts() -> Roasts {
    let text = match read_file(String::from("src/data/roasts.txt")) {
        Ok(file) => file,
        Err(err) => panic!("Roasts no parse {}", err),
    };
    let mut text_vex: Vec<String> = Vec::new();
    let lines = text.lines();
    for line in lines {
        text_vex.push(line.to_string());
    }
    let roasts: Roasts = Roasts { list: text_vex };
    roasts
}

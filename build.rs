extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::collections::HashMap;
use std::path::Path;

struct HanjaDictEntry {
    hangeul: char,
    hanja: char,
    desc: String,
}

impl HanjaDictEntry {
    pub fn get_value_string(&self) -> String {
        let mut s = String::from("('");
        s.push(self.hanja);
        s.push('\'');
        s.push(',');
        s.push('\"');
        s.push_str(&self.desc);
        s.push('\"');
        s.push(')');
        return s;
    }
}

fn parse_dict_line(line: io::Result<String>) -> Option<HanjaDictEntry> {
    let string = match line {
        Ok(x) => x,
        Err(_) => return None
    };
    let splitted: Vec<&str> = string.split(|c| c == ':' || c == '\n').collect();
    if splitted.len() < 3 ||
        splitted[0].len() != 3 ||
        splitted[1].len() != 3 {
        return None;
    }

    Some(HanjaDictEntry {
        hangeul: splitted[0].chars().nth(0).unwrap(),
        hanja: splitted[1].chars().nth(0).unwrap(),
        desc: String::from(splitted[2])
    })
}

struct HanjaFreqEntry {
    hanja: char,
    freq: i32,
}

fn parse_freq_line(line: io::Result<String>) -> Option<HanjaFreqEntry> {
    let string = match line {
        Ok(x) => x,
        Err(_) => return None
    };
    let splitted: Vec<&str> = string.split(|c| c == ':' || c == '\n').collect();
    if splitted.len() < 2 ||
        splitted[0].len() != 3 {
        return None;
    }

    let freq:i32 = splitted[1].parse().unwrap();
    if freq == 0 {
        return None;
    }

    Some(HanjaFreqEntry {
        hanja: splitted[0].chars().nth(0).unwrap(),
        freq: freq,
    })
}

const DICT_FILE_PATH: &'static str = "data/hanja.txt";
const FREQ_FILE_PATH: &'static str = "data/freq-hanja.txt";

fn main() {
    println!("cargo:rerun-if-changed={}", DICT_FILE_PATH);
    println!("cargo:rerun-if-changed={}", FREQ_FILE_PATH);

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("data.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(&mut file, "static ENTRIES: phf::Map<char, &'static [(char, &'static str)]> = ").unwrap();

    let freq_lines = BufReader::new(File::open(FREQ_FILE_PATH).unwrap()).lines();
    let freq_entries = freq_lines.filter_map(parse_freq_line);
    let mut freq_map = HashMap::<char, i32>::new();
    for entry in freq_entries {
        freq_map.insert(entry.hanja, entry.freq);
    }

    let dict_lines = BufReader::new(File::open(DICT_FILE_PATH).unwrap()).lines();
    let dict_entries = dict_lines.filter_map(parse_dict_line);

    let mut map = phf_codegen::Map::new();
    let mut current_char = '가';
    let mut current_char_entries = Vec::<HanjaDictEntry>::new();
    for entry in dict_entries {
        if current_char != entry.hangeul {
            let mut value_string = String::from("&[");
            current_char_entries.sort_unstable_by_key(|entry| match freq_map.get(&entry.hanja) {
                Some(&x) => -x,
                None => 0
            });
            for current_char_entry in &current_char_entries {
                value_string.push_str(&current_char_entry.get_value_string());
                value_string.push(',');
            }
            value_string.push(']');
            map.entry(current_char, &value_string);
            current_char = entry.hangeul;
            current_char_entries.clear();
        }
        current_char_entries.push(entry);
    }
    if current_char_entries.len() > 0 {
        let mut value_string = String::from("&[");
        current_char_entries.sort_unstable_by_key(|entry| freq_map.get(&entry.hanja));
        for current_char_entry in &current_char_entries {
            value_string.push_str(&current_char_entry.get_value_string());
            value_string.push(',');
        }
        value_string.push(']');
        map.entry(current_char, &value_string);
    }
    map.build(&mut file).unwrap();
    write!(&mut file, ";\n").unwrap();
}

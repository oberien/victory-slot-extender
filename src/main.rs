use std::fs::{self, File};
use std::path::PathBuf;
use std::net::TcpStream;
use std::io::{Read, Write, stdin, stdout};

fn main() {
    print!("How many character solts would you like to have: ");
    stdout().flush().unwrap();
    let mut line = String::new();
    stdin().read_line(&mut line).expect("Could not read");
    assert_eq!(line.pop().unwrap(), '\n');
    let number = line.trim().parse::<u32>().expect("Input was not a number");

    let mut path = std::env::home_dir().unwrap();
    if cfg!(windows) {
        path.push("Appdata"); path.push("Local");
    } else {
        path.push(".config"); path.push("Epic");
    };
    path.push("Victory"); path.push("Saved");
    path.push("SaveGames"); path.push("ChracterSlotSave.7.sav");

    println!("Using file {}", path.display());
    // create backup file
    let file_name_bck = format!("{}{}", path.display(), ".bck");
    let path_bck = PathBuf::from(&file_name_bck);
    println!("Creating backup file...");
    fs::copy(&path, path_bck).expect("Error creating backup");
    println!("Backup finished");

    let mut buf = Vec::new();
    {
        let mut file = File::open(&path).expect("Error opening file");
        file.read_to_end(&mut buf).expect("Error reading file");
    }
    let bck = buf.clone();
    println!("Checking file...");
    // check header
    assert_eq!(&buf[..4], "GVAS".as_bytes());
    assert_eq!(&buf[26..48], b"++depot+UE4-Releases+4");
    assert_eq!(&buf[56..78], b"LocalCharacterSlotSave");
    let to_seek = b"\x0f\x00\x00\x00CharacterSlots\x00\x0e\x00\x00\x00ArrayProperty\x00";
    let pos = buf.windows(to_seek.len()).position(|a| a == &to_seek[..])
        .expect("Can not find correct position");
    let pos = pos + to_seek.len();
    assert_eq!(&buf[pos+8..pos+27], b"\x0f\x00\x00\x00StructProperty\x00");
    let pos = pos + 27;
    buf[pos] = number as u8;
    buf[pos+1] = (number >> 8) as u8;
    buf[pos+2] = (number >> 16) as u8;
    buf[pos+3] = (number >> 24) as u8;
    println!("Writing file...");
    {
        let mut file = File::create(path).expect("Error opening file for writing");
        file.write(&buf[..]).expect("Error writing file");
    }
    println!("Slots successfully added");
}


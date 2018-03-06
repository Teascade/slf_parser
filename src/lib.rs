use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;
use std::io::Read;

pub struct BMCharacter {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub xoffset: u32,
    pub yoffset: u32,
    pub xadvance: u32,
}

pub struct BMFont {
    pub font_name: String,
    pub chars: HashMap<u32, BMCharacter>,
    pub line_height: u32,
    pub size: u32,
}

impl BMFont {
    pub fn load_and_parse<T: Into<PathBuf>>(path: T) -> Result<BMFont, String> {
        let path = path.into();
        let mut file;
        match File::open(&path) {
            Ok(f) => file = f,
            Err(error) => return Err(format!("Error while loading .sfl file: {}", error)),
        }

        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let mut lines = buffer.lines();

        if lines.clone().count() < 5 {
            return Err("Erronous .sfl file; too few lines to initialize.".to_owned());
        }

        // Take font name from first line
        let font_name = lines.next().unwrap().to_owned();

        // Take line height and font size from second line
        let line_h_and_size = lines.next().unwrap().to_owned();
        let mut parts = line_h_and_size.split(' ');
        let size;
        let line_height;
        if parts.clone().count() == 2 {
            match parts.nth(0).unwrap().parse::<u32>() {
                Ok(number) => size = number,
                Err(error) => return Err(format!("Error parsing line height: '{}'", error)),
            }
            match parts.nth(0).unwrap().parse::<u32>() {
                Ok(number) => line_height = number,
                Err(error) => return Err(format!("Error parsing size: '{}'", error)),
            }
        } else {
            return Err(format!(
                "Second line does not contain two values formatted as 'line-height size'"
            ));
        }

        // Skip image name, not saved for now at least.
        let mut lines = lines.skip(1);

        // Read characters
        let character_amount;
        match lines.next().unwrap().to_owned().parse::<u32>() {
            Ok(amount) => character_amount = amount,
            Err(_) => return Err(format!("Error while parsing character amount at line: 4")),
        }

        if lines.clone().count() + 5 < 5 + character_amount as usize {
            return Err(format!("Erronous .sfl file; character amount (line 4) does not match actual character amount; is {}, should be {}", lines.count() + 5, 5 + character_amount));
        }

        let mut chars = HashMap::<u32, BMCharacter>::new();
        for i in 0..character_amount {
            let character = BMFont::read_character(lines.next().unwrap().to_owned(), i + 1);
            match character {
                Ok(ch) => chars.insert(ch.id, ch),
                Err(error) => return Err(error),
            };
        }

        return Ok(BMFont {
            font_name,
            chars,
            line_height,
            size,
        });
    }

    fn read_character(line: String, line_number: u32) -> Result<BMCharacter, String> {
        let mut parts = line.split(' ');
        if parts.clone().count() < 8 {
            return Err(format!(
                "Too few parts in character at line: {}",
                line_number
            ));
        }

        let mut numbers: Vec<u32> = vec![0; 8];
        for i in 0..8 {
            match parts.nth(0).unwrap().parse::<u32>() {
                Ok(number) => numbers[i] = number,
                Err(_) => {
                    return Err(format!(
                        "Error while parsing number at line: {}",
                        line_number
                    ));
                }
            }
        }

        Ok(BMCharacter {
            id: numbers[0],
            x: numbers[1],
            y: numbers[2],
            width: numbers[3],
            height: numbers[4],
            xoffset: numbers[5],
            yoffset: numbers[6],
            xadvance: numbers[7],
        })
    }
}

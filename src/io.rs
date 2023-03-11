use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub struct IO;

impl<'a> IO {
    pub fn read(&self, input_path: &str) -> std::string::String {
        let path = Path::new(input_path);
        let display = path.display();
        let mut string = String::new();

        let mut file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        match file.read_to_string(&mut string) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(string) => string,
        };
        string
    }

    pub fn write(&self, output_path: &str, buffer: &[String]) {
        let path = Path::new(&output_path);
        let display = path.display();
        let mut file = match File::create(path) {
            Err(v) => panic!("couldn't write to {}: {}", display, v),
            Ok(file) => file,
        };
        for i in buffer {
            if let Err(why) = file.write_all(i.as_bytes()) {
                panic!("couldn't write to {}: {}", display, why);
            };
        }
    }

    pub fn parse(&self, text: &'a str, base_address: i32) {
        let mut flag: bool = false;
        let mut buffer = Vec::new();

        let convert = |x: &str| i32::from_str_radix(x, 16).unwrap();
        let old_gadgets: Vec<_> = text.lines().collect();

        for gadget in old_gadgets {
            let lines: Vec<_> = gadget.split(": ").collect();
            let Some(new_address) = base_address.checked_add(convert(&lines[0][2..])) else { break };
            let new_address = format!("{new_address:X}");
            self.check(&new_address, &mut flag);
            if flag {
                buffer.push(format!("{:#08X?} : {}\n", convert(&new_address), lines[1]));
                flag = false;
            }
        }
        self.write("new_rop_gadgets.txt", &buffer);
    }

    pub fn check(&self, address: &'a str, state: &'a mut bool) {
        let length = address.chars().count();
        if length % 2 != 0 || length != 8 {
            return;
        }
        for i in (0..length).step_by(2) {
            let byte = address[i..i + 2].parse::<u8>().unwrap_or(0);
            if !(0x20..0x80).contains(&byte) {
                *state = false;
                break;
            }
            *state = true;
        }
    }
}

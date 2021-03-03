use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub struct IO;

impl IO {
    pub fn read(&self, input_path: &str) -> std::string::String {
        let path = Path::new(input_path);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        let mut string = String::new();
        match file.read_to_string(&mut string) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => println!("You Did It. The Crazy Son of a Bitch, You Did It!"),
        };
        string
    }

    pub fn write(&self, output_path: String, buffer: Vec<String>) {
        let path = Path::new(&output_path);
        let display = path.display();
        let mut file = match File::create(&path) {
            Err(v) => panic!("couldn't write to {}: {}", display, v),
            Ok(file) => file,
        };
        for i in buffer.iter() {
            match file.write_all(i.as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why),
                Ok(_) => (),
            };
        }
    }

    pub fn parse(&self, _text: String, base_address: i32) {
        let mut _flag: bool = false;
        let mut buffer = Vec::new();

        let convert = |x: &str| i32::from_str_radix(x, 16).unwrap();
        let old_gadgets: Vec<_> = _text.lines().collect();

        for gadget in old_gadgets.into_iter() {
            let lines: Vec<_> = gadget.split(": ").collect();
            let new_address = match base_address.checked_add(convert(&lines[0][2..])) {
                Some(new_address) => new_address,
                None => break,
            };
            let new_address = format!("{:X}", new_address);
            self.check(&new_address, &mut _flag);
            if _flag == true {
                buffer.push(format!("{:#08X?} : {}\n", convert(&new_address), lines[1]));
                _flag = false;
            }
        }
        self.write("new_rop_gadgets.txt".to_string(), buffer);
    }

    pub fn check(&self, address: &str, state: &mut bool) {
        let length = address.chars().count();
        if length % 2 != 0 || length != 8 {
            return;
        }
        for i in (0..length).step_by(2) {
            let _byte = address[i..i + 2].parse::<u8>().unwrap_or(0);
            if (0x20..0x80).contains(&_byte) == false {
                *state = false;
                break;
            } else {
                *state = true;
            }
        }
    }
}

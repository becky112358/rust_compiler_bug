
use serialport;

const BAUD_RATE: u32 = 115200;

use serialport::{SerialPort};

struct Example {
    member0: &'static [u8],
    member1: &'static str,
    member2: fn(&mut Box<dyn SerialPort>),
}

const EXAMPLE_MAP: &[Example] = &[
    Example { member0: "0".as_bytes(), member1: "1", member2: function0 },
    Example { member0: "0".as_bytes(), member1: "1", member2: function1 },
];

fn function0(_unused: &mut Box<dyn SerialPort>) { }

fn function1(_unused: &mut Box<dyn SerialPort>) { }


fn main() {
    let mut port = serialport::new("COM2", BAUD_RATE).open().unwrap();

    for example_member in EXAMPLE_MAP {
        (example_member.member2)(&mut port);
    }
}


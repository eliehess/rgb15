use std::{env, vec::Vec};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        2 => hex_to_rgb(&args[1]),
        4 => rgb_to_hex(&args[1], &args[2], &args[3]),
        _ => { println!("Enter one argument to convert 2-byte hex to RGB15, or three to convert RGB15 to 2-byte hex") }
    };
}

fn hex_to_rgb(hex: &str) {
    let mut newhex: &str = hex.clone();

    match newhex.len() {
        4 => (),
        6 => {
            if (&newhex[0..2]).eq("0x") {
                newhex = &newhex[2..6];
            } else {
                eprintln!("Please enter a valid 2-byte hex string"); return;
            }
        },
        _ => { eprintln!("Please enter a valid 2-byte hex string"); return; }
    }

    let converted: Vec<u8> = match hex::decode(newhex) {
        Ok(x) => x,
        Err(_) => { eprintln!("Please enter a valid 2-byte hex string"); return; }
    };

    if converted.len() != 2 {
        eprintln!("Something went wrong."); 
        return;
    }

    let temp: u16 = (converted[0] as u16) << 8 | (converted[1] as u16);

    println!("({}, {}, {})", temp & 0x1F, (temp >> 5) & 0x1F, (temp >> 10) & 0x1F);
}

fn rgb_to_hex(r: &str, g: &str, b: &str) {
    let r16: u16 = match r.parse::<u16>() {
        Ok(x) => x,
        Err(_) => { eprintln!("Please enter numerical input"); return; }
    };
    let g16: u16 = match g.parse::<u16>() {
        Ok(x) => x,
        Err(_) => { eprintln!("Please enter numerical input"); return; }
    };
    let b16: u16 = match b.parse::<u16>() {
        Ok(x) => x,
        Err(_) => { eprintln!("Please enter numerical input"); return; }
    };

    if r16 > 31 || g16 > 31 || b16 > 31 {
        eprintln!("All three numbers must be between 0 and 31, inclusive.");
        return;
    }
        
    let fin: u16 = r16 | (g16 << 5) | (b16 << 10);
    println!("{:#01x}", fin);
}

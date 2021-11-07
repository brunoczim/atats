use atats::{
    assembly::disassemble::{Config, Disassembler},
    binary::decode::{Decoder, IoDecoder},
    memory::RomBank,
};
use std::{
    io::{self, Read},
    process,
};

fn main() {
    if let Err(error) = try_main() {
        eprintln!("stdin: {}", error);
        process::exit(-1)
    }
}

fn try_main() -> io::Result<()> {
    let mut bank = 0;
    let mut rom_bank_content = [0; RomBank::SIZE];
    let config = Config::default();

    'outer: loop {
        match io::stdin().read_exact(&mut rom_bank_content) {
            Ok(()) => (),
            Err(error) => {
                if error.kind() == io::ErrorKind::UnexpectedEof {
                    break Ok(());
                } else {
                    break Err(error);
                }
            },
        }

        let stream: &[u8] = &rom_bank_content;
        let mut decoder = IoDecoder::new(stream);
        let mut disassembler =
            Disassembler::with_config(RomBank::OFFSET, config);

        println!("; Bank {}", bank);
        loop {
            match decoder.decode() {
                Ok(instruction) => {
                    println!("{}", disassembler.next(instruction))
                },
                Err(error) => {
                    if error.kind() == io::ErrorKind::UnexpectedEof {
                        break;
                    } else {
                        break 'outer Err(error);
                    }
                },
            }
        }

        bank += 1;
    }
}

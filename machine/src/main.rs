/*
* R1 - accumulator upper/carry
* R2 - accumulator
*
*
* 0x00 - invalid
* 0x01 - program end
* 0x02 ## - print byte
* 0x03 ## - load to R2
*/

fn main() {
    let code: &[u8] = &[
        0x02, b'H', //
        0x02, b'e', //
        0x02, b'l', //
        0x02, b'l', //
        0x02, b'o', //
        0x02, b' ', //
        0x02, b'W', //
        0x02, b'o', //
        0x02, b'r', //
        0x02, b'l', //
        0x02, b'd', //
        0x02, b'!', //
        0x02, b'\n', //
        0x01,  // end
    ];
    let code_len = code.len();

    print!("Program length: {} bytes\n\n", code_len);

    let mut ip: u16 = 0;
    const MAX_INSTR: u32 = 50_000;
    let mut instruction_counter: u32 = 0;

    let mut finished = false;
    while instruction_counter < MAX_INSTR {
        let opcode = code[ip as usize];

        match opcode {
            0x01 => {
                finished = true;
                break;
            }
            0x02 => {
                // TODO: check for overflow
                print!("{}", code[ip as usize + 1] as char);
                ip += 2;
            }
            0x03 => {
                let data = code[ip as usize + 1];
                ip += 2;
            }
            _ => {
                println!("Invalid opcode: 0x{:#x}", opcode);
                break;
            }
        }

        instruction_counter += 1;
    }

    if finished {
        print!("\nProgram finished succesfully\n");
    } else {
        print!("\n!! Program did not finish!\n");
    }

    print!("Executed {} instructions\n", instruction_counter);
}

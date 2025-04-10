use std::usize;

// https://en.wikipedia.org/wiki/CHIP-8
struct CPU {
    current_operation: u16,
    registers: [u8; 2],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn run(&mut self) {
        // loop {
        let opcode = self.read_opcode();

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        match (c, x, y, d) {
            (0x8, _, _, 4) => self.add(x, y),
            _ => todo!("opcode {:04x}", opcode),
        };
        // }
    }

    fn add(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize]
    }
}

fn main() {
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2],
    };

    cpu.current_operation = 0x8014;
    cpu.registers[0] = 0;
    cpu.registers[1] = 5;

    cpu.run();

    assert_eq!(cpu.registers[0], 5);
}

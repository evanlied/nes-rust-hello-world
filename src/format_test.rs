use crate::{cpu::{addressing_modes::AddressingMode, opcodes::OP_CODE_REF_TABLE, CPU}, MemAccess};

pub fn trace(cpu: &mut CPU) -> String {
    let op_code_byte = cpu.mem_read(cpu.program_counter);
    let op_code = OP_CODE_REF_TABLE.get(&op_code_byte)
        .expect(&format!("{op_code_byte} is not a valid opcode"));
    let op_code_args = {
        let arg1 = if op_code.bytes > 1 
            { &format!("{:02X}", cpu.mem_read(cpu.program_counter + 1)) }
            else { "  " };
        let arg2 = if op_code.bytes > 2
            { &format!("{:02X}", cpu.mem_read(cpu.program_counter + 2))}
            else { "  "};
        format!(
            "{arg1} {arg2}",
        )
    };
    let op_code_parametize: String = {
        let trimmed_addr = op_code_args.trim();
        let is_indirect = match op_code.addressing_mode {
            AddressingMode::IndirectX | AddressingMode::IndirectY => true,
            _ => false,
        };

        if trimmed_addr.len() == 0 { String::from("       ") }

        else if is_indirect {
            format!("(${trimmed_addr})")
        } 

        else { format!("#${trimmed_addr:<4} ") }
    };
    let indirect_addr: String = {
        let mut result = String::new();
        match op_code.addressing_mode {
            AddressingMode::IndirectX => {
                let addr = get_operand_address(cpu, cpu.program_counter + 1, &op_code.addressing_mode);
                let data = cpu.mem_read(addr);
                result.push_str(&format!(",X = {addr:04X} @ {addr:04X} = {data:02X}"))
            },
            AddressingMode::IndirectY => {
                let addr = get_operand_address(cpu, cpu.program_counter + 1, &op_code.addressing_mode);
                let data = cpu.mem_read(addr);
                result.push_str(&format!(",Y = {addr:04X} @ {addr:04X} = {data:02X}"))
            },
            _ => { result.push_str("                   "); }
        };
        result
    };

    format!(
        "{:04X}  {:02X} {}  {} {}{}  A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
        cpu.program_counter,
        op_code_byte,
        op_code_args,
        op_code.instruction,
        op_code_parametize,
        indirect_addr,
        cpu.register_a,
        cpu.register_x,
        cpu.register_y,
        cpu.status.0,
        cpu.stack_pointer,
    )
}

// Get operand address by locally changing the cpu program counter and changing it back after leaving function scope
fn get_operand_address(cpu: &mut CPU, new_counter: u16, mode: &AddressingMode) -> u16 {
    let orig_pc = cpu.program_counter;
    cpu.program_counter = new_counter;
    let result = cpu.get_operand_address(mode);
    cpu.program_counter = orig_pc;
    result
}

#[cfg(test)]
mod format_tests {
    use super::*;
    use crate::{bus::Bus, cpu::CPU, rom::Rom, MemAccess};
    // use crate::rom::

    #[test]
    fn test_format_trace() {
        let mut bus = Bus::new(
            Rom::from_rom("./nestest.nes").unwrap()
        );
        bus.mem_write(100, 0xa2);
        bus.mem_write(101, 0x01);
        bus.mem_write(102, 0xCA);
        bus.mem_write(103, 0x88);
        bus.mem_write(104, 0x0);

        let mut cpu = CPU::new_with_bus(bus);
        cpu.program_counter = 0x64;
        cpu.status.0 = 0b0010_0100;
        cpu.register_a = 1;
        cpu.register_x = 2;
        cpu.register_y = 3;
        let mut result: Vec<String> = vec!();
        cpu.run_with_callback(|cpu| {
            result.push(trace(cpu))
        });
        assert_eq!(
            "0064  A2 01     LDX #$01                        A:01 X:02 Y:03 P:24 SP:FD",
            result[0]
        );
        assert_eq!(
            "0066  CA        DEX                             A:01 X:01 Y:03 P:24 SP:FD",
            result[1]
        );
        assert_eq!(
            "0067  88        DEY                             A:01 X:00 Y:03 P:26 SP:FD",
            result[2]
        );
    }

    #[test]
    fn test_format_mem_access() {
        let mut bus = Bus::new(
            Rom::from_rom("nestest.nes").unwrap()
        );
        bus.mem_write(100, 0x11);
        bus.mem_write(101, 0x33);
        bus.mem_write(0x33, 00);
        bus.mem_write(0x34, 04);
        bus.mem_write(0x400, 0xAA);

        let mut cpu = CPU::new_with_bus(bus);
        cpu.program_counter = 0x64;
        cpu.status.0 = 0x24;
        cpu.register_y = 0;
        let mut result: Vec<String> = vec!();
        cpu.run_with_callback(|cpu| {
            result.push(trace(cpu));
        });
        assert_eq!(
            "0064  11 33     ORA ($33),Y = 0400 @ 0400 = AA  A:00 X:00 Y:00 P:24 SP:FD",
            result[0]
        );
    }
}
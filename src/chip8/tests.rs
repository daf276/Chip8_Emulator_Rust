mod tests {
    use super::super::Chip8;

    #[test]
    fn test_jump() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x18;
        test_chip.memory[0x201] = 0x54;
        test_chip.emulate_cycle();
        assert_eq!(test_chip.pc, 0x0854);
    }

    #[test]
    fn test_call_subroutine() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x28;
        test_chip.memory[0x201] = 0x54;
        test_chip.emulate_cycle();

        assert_eq!(test_chip.pc, 0x0854);
        assert_eq!(test_chip.stack[0], 0x0200);
        assert_eq!(test_chip.sp, 1);
    }

    #[test]
    fn test_return_from_subroutine() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x28;
        test_chip.memory[0x201] = 0x54;
        test_chip.memory[0x854] = 0x00;
        test_chip.memory[0x855] = 0xEE;

        test_chip.emulate_cycle();
        test_chip.emulate_cycle();

        assert_eq!(test_chip.pc, 0x202);
        assert_eq!(test_chip.stack[0], 0x0200);
        assert_eq!(test_chip.sp, 0);
    }

    #[test]
    fn test_register_number_se() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x38;
        test_chip.memory[0x201] = 0x54;
        test_chip.v[8] = 0x54;

        test_chip.emulate_cycle();
        assert_eq!(test_chip.pc, 0x204);

        let mut test_chip2 = Chip8::new();
        test_chip2.memory[0x200] = 0x38;
        test_chip2.memory[0x201] = 0x54;
        test_chip2.v[8] = 0x64;

        test_chip2.emulate_cycle();
        assert_eq!(test_chip2.pc, 0x202);
    }

    #[test]
    fn test_register_number_sne() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x48;
        test_chip.memory[0x201] = 0x54;
        test_chip.v[8] = 0x54;

        test_chip.emulate_cycle();
        assert_eq!(test_chip.pc, 0x202);

        let mut test_chip2 = Chip8::new();
        test_chip2.memory[0x200] = 0x48;
        test_chip2.memory[0x201] = 0x54;
        test_chip2.v[8] = 0x64;

        test_chip2.emulate_cycle();
        assert_eq!(test_chip2.pc, 0x204);
    }

    #[test]
    fn test_register_register_se() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x58;
        test_chip.memory[0x201] = 0x50;
        test_chip.v[8] = 0x54;
        test_chip.v[5] = 0x54;

        test_chip.emulate_cycle();
        assert_eq!(test_chip.pc, 0x204);

        let mut test_chip2 = Chip8::new();
        test_chip2.memory[0x200] = 0x58;
        test_chip2.memory[0x201] = 0x50;
        test_chip2.v[8] = 0x64;
        test_chip2.v[5] = 0x54;

        test_chip2.emulate_cycle();
        assert_eq!(test_chip2.pc, 0x202);
    }

    #[test]
    fn test_ld() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x61;
        test_chip.memory[0x201] = 0x05;
        test_chip.emulate_cycle();

        assert_eq!(test_chip.v[1], 5);
    }

    #[test]
    fn test_add() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x71;
        test_chip.memory[0x201] = 0x05;
        test_chip.v[1] = 5;
        test_chip.emulate_cycle();

        assert_eq!(test_chip.v[1], 10);
    }

    #[test]
    fn test_or() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x81;
        test_chip.memory[0x201] = 0x21;
        test_chip.v[1] = 0b0001;
        test_chip.v[2] = 0b1100;
        test_chip.emulate_cycle();

        assert_eq!(test_chip.v[1], 0b1101);
    }

    #[test]
    fn test_and() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x81;
        test_chip.memory[0x201] = 0x22;
        test_chip.v[1] = 0b0011;
        test_chip.v[2] = 0b1010;
        test_chip.emulate_cycle();

        assert_eq!(test_chip.v[1], 0b0010);
    }

    #[test]
    fn test_xor() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x81;
        test_chip.memory[0x201] = 0x23;
        test_chip.v[1] = 0b0011;
        test_chip.v[2] = 0b1010;
        test_chip.emulate_cycle();

        assert_eq!(test_chip.v[1], 0b1001);
    }

    #[test]
    fn test_add_reg_reg() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x81;
        test_chip.memory[0x201] = 0x24;
        test_chip.v[1] = 5;
        test_chip.v[2] = 6;
        test_chip.emulate_cycle();

        assert_eq!(test_chip.v[1], 11);
        assert_eq!(test_chip.v[15], 0);

        let mut test_chip2 = Chip8::new();
        test_chip2.memory[0x200] = 0x81;
        test_chip2.memory[0x201] = 0x24;
        test_chip2.v[1] = u8::max_value();
        test_chip2.v[2] = 6;
        test_chip2.emulate_cycle();

        assert_eq!(test_chip2.v[1], 5);
        assert_eq!(test_chip2.v[15], 1);
    }

    #[test]
    fn test_sub_reg_reg() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x81;
        test_chip.memory[0x201] = 0x25;
        test_chip.v[1] = 7;
        test_chip.v[2] = 2;
        test_chip.emulate_cycle();

        assert_eq!(test_chip.v[1], 5);
        assert_eq!(test_chip.v[15], 0);

        let mut test_chip2 = Chip8::new();
        test_chip2.memory[0x200] = 0x81;
        test_chip2.memory[0x201] = 0x25;
        test_chip2.v[1] = 5;
        test_chip2.v[2] = 7;
        test_chip2.emulate_cycle();

        assert_eq!(test_chip2.v[1], 254);
        assert_eq!(test_chip2.v[15], 1);
    }

    #[test]
    fn test_shr() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x81;
        test_chip.memory[0x201] = 0x26;
        test_chip.v[1] = 0b0110;
        test_chip.emulate_cycle();

        assert_eq!(test_chip.v[1], 3);
        assert_eq!(test_chip.v[15], 0);

        let mut test_chip2 = Chip8::new();
        test_chip2.memory[0x200] = 0x81;
        test_chip2.memory[0x201] = 0x26;
        test_chip2.v[1] = 0b1111;
        test_chip2.emulate_cycle();

        assert_eq!(test_chip2.v[1], 7);
        assert_eq!(test_chip2.v[15], 1);
    }

    #[test]
    fn test_subn_reg_reg() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x81;
        test_chip.memory[0x201] = 0x27;
        test_chip.v[1] = 2;
        test_chip.v[2] = 7;
        test_chip.emulate_cycle();

        assert_eq!(test_chip.v[1], 5);
        assert_eq!(test_chip.v[15], 0);

        let mut test_chip2 = Chip8::new();
        test_chip2.memory[0x200] = 0x81;
        test_chip2.memory[0x201] = 0x27;
        test_chip2.v[1] = 7;
        test_chip2.v[2] = 5;
        test_chip2.emulate_cycle();

        assert_eq!(test_chip2.v[1], 254);
        assert_eq!(test_chip2.v[15], 1);
    }

    #[test]
    fn test_shl() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x81;
        test_chip.memory[0x201] = 0x2E;
        test_chip.v[1] = 0b0010;
        test_chip.emulate_cycle();

        assert_eq!(test_chip.v[1], 4);
        assert_eq!(test_chip.v[15], 0);

        let mut test_chip2 = Chip8::new();
        test_chip2.memory[0x200] = 0x81;
        test_chip2.memory[0x201] = 0x2E;
        test_chip2.v[1] = 0b10000010;
        test_chip2.emulate_cycle();

        assert_eq!(test_chip2.v[1], 4);
        assert_eq!(test_chip2.v[15], 1);
    }

    #[test]
    fn test_register_register_sne() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x98;
        test_chip.memory[0x201] = 0x50;
        test_chip.v[8] = 0x54;
        test_chip.v[5] = 0x64;

        test_chip.emulate_cycle();
        assert_eq!(test_chip.pc, 0x204);

        let mut test_chip2 = Chip8::new();
        test_chip2.memory[0x200] = 0x98;
        test_chip2.memory[0x201] = 0x50;
        test_chip2.v[8] = 0x64;
        test_chip2.v[5] = 0x64;

        test_chip2.emulate_cycle();
        assert_eq!(test_chip2.pc, 0x202);
    }

    #[test]
    fn test_ldi() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0xA5;
        test_chip.memory[0x201] = 0x53;
        test_chip.emulate_cycle();

        assert_eq!(test_chip.i_reg, 0x553);
    }

    #[test]
    fn test_jump_plusv0() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0xB5;
        test_chip.memory[0x201] = 0x53;
        test_chip.v[0] = 0x30;
        test_chip.emulate_cycle();
        assert_eq!(test_chip.pc, 0x583);
    }

    #[test]
    fn test_ld_into_ireg() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0xF0;
        test_chip.memory[0x201] = 0x07;
        test_chip.delay_timer = 5;
        test_chip.emulate_cycle();
        assert_eq!(test_chip.v[0], 5);
    }

    #[test]
    fn test_ld_reg_into_delay_timer() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0xF5;
        test_chip.memory[0x201] = 0x15;
        test_chip.v[5] = 12;
        test_chip.emulate_cycle();
        assert_eq!(test_chip.delay_timer, 11);
    }

    #[test]
    fn test_ld_reg_into_sound_timer() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0xF5;
        test_chip.memory[0x201] = 0x18;
        test_chip.v[5] = 12;
        test_chip.emulate_cycle();
        assert_eq!(test_chip.sound_timer, 12);
    }

    #[test]
    fn test_add_reg_to_ireg() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0xF5;
        test_chip.memory[0x201] = 0x1E;
        test_chip.v[5] = 12;
        test_chip.i_reg = 12;
        test_chip.emulate_cycle();
        assert_eq!(test_chip.i_reg, 24);
        assert_eq!(test_chip.v[15], 0);

        let mut test_chip2 = Chip8::new();
        test_chip2.memory[0x200] = 0xF5;
        test_chip2.memory[0x201] = 0x1E;
        test_chip2.v[5] = 12;
        test_chip2.i_reg = u16::max_value();
        test_chip2.emulate_cycle();
        assert_eq!(test_chip2.i_reg, 11);
        assert_eq!(test_chip2.v[15], 1);
    }

    #[test]
    fn test_ireg_to_sprite_location() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0xF5;
        test_chip.memory[0x201] = 0x29;
        test_chip.v[5] = 5;
        test_chip.emulate_cycle();
        assert_eq!(test_chip.i_reg, 25);
    }

    #[test]
    fn test_store_bcd_in_memory() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0xF5;
        test_chip.memory[0x201] = 0x33;
        test_chip.v[5] = 234;
        test_chip.emulate_cycle();
        assert_eq!(test_chip.memory[test_chip.i_reg as usize], 2);
        assert_eq!(test_chip.memory[test_chip.i_reg as usize + 1], 3);
        assert_eq!(test_chip.memory[test_chip.i_reg as usize + 2], 4);
    }

    #[test]
    fn test_store_all_regs_to_memory() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0xF3;
        test_chip.memory[0x201] = 0x55;

        test_chip.i_reg = 0x500;
        test_chip.v[0] = 234;
        test_chip.v[1] = 2;
        test_chip.v[2] = 35;
        test_chip.emulate_cycle();

        assert_eq!(test_chip.memory[test_chip.i_reg as usize], 234);
        assert_eq!(test_chip.memory[(test_chip.i_reg + 1) as usize], 2);
        assert_eq!(test_chip.memory[(test_chip.i_reg + 2) as usize], 35);
    }

    #[test]
    fn test_store_memory_to_all_regs() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0xF3;
        test_chip.memory[0x201] = 0x65;

        test_chip.i_reg = 0x400;
        test_chip.memory[0x400] = 234;
        test_chip.memory[0x401] = 2;
        test_chip.memory[0x402] = 35;
        test_chip.emulate_cycle();

        assert_eq!(test_chip.v[0], 234);
        assert_eq!(test_chip.v[1], 2);
        assert_eq!(test_chip.v[2], 35);
    }
}

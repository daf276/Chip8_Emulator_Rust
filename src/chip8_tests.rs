#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::Chip8;

    #[test]
    fn test_jump() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x18;
        test_chip.memory[0x201] = 0x54;
        test_chip.emulate_cycle();
        assert_eq!(test_chip.pc.get(), 0x0854);
    }

    #[test]
    fn test_call_subroutine() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x28;
        test_chip.memory[0x201] = 0x54;
        test_chip.emulate_cycle();

        assert_eq!(test_chip.pc.get(), 0x0854);
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

        assert_eq!(test_chip.pc.get(), 0x202);
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
        assert_eq!(test_chip.pc.get(), 0x204);

        let mut test_chip2 = Chip8::new();
        test_chip2.memory[0x200] = 0x38;
        test_chip2.memory[0x201] = 0x54;
        test_chip2.v[8] = 0x64;

        test_chip2.emulate_cycle();
        assert_eq!(test_chip2.pc.get(), 0x202);
    }

    #[test]
    fn test_register_number_sne() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x48;
        test_chip.memory[0x201] = 0x54;
        test_chip.v[8] = 0x54;

        test_chip.emulate_cycle();
        assert_eq!(test_chip.pc.get(), 0x202);

        let mut test_chip2 = Chip8::new();
        test_chip2.memory[0x200] = 0x48;
        test_chip2.memory[0x201] = 0x54;
        test_chip2.v[8] = 0x64;

        test_chip2.emulate_cycle();
        assert_eq!(test_chip2.pc.get(), 0x204);
    }

    #[test]
    fn test_register_register_se() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x58;
        test_chip.memory[0x201] = 0x50;
        test_chip.v[8] = 0x54;
        test_chip.v[5] = 0x54;

        test_chip.emulate_cycle();
        assert_eq!(test_chip.pc.get(), 0x204);

        let mut test_chip2 = Chip8::new();
        test_chip2.memory[0x200] = 0x58;
        test_chip2.memory[0x201] = 0x50;
        test_chip2.v[8] = 0x64;
        test_chip2.v[5] = 0x54;

        test_chip2.emulate_cycle();
        assert_eq!(test_chip2.pc.get(), 0x202);
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
    fn test_register_register_sne() {
        let mut test_chip = Chip8::new();
        test_chip.memory[0x200] = 0x98;
        test_chip.memory[0x201] = 0x50;
        test_chip.v[8] = 0x54;
        test_chip.v[5] = 0x64;

        test_chip.emulate_cycle();
        assert_eq!(test_chip.pc.get(), 0x204);

        let mut test_chip2 = Chip8::new();
        test_chip2.memory[0x200] = 0x98;
        test_chip2.memory[0x201] = 0x50;
        test_chip2.v[8] = 0x64;
        test_chip2.v[5] = 0x64;

        test_chip2.emulate_cycle();
        assert_eq!(test_chip2.pc.get(), 0x202);
    }
}

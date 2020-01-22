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
        assert_eq!(test_chip.pc, 0x0854);
    }
}

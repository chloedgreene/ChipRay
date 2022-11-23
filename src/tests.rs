#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn font() {
        let mut cpu = crate::cpu::cpu::new();

        cpu.inject_font(crate::font::FONT_MEM);

        //Check if font is loaded and in right spot
        assert_eq!(cpu.ram[0x50..0xA0], crate::font::FONT_MEM);
    }

    #[test]
    fn font_init() {
        let mut cpu = crate::cpu::cpu::new();

        cpu.init(&[0x60, 0x90, 0x40, 0x20, 0x00]);
        //Check if font is loaded and in right spot
        assert_eq!(cpu.ram[0x50..0xA0], crate::font::FONT_MEM);
    }

    #[test]
    fn code() {
        let code: &[u8; 5] = &[0x60, 0x90, 0x40, 0x20, 0x00];

        let mut cpu = crate::cpu::cpu::new();

        cpu.inject_code(&[0x60, 0x90, 0x40, 0x20, 0x00]);
        //Check if font is loaded and in right spot
        assert_eq!(&cpu.ram[0x200..0x205], code);
    }

    #[test]
    fn code_init() {
        let code: &[u8; 5] = &[0x60, 0x90, 0x40, 0x20, 0x00];

        let mut cpu = crate::cpu::cpu::new();

        cpu.init(&[0x60, 0x90, 0x40, 0x20, 0x00]);
        //Check if font is loaded and in right spot
        assert_eq!(&cpu.ram[0x200..0x205], code);
    }
}

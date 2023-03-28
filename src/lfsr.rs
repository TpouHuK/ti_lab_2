pub struct LFSR {
    state: u32,
    taps: u32,
}

const LFSR_LEN: u32 = 31;
impl LFSR {
    pub fn new(state: u32, taps: u32) -> LFSR {
        //assert_ne!(state, 0, "LFSR state should not be 0");
        LFSR { state, taps }
    }

    pub fn get_byte(&mut self) -> u8 {
        let mut out: u8 = 0;
        for _ in 0..8 {
            let pop_bit = self.state & (1 << LFSR_LEN - 1);
            out = (out << 1) | (pop_bit >> LFSR_LEN - 1) as u8;
            let new_bit = (self.state & self.taps).count_ones() & 1;
            self.state = (self.state << 1) | new_bit;
        }
        out
    }

    fn get_bit(&mut self) -> u8 {
        let pop_bit = self.state & (1 << LFSR_LEN - 1);
        let new_bit = (self.state & self.taps).count_ones() & 1;
        self.state = (self.state << 1) | new_bit;
        (pop_bit >> LFSR_LEN - 1) as u8
    }
}

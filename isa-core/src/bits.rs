pub fn get_bits(word: u32, hi: u8, lo: u8) -> u32 {
    debug_assert!(hi >= lo);
    debug_assert!(hi < 32);
    let width = (hi - lo + 1) as u32;
    let mask: u32 = if width == 32 {
        // 1 repeated width times
        u32::MAX
    } else {
        (1u32 << width) - 1
    };
    (word >> lo) & mask
}

pub fn set_bits(word: &mut u32, hi: u8, lo: u8, value: u32) {
    debug_assert!(hi >= lo);
    debug_assert!(hi < 32);
    let width = (hi - lo + 1) as u32;

    let mask: u32 = if width == 32 {
        // Suppose width=4, then mask will be 01111 (4 1's)
        // Used to extract the bits we want from value, but value should never be bigger than the
        // allowed bit widths
        u32::MAX
    } else {
        (1u32 << width) - 1
    };
    let shift = lo as u32;
    *word |= (value & mask) << shift;
}

pub fn fits_in_bits(value: usize, width: u8) -> bool {
    if width >= 31 {
        return true;
    }
    // 00001 << 4 -1 = 10000 -1 = 01111 (max val of 4 bit)
    value <= ((1usize << width) - 1)
}

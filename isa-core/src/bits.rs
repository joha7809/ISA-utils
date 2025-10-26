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

/// Check if a signed value fits in the given number of bits (using two's complement)
pub fn fits_in_signed_bits(value: isize, width: u8) -> bool {
    if width == 0 {
        return value == 0;
    }
    if width >= 64 {
        return true;
    }
    // For width=8: min=-128, max=127
    // For width=N: min = -(2^(N-1)), max = 2^(N-1) - 1
    let min = -(1isize << (width - 1));
    let max = (1isize << (width - 1)) - 1;
    value >= min && value <= max
}

/// Check if an unsigned value fits in the given number of bits
pub fn fits_in_unsigned_bits(value: usize, width: u8) -> bool {
    if width >= 31 {
        return true;
    }
    // 00001 << 4 -1 = 10000 -1 = 01111 (max val of 4 bit)
    value <= ((1usize << width) - 1)
}

/// Sign-extend a value from a given bit width to isize
pub fn sign_extend(value: u32, width: u8) -> isize {
    if width == 0 {
        return 0;
    }
    // Check if the sign bit is set
    let sign_bit = 1u32 << (width - 1);
    if (value & sign_bit) != 0 {
        // Negative: extend with 1s
        let mask = !((1u32 << width) - 1);
        (value | mask) as i32 as isize
    } else {
        // Positive: just cast
        value as isize
    }
}

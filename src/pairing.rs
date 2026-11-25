//! Pairing PIN generation and VGA welcome banner formatting for bare-metal boot.

/// Multiplier for the 64-bit Linear Congruential Generator used for pairing PIN derivation.
pub const PAIRING_LCG_MULTIPLIER: u64 = 6_364_136_223_846_793_005;

/// Derives an 8-digit pairing PIN (`0..100_000_000`) from a CPU timestamp counter sample.
pub fn derive_pairing_pin(tsc: u64) -> u32 {
    let lcg = tsc.wrapping_mul(PAIRING_LCG_MULTIPLIER).wrapping_add(1);
    ((lcg >> 32) % 100_000_000) as u32
}

/// Formats an 8-digit numeric pairing PIN as `"XXXX XXXX"` into a 9-byte ASCII buffer.
pub fn format_pairing_pin(mut pin: u32) -> [u8; 9] {
    let mut pin_str = [b'0'; 9];
    for i in (0..9).rev() {
        if i == 4 {
            pin_str[i] = b' ';
        } else {
            pin_str[i] = b'0' + (pin % 10) as u8;
            pin /= 10;
        }
    }
    pin_str
}

/// Builds the 13-row by 80-column ASCII boot banner with the formatted pairing PIN injected.
pub fn build_welcome_banner(pin_str: &[u8; 9]) -> [[u8; 80]; 13] {
    let mut lines = [
        *b"================================================================================",
        *b"                                                                                ",
        *b"                               Welcome to Silvae!                               ",
        *b"                                                                                ",
        *b"================================================================================",
        *b"                                                                                ",
        *b"  Connected to $NETWORKID                                                       ",
        *b"                                                                                ",
        *b"  To claim this node, enter this pairing code on the Silvae app or at           ",
        *b"  http://Silv.ae/start:                                                         ",
        *b"                                                                                ",
        *b"                                   1234 5678                                    ",
        *b"                                                                                ",
    ];
    lines[11][35..44].copy_from_slice(pin_str);
    lines
}

pub fn is_negative(byte: u8) -> bool {
    return (byte & (1 << 7)) != 0;
}

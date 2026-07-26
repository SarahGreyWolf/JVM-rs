pub const fn index_from_bytes(index1: u8, index2: u8) -> usize {
    ((index1 as usize) << 8) | index2 as usize
}

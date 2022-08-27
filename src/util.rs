pub const fn align_up(base: usize, align: usize) -> usize {
	base.wrapping_add(align.wrapping_sub(1)) & !align.wrapping_sub(1)
}
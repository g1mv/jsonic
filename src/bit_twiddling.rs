// https://graphics.stanford.edu/~seander/bithacks.html#HasBetweenInWord

#[inline(always)]
fn _has_less(x: u64, n: u64) -> u64 {
    (x - !0_u64 / 255 * n) & !x & !0_u64 / 255 * 128
}

#[inline(always)]
fn _has_zero(v: u64) -> u64 {
    _has_less(v, 1u64)
}

#[inline(always)]
fn _has_value(x: u64, n: u64) -> u64 {
    _has_zero(x ^ (!0_u64 / 255 * n))
}

#[inline(always)]
fn _has_between(x: u64, m: u64, n: u64) -> u64 {
    (!0_u64 / 255 * (127 + n) - (x & !0_u64 / 255 * 127) & !x & (x & !0_u64 / 255 * 127) + !0_u64 / 255 * (127 - m)) & !0_u64 / 255 * 128
}

#[inline(always)]
fn _count_between(x: u64, m: u64, n: u64) -> u64 {
    _has_between(x, m, n) / 128 % 255
}
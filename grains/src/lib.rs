pub fn square(s: u32) -> u64 {
   assert!((1..=64).contains(&s));
   2_u64.pow(s-1)
}

pub fn total() -> u64 {
    // https://rust-lang.github.io/rust-clippy/master/index.html
    (1..=64).map(square).sum()
}

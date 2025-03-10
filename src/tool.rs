pub fn parse_bn(bn: u64, decimals: u8) -> f64 {
    bn as f64 / 10u64.pow(decimals as u32) as f64
}

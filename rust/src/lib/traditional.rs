use super::data::Pattern;

impl Pattern {
    #[allow(dead_code)]
    pub fn get_traditional_siteswap(&self) -> String {
        let mut result = String::new();
        for (i, digit) in self.siteswap.iter().enumerate() {
            let crossing = if digit % 2 == 0 { "" } else { "x" };
            let digit_str = format!("{:x}{}", digit * 2, crossing);
            result = match i % 2 {
                0 => format!("{}({},2x)", result, digit_str),
                1 => format!("{}(2x,{})", result, digit_str),
                _ => unreachable!(),
            }
        }
        result += "*";
        return result;
    }
}

use super::data::Pattern;

impl Pattern {
    pub fn get_traditional_siteswap(&self) -> String {
        let mut result = String::new();
        for (i, digit) in self.siteswap.iter().enumerate() {
            result = match i % 2 {
                0 => format!("{}({},2x)", result, digit * 2),
                1 => format!("{}(2x,{})", result, digit * 2),
                _ => unreachable!(),
            }
        }
        result += "*";
        return result;
    }
}

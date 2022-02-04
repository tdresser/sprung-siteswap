use super::data::Pattern;

use super::data::Position;

fn get_throw_hand_position(p: &Position, sprung_digit: u32) -> String {
    if sprung_digit == 2 {
        return match p {
            Position::BottomNatural => "(20)",
            Position::BottomOpposite => "(-20)",
            Position::TopNatural => "(20,50)",
            Position::TopOpposite => "(-20,50)",
        }
        .to_string();
    }
    return match p {
        Position::BottomNatural => "(10)",
        Position::BottomOpposite => "(-10)",
        Position::TopNatural => "(10,50)",
        Position::TopOpposite => "(-10,50)",
    }
    .to_string();
}

fn get_catch_hand_position(p: &Position, sprung_digit: u32) -> String {
    if sprung_digit == 2 {
        return match p {
            Position::BottomNatural => "(20)",
            Position::BottomOpposite => "(-20)",
            Position::TopNatural => "(20,50)",
            Position::TopOpposite => "(-20,50)",
        }
        .to_string();
    }
    return match p {
        Position::BottomNatural => "(32)",
        Position::BottomOpposite => "(-32)",
        Position::TopNatural => "(32,50)",
        Position::TopOpposite => "(-32,50)",
    }
    .to_string();
}

#[allow(dead_code)]
fn get_hand_positions(a: &Position, b: &Position, sprung_digit: u32) -> String {
    return format!(
        "{}{}.",
        get_throw_hand_position(a, sprung_digit),
        get_catch_hand_position(b, sprung_digit)
    );
}

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
        if self.siteswap.len() % 2 == 1 {
            result += "*";
        }
        return result;
    }

    #[allow(dead_code)]
    pub fn get_colors(&self) -> String {
        let mut result = "".to_string();
        for i in 0..self.num_balls() {
            let color = if i == 0 { "red" } else { "green" };
            result = format!("{}{{{}}}", result, color)
        }
        return result;
    }

    fn num_balls(&self) -> u32 {
        return self.siteswap.iter().sum::<u32>() / (self.siteswap.len() as u32) + 1;
    }

    #[allow(dead_code)]
    pub fn get_hand_positions(&self) -> String {
        let len = num::integer::lcm(
            num::integer::lcm(self.arc_positions.len(), self.zip_positions.len()),
            self.siteswap.len(),
        );
        let mut arc_iter = self.arc_positions.iter().cycle();
        let mut zip_iter = self.zip_positions.iter().cycle();
        let mut swap_iter = self.siteswap.iter().cycle();

        let mut result = "".to_string();
        println!("arc: {}", self.arc_positions().len());
        println!("zip: {}", self.zip_positions().len());
        println!("lcm: {}", len);
        // Throws go [arc, zip, zip, arc, ...].
        for i in 0..len {
            let (arc_t, arc_c) = arc_iter.next().unwrap();
            let (zip_t, zip_c) = zip_iter.next().unwrap();
            let digit = swap_iter.next().unwrap();
            let arc = get_hand_positions(arc_t, arc_c, *digit);
            let zip = get_hand_positions(zip_t, zip_c, *digit);
            if i % 2 == 0 {
                result = format!("{}{}{}", result, arc, zip);
            } else {
                result = format!("{}{}{}", result, zip, arc);
            }
        }
        return result;
    }
}

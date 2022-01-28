use num;

use super::data::Pattern;
use super::data::Position;
use super::data::Positions;
use super::data::DEFAULT_POSITION;

fn get_advanced_position_str(a: &Position) -> &str {
    return match a {
        Position::BottomNatural => "B",
        Position::BottomOpposite => "b",
        Position::TopNatural => "T",
        Position::TopOpposite => "t",
    };
}

fn get_position_str((a, b): &(Position, Position)) -> String {
    return match (a, b) {
        (Position::BottomNatural, &Position::BottomNatural) => "n".to_string(),
        (Position::BottomOpposite, &Position::BottomOpposite) => "c".to_string(),
        (Position::TopNatural, &Position::TopNatural) => "i".to_string(),
        (Position::TopOpposite, &Position::TopOpposite) => "ci".to_string(),
        (a, b) => get_advanced_position_str(a).to_owned() + get_advanced_position_str(b),
    };
}

impl Pattern {
    pub fn get_canonical_form(&mut self) -> String {
        let mut result = "".to_string();
        if self.zip_positions != vec![(Position::BottomNatural, Position::BottomNatural)] {
            for zip_position in &self.zip_positions {
                result = format!("{}{}z", result, get_position_str(zip_position));
            }
        }
        result += "S";

        let len = num::integer::lcm(self.arc_positions.len(), self.siteswap.len());
        let mut current_arc_position = "n".to_string();
        let mut siteswap_iter = self.siteswap.iter().cycle();
        let mut arc_position_iter = self.arc_positions.iter().cycle();
        for _ in 0..len {
            let new_position = get_position_str(arc_position_iter.next().unwrap());
            let digit = char::from_digit(*siteswap_iter.next().unwrap(), 16).unwrap();
            if current_arc_position == new_position {
                result = format!("{}{}", result, digit,);
            } else {
                result = format!("{}{}{}", result, new_position, digit,);
            }
            current_arc_position = new_position;
        }

        return result;
    }
}

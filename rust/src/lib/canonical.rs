use super::data::Pattern;
use super::data::Position;

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

// Collapses repetition. e.g., cici -> ci.
fn collapse_positions(positions: &Vec<(Position, Position)>) -> Vec<(Position, Position)> {
    if positions.len() == 0 {
        return vec![];
    }
    for n in (1..positions.len() + 1).rev() {
        println!("N {}", n);
        // Can we divide |positions| into chunks of length n?
        if positions.len() % n != 0 {
            continue;
        }
        // Are all chunks equal?
        let mut chunks = positions.chunks_exact(positions.len() / n);
        let first = chunks.nth(0).unwrap();
        if chunks.all(|x| x == first) {
            // All chunks are equal, we're done.
            return first.to_vec();
        }
    }
    println!("{:?}", positions);
    unreachable!();
}

impl Pattern {
    fn normalize(&mut self) {
        self.zip_positions = collapse_positions(&self.zip_positions);
    }

    pub fn get_canonical_form(&mut self) -> String {
        self.normalize();
        let mut result = "".to_string();
        for zip_position in &self.zip_positions {
            result = format!("{}{}z", result, get_position_str(zip_position));
        }
        result += "S";
        assert!(self.nonzip_positions.len() == self.siteswap.len());

        let mut current_position = "n".to_string();
        for i in 0..self.nonzip_positions.len() {
            let new_position = get_position_str(&self.nonzip_positions[i]);
            let digit = char::from_digit(self.siteswap[i], 16).unwrap();
            if current_position == new_position {
                result = format!("{}{}", result, digit,);
            } else {
                result = format!("{}{}{}", result, new_position, digit,);
            }
            current_position = new_position;
        }

        return result;
    }
}

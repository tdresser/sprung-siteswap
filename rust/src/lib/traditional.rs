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

fn collect_cycle<'a, A>(a: &'a Vec<A>, len: usize) -> Vec<&'a A> {
    return a.iter().cycle().take(len).collect::<Vec<_>>();
}

#[allow(dead_code)]
fn get_hand_positions(a: &Position, b: &Position, throw_digit: u32, catch_digit: u32) -> String {
    return format!(
        "{}{}.",
        get_throw_hand_position(a, throw_digit),
        get_catch_hand_position(b, catch_digit)
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

    fn get_origin_beats(&self) -> Vec<usize> {
        let len = self.siteswap.len();
        let mut catch_indices: Vec<usize> = vec![0; len];
        let mut origin_beats: Vec<usize> = vec![0; len];

        for i in 0..len {
            catch_indices[i] = (i + self.siteswap[i] as usize) % len;
        }

        for i in 0..len {
            origin_beats[catch_indices[i] % len] = i;
        }

        return origin_beats;
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
        let arcs = collect_cycle(&self.arc_positions, len);
        let zips = collect_cycle(&self.zip_positions, len);
        let swaps = collect_cycle(&self.siteswap, len);
        let origin_beats_short = &self.get_origin_beats();
        let origin_beats = collect_cycle(origin_beats_short, len);

        let mut result = "".to_string();

        println!("Origin beats: {:?}", origin_beats);

        // Throws go [arc, zip, zip, arc, ...].
        for i in 0..len {
            let (arc_t, _) = arcs[i];
            let (zip_t, _) = zips[i];
            let throw_digit = swaps[i];

            // Need the position of the NEXT CATCH, which was described on the beat it was thrown.
            let origin_beat = *origin_beats[(i + 1) % len];

            let (_, arc_c) = arcs[origin_beat];
            let (_, zip_c) = zips[origin_beat];
            let catch_digit = swaps[origin_beat];

            let arc = get_hand_positions(arc_t, arc_c, *throw_digit, *catch_digit);
            let zip = get_hand_positions(zip_t, zip_c, 1, 1);
            if i % 2 == 0 {
                result = format!("{}{}{}", result, arc, zip);
            } else {
                result = format!("{}{}{}", result, zip, arc);
            }
        }
        return result;
    }
}

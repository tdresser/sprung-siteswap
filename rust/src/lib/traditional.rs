use super::data::Pattern;

use super::data::Position;

#[derive(PartialEq)]
enum ThrowOrCatch {
    Throw,
    Catch,
}

fn get_hand_position(p: &Position, throw_or_catch: ThrowOrCatch, sprung_digit: u32) -> String {
    let mut x = if sprung_digit == 2 {
        20
    } else if throw_or_catch == ThrowOrCatch::Throw {
        10
    } else {
        30
    };
    if let Position::BottomOpposite | Position::TopOpposite = p {
        x = -x;
    }
    return if let Position::BottomNatural | Position::BottomOpposite = p {
        format!("({})", x)
    } else {
        format!("({}, 50)", x)
    };
}

fn collect_cycle<'a, A>(a: &'a Vec<A>, len: usize) -> Vec<&'a A> {
    return a.iter().cycle().take(len).collect::<Vec<_>>();
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
        let mut len = num::integer::lcm(
            num::integer::lcm(self.arc_positions.len(), self.zip_positions.len()),
            self.siteswap.len(),
        );
        // We need an even length for hand position computation to work.
        if len % 2 == 1 {
            len = len * 2;
        }
        let arcs = collect_cycle(&self.arc_positions, len);
        let zips = collect_cycle(&self.zip_positions, len);
        let swaps = collect_cycle(&self.siteswap, len);
        let origin_beats_short = &self.get_origin_beats();
        let origin_beats = collect_cycle(origin_beats_short, len);

        let mut result = "".to_string();

        println!("Origin beats: {:?}", origin_beats);

        // Throws go [arc, zip, zip, arc, ...].
        for i in 0..len {
            println!("i: {}", i);
            let (arc_t, _) = arcs[i];
            let (zip_t, _) = zips[i];
            let throw_digit = swaps[i];

            // Need the position of the NEXT CATCH, which was described on the beat it was thrown.
            let origin_beat = *origin_beats[(i + 1) % len];
            println!("Origin beat: {}", origin_beat);

            let (_, arc_c) = arcs[origin_beat];
            let (_, zip_c) = zips[origin_beat];
            let catch_digit = swaps[origin_beat];
            println!("arc c: {:?}", arc_c);
            println!("zip c: {:?}", zip_c);
            println!("catch: {}", catch_digit);

            // Needs to go (arc,zip)(zip,arc).
            let arc_t_str = get_hand_position(arc_t, ThrowOrCatch::Throw, *throw_digit);
            let zip_t_str = get_hand_position(zip_t, ThrowOrCatch::Throw, 1);
            let arc_c_str = get_hand_position(arc_c, ThrowOrCatch::Catch, *catch_digit);
            let zip_c_str = get_hand_position(zip_c, ThrowOrCatch::Catch, 1);

            if i % 2 == 0 {
                result = format!(
                    "{}{}{}.{}{}.",
                    result, arc_t_str, zip_c_str, zip_t_str, arc_c_str
                );
            } else {
                result = format!(
                    "{}{}{}.{}{}.",
                    result, zip_t_str, arc_c_str, arc_t_str, zip_c_str
                );
            }
        }
        return result;
    }
}

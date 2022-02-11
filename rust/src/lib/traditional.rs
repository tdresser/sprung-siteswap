use num::integer::Roots;

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
    fn lcm_len(&self) -> usize {
        return num::integer::lcm(
            num::integer::lcm(self.arc_positions.len(), self.zip_positions.len()),
            self.siteswap.len(),
        );
    }

    #[allow(dead_code)]
    pub fn get_bps(&self) -> u32 {
        let max_height = self.siteswap.iter().max().unwrap();
        return (max_height).sqrt() * 5;
    }

    #[allow(dead_code)]
    pub fn get_traditional_siteswap(&self) -> String {
        let mut result = String::new();
        for (i, digit) in self
            .siteswap
            .iter()
            .cycle()
            .take(self.lcm_len())
            .enumerate()
        {
            let crossing = if digit % 2 == 0 { "" } else { "x" };
            let digit_str = format!("{:x}{}", digit * 2, crossing);
            result = match i % 2 {
                0 => format!("{}({},2x)", result, digit_str),
                1 => format!("{}(2x,{})", result, digit_str),
                _ => unreachable!(),
            }
        }
        if (self.lcm_len()) % 2 == 1 {
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
        let len = self.lcm_len();
        println!("len: {}", len);
        let mut catch_indices: Vec<usize> = vec![0; len];
        let mut origin_beats: Vec<usize> = vec![0; len];
        let siteswap_repeated = collect_cycle(&self.siteswap, len);

        for i in 0..len {
            catch_indices[i] = (i + *siteswap_repeated[i] as usize) % len;
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
        let mut len = self.lcm_len();
        // We need an even length for hand position computation to work.
        // TODO: which of these is more correct?
        //if self.siteswap.len() % 2 == 1 {
        if len % 2 == 1 {
            len = len * 2;
        }
        let arcs = collect_cycle(&self.arc_positions, len);
        let zips = collect_cycle(&self.zip_positions, len);
        let swaps = collect_cycle(&self.siteswap, len);
        let origin_beats_short = &self.get_origin_beats();
        let origin_beats = collect_cycle(origin_beats_short, len);

        println!("origin_beats: {:?}", origin_beats);

        let mut result = "".to_string();

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

            // Needs to go (arc,zip)(zip,arc).
            let arc_t_str = get_hand_position(arc_t, ThrowOrCatch::Throw, *throw_digit);
            let zip_t_str = get_hand_position(zip_t, ThrowOrCatch::Throw, 1);
            let arc_c_str = get_hand_position(arc_c, ThrowOrCatch::Catch, *catch_digit);
            let zip_c_str = get_hand_position(zip_c, ThrowOrCatch::Catch, 1);

            println!("i: {}", i);
            println!("origin: {}", origin_beat);
            println!("zip_t: {:?}", zip_t);
            println!("zip_c: {:?}", zip_c);

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

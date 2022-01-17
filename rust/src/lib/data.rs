#[derive(Debug)]
pub struct Pattern {
    pub zip_positions: Vec<(Position, Position)>,
    pub nonzip_positions: Vec<(Position, Position)>,
    pub siteswap: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Position {
    BottomNatural,
    BottomOpposite,
    TopNatural,
    TopOpposite,
}

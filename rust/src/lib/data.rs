#[derive(Debug)]
pub struct Pattern {
    pub(super) zip_positions: Vec<(Position, Position)>,
    pub(super) nonzip_positions: Vec<(Position, Position)>,
    pub(super) siteswap: Vec<u32>,
    pub(super) error: Option<String>,
}

impl Pattern {
    pub fn new(s: &str) -> Pattern {
        return Pattern::parse(s);
    }

    pub fn zip_positions(&self) -> &Vec<(Position, Position)> {
        return &self.zip_positions;
    }

    pub fn nonzip_positions(&self) -> &Vec<(Position, Position)> {
        return &self.nonzip_positions;
    }

    pub fn siteswap(&self) -> &Vec<u32> {
        return &self.siteswap;
    }

    pub fn error(&self) -> &Option<String> {
        return &self.error;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Position {
    BottomNatural,
    BottomOpposite,
    TopNatural,
    TopOpposite,
}

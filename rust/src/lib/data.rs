#[derive(Debug)]
pub struct Pattern {
    pub(super) zip_positions: Positions,
    pub(super) arc_positions: Positions,
    pub(super) siteswap: Vec<u32>,
    pub(super) error: Option<String>,
}

impl Pattern {
    pub fn new(s: &str) -> Pattern {
        return Pattern::parse(s);
    }

    #[allow(dead_code)]
    pub fn zip_positions(&self) -> &Positions {
        return &self.zip_positions;
    }

    #[allow(dead_code)]
    pub fn arc_positions(&self) -> &Positions {
        return &self.arc_positions;
    }

    #[allow(dead_code)]
    pub fn siteswap(&self) -> &Vec<u32> {
        return &self.siteswap;
    }

    #[allow(dead_code)]
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

pub type Positions = Vec<(Position, Position)>;

pub const DEFAULT_POSITION: (Position, Position) =
    (Position::BottomNatural, Position::BottomNatural);

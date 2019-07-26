use crate::vm::tank::Tank;

#[derive(Clone, Debug)]
pub enum WinStatus {
    Won,
    Lost,
    Draw
}

#[derive(Debug)]
pub struct Shot {
    pub players: Vec<Tank>
}

#[derive(Debug)]
pub struct Report {
    pub player_names: Vec<String>,
    pub match_results: Vec<WinStatus>,
    pub replay: Vec<Shot>
}

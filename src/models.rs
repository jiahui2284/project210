use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Player {
    pub Player: String,
    pub Tm: String,
}
//This Model for cite two columns of csv  as “ Player” and “Tm”(Team)

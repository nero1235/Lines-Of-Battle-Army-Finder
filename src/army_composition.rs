use std::fmt::Alignment::Center;
use crate::prelude::*;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameModes{
    Clash,
    Combat,
    Battle,
    GrandBattle
}

impl GameModes
{
    pub fn get_maximum_rifles(&self) -> i32
    {
        match self
        {
            GameModes::Clash => 3,
            GameModes::Combat => 4,
            GameModes::Battle => 6,
            GameModes::GrandBattle => 8
        }
    }
    pub fn get_maximum_rockets(&self) -> i32
    {
        match self
        {
            GameModes::Clash => 1,
            GameModes::Combat => 1,
            GameModes::Battle => 2,
            GameModes::GrandBattle => 2
        }
    }
}
pub const SKIRMISHER_RATIOS: [(i32, i32, GameModes); 4] =
    [
        (1, 4, GameModes::Clash),
        (4, 8, GameModes::Combat),
        (2, 5, GameModes::Battle),
        (1, 3, GameModes::GrandBattle),
    ];

#[derive(Clone, Debug)]
pub struct ArmyComposition
{
    game_mode: GameModes,
    unit_roster: Vec<(Unit, i32)>
}

impl ArmyComposition
{
    pub fn new(game_mode: GameModes) -> Self
    {
        Self
        {
            game_mode,
            unit_roster: Vec::new()
        }
    }
    pub fn get_gold_cost(&self) -> i32
    {
        self.unit_roster.iter().fold(
            0,
            |acc, (k, v)| -> i32
            {
                acc + k.unit_stats.gold_cost * v
            }
        )
        
    }
    pub fn get_manpower_cost(&self) -> i32
    {
        self.unit_roster.iter().fold(
            0,
            |acc, (k, v)| -> i32
            {
                acc + k.unit_stats.manpower_cost * v
            }
        )
      
    }

}

use serde::{Serialize, Deserialize};
use crate::prelude::*;
use crate::units::unit_templates::Templates;
use crate::units::unitstats::UnitStats;

pub mod unit_templates;
pub mod unitstats;


#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum UnitTypes
{
    SkirmishInfantry,
    Infantry,
    Cavalry,
    Artillery
}
#[derive(Clone, Debug)]
pub struct Unit
{
    pub name: String,
    pub unit_type: UnitTypes,
    pub unit_stats: UnitStats
}
pub fn generate_units()->Vec<Unit>
{
    let templates = Templates::load();
    templates.to_units()
}
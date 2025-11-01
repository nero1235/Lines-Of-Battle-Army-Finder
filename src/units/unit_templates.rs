use crate::prelude::*;
use ron::de::from_reader;
use serde::{Serialize, Deserialize};
use std::fs::File;
use crate::units::unitstats::UnitStats;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnitTemplate
{
    pub name: String,
    pub unit_type: UnitTypes,
    pub unit_stats: UnitStats,
    
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Templates
{
    pub units: Vec<UnitTemplate>
}

impl Templates
{
    pub fn load() -> Self{
        let file = File::open("resources/unit_templates.ron").expect("Failed opening file");
        from_reader(file).expect("Failed loading templates")
    }
    pub fn to_units(&self) -> Vec<Unit>
    {
        let mut units = Vec::new();
        
        for i in self.units.iter()
        {
            units.push(Unit {
                name: i.name.clone(),
                unit_type: i.unit_type,
                unit_stats: i.unit_stats.clone()
            });
        }
        units
    }
    
}

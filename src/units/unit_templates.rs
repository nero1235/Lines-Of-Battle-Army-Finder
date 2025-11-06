use std::cell::LazyCell;
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
    pub units: Vec<UnitTemplate>,
}

impl Templates
{
    //Disgusting, but works? It's either this or pass around a Templates to every function, which is annoying!
    const GLOBAL_TEMPLATES: LazyCell<Self> = LazyCell::new(|| {
        let file = File::open("resources/unit_templates.ron").expect("Failed opening file");
        from_reader(file).expect("Failed loading templates")
    });
    const GLOBAL_DMGS: LazyCell<DamageResistanceTemplates> = LazyCell::new(||
        DamageResistanceTemplates::load()
    );
    pub fn load() -> Self{
        Self::GLOBAL_TEMPLATES.clone()


    }
    pub fn to_units(&self) -> Vec<Unit>
    {
        let mut units = Vec::new();
        
        for i in self.units.iter()
        {
            units.push(Unit {
                name: i.name.clone(),
                unit_type: i.unit_type,
                unit_stats: i.unit_stats.clone(),
                damage_resistances: Self::GLOBAL_DMGS.damage_resistances.iter().find(
                    |(k, _)| k == &i.name
                ).unwrap_or(&(i.name.clone(), Vec::new())).1.clone()

            });
        }
        units
    }
    pub fn get_unit_by_name(name: &str) -> Unit
    {
        let temp = &Self::GLOBAL_TEMPLATES;
        let template = temp.units.iter().find(|u| u.name == name).unwrap();
        Unit {
            name: template.name.clone(),
            unit_type: template.unit_type,
            unit_stats: template.unit_stats.clone(),
            damage_resistances: Self::GLOBAL_DMGS.damage_resistances.iter().find(
            |(k, _)| k == &template.name
            ).unwrap_or(&(template.name.clone(), Vec::new())).1.clone()
        }
    }
    
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct DamageResistanceTemplates
{
    damage_resistances: Vec<(String, Vec<(String, i32)>)>
}
impl DamageResistanceTemplates
{
    const GLOBAL_TEMPLATES: LazyCell<Self> = LazyCell::new(|| {
        let file = File::open("resources/damage_resistance_templates.ron").expect("Failed opening file");
        from_reader(file).expect("Failed loading templates")
    });
    pub fn load() -> Self{
        Self::GLOBAL_TEMPLATES.clone()
    }
}
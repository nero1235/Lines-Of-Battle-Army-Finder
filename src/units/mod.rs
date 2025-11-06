use serde::{Serialize, Deserialize};
use crate::prelude::*;
use crate::units::unit_templates::Templates;
use crate::units::unitstats::UnitStats;


pub mod unit_templates;
pub mod unitstats;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DamageSource
{
    Musket,
    ExplosiveShell,

}
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
    pub unit_stats: UnitStats,
    pub damage_resistances: Vec<(String, i32)>
}

unsafe impl Send for Unit {}
unsafe impl Sync for Unit {}


impl Unit
{
    pub const ORGANIZATION_DMG_RATIO_MELEE: i32 = 240;
    pub fn get_damage_modifier(&self, source: &str) -> i32
    {
        if let Some(dmg_val) = self.damage_resistances.iter().find(
            |(k, _)| k == source
        )
        {
            dmg_val.1
        }
        else
        {
            0
        }
    }

    //There is no such thing as 100% damage reduction, so division by zero should not occur
    pub fn get_ehp_vs_src(&self, source: &str) -> i32
    {
        let modifier = self.get_damage_modifier(source);
        debug_assert!(modifier != 100);
        ((self.unit_stats.hp as f32) / (1.0-((modifier as f32)/100.0))) as i32
    }
    pub fn get_dmg_over_band(&self, band: (i32, i32)) -> i32
    {
        if let Some(range) = &self.unit_stats.ranged_stats
        {
            range.get_average_damage_over_band(band)
        }
        else
        {
            0
        }
    }
    pub fn get_melee_org_damage(&self, dest_hp: i32) -> i32
    {
        let hp_damage = self.unit_stats.melee_attack as f32;
        let scaled_damage_pct = hp_damage / (dest_hp as f32);
        (Self::ORGANIZATION_DMG_RATIO_MELEE as f32 * scaled_damage_pct * 10.0) as i32
    }
    pub fn get_ranged_org_damage(&self, dest_hp: i32, range: i32) -> i32
    {
        if self.unit_stats.ranged_stats.is_none()
        {
            return 0;
        }
        let ranged_stats = self.unit_stats.ranged_stats.clone().unwrap();
        let hp_dmg = ranged_stats.get_damage_at_range(range);
        let scaled_damage_pct = hp_dmg as f32 / (dest_hp as f32);
        let org_dmg_ratio = ranged_stats.get_org_ratio_at_range(range);
        (org_dmg_ratio as f32 * scaled_damage_pct * 10.0) as i32
    }
    pub fn get_ranged_odp(&self, range: i32) -> i32
    {
        if let Some(ranged_stats) = self.unit_stats.ranged_stats.clone()
        {
            ranged_stats.get_org_ratio_at_range(range) * ranged_stats.get_damage_at_range(range)
        }
        else
        {
            0
        }
    }
    pub fn get_avg_odp_over_band(&self, band: (i32, i32)) -> i32
    {
        if let Some(ranged_stats) = self.unit_stats.ranged_stats.clone()
        {
            ranged_stats.get_average_odp_over_band(band)
        }
        else
        {
            0
        }
    }

}

pub fn generate_units()->Vec<Unit>
{
    let templates = Templates::load();
    templates.to_units()
}
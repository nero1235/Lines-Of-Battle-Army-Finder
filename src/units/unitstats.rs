use crate::prelude::*;
use serde::{Serialize, Deserialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnitStats
{
    pub manpower_cost: i32,
    pub gold_cost: i32,
    pub hp: i32,
    pub organization: i32,
    pub stamina: Option<i32>,
    pub melee_attack: i32,
    pub melee_defense: i32,
    pub charge_penetration: i32,
    pub charge_resistance: i32,
    pub ranged_stats: Option<RangedStats>, 
    pub has_skirmishers: bool
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RangedStats
{
    bands: Vec<RangedBand>
}
impl RangedStats
{
    pub fn new(bands: &[RangedBand]) -> RangedStats
    {
        RangedStats {
            bands: bands.to_vec()
        }
    }
    pub fn get_damage_at_range(&self, range: i32) -> i32
    {
        let mut in_range = false;
        let mut damage = 0;
        for band in &self.bands
        {
            if !in_range
            {
                if range >= band.start && range <= band.end
                {
                    in_range = true;
                    damage = band.get_raw_damage(range) as _;
                }
            }
        }
        damage
    }
    pub fn get_org_ratio_at_range(&self, range: i32) -> i32
    {
        let mut in_range = false;
        let mut ratio = 0;
        for band in &self.bands
        {
            if !in_range
            {
                if range >= band.start && range <= band.end
                {
                    in_range = true;
                    ratio = band.org_damage_ratio;
                }
            }
        }
        ratio
    }
    pub fn get_average_damage_over_band(&self, band: (i32, i32)) -> i32
    {
        if band.0 == band.1
        {
            return self.get_damage_at_range(band.0);
        }
            //Perhaps should panic instead, not willing to deal with this error here
        else if band.0 > band.1
        {
            return 0;
        }
        let mut damage = 0;
        for n in (band.0..band.1)
        {
            damage += self.get_damage_at_range(n) as i32;
        }
        damage / (band.1 - band.0)
    }
    pub fn get_average_odp_over_band(&self, band: (i32, i32)) -> i32
    {
        if band.0 == band.1
        {
            return self.get_damage_at_range(band.0) * self.get_org_ratio_at_range(band.0);
        }
        //Perhaps should panic instead, not willing to deal with this error here
        else if band.0 > band.1
        {
            return 0;
        }
        let mut odp_acc = 0;
        for n in (band.0..band.1)
        {
            odp_acc += self.get_org_ratio_at_range(n) * self.get_damage_at_range(n);
        }
        odp_acc / (band.1 - band.0)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RangedBand
{
    pub ranged_attack: i32,
    pub start: i32,
    pub end: i32,
    pub start_damage_modifier: i32,
    pub end_damage_modifier: i32, 
    pub org_damage_ratio: i32
}
impl RangedBand
{
    pub fn new(ranged_attack: i32, start: i32, end: i32, start_damage_modifier: i32, end_damage_modifier: i32, org_damage_ratio: i32) -> RangedBand
    {
        RangedBand {
            ranged_attack: ranged_attack,
            start: start,
            end: end,
            start_damage_modifier: start_damage_modifier,
            end_damage_modifier: end_damage_modifier,
            org_damage_ratio: org_damage_ratio
        }
    }
    pub fn get_raw_damage(&self, range: i32) -> f32 
    {
        let band_range = self.end - self.start;
        if range < self.start || range > self.end
        {
            return 0.0;
        }
            // The following should never occur and isn't worth handling and allowing a mistake of this form to propagate
        else if self.start == self.end
        {
            panic!()
        }
        let lerp_factor = (range as f32 - self.start as f32) / band_range as f32;
        let modifier_range = (self.start_damage_modifier - self.end_damage_modifier) as f32;
        let modifier_at_user_range = self.start_damage_modifier as f32 - (modifier_range * lerp_factor);
        self.ranged_attack as f32 * (1.0 + (modifier_at_user_range / 100.0))
    }

}


        

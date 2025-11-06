use std::cell::LazyCell;
use std::sync::Arc;
use crate::prelude::*;
use crate::units::unit_templates::Templates;

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
    pub fn get_skirmisher_ratio(&self) -> (i32, i32)
    {
        match self
        {
            GameModes::Clash => (SKIRMISHER_RATIOS[0].0, SKIRMISHER_RATIOS[0].1),
            GameModes::Combat => (SKIRMISHER_RATIOS[1].0, SKIRMISHER_RATIOS[1].1),
            GameModes::Battle => (SKIRMISHER_RATIOS[2].0, SKIRMISHER_RATIOS[2].1),
            GameModes::GrandBattle => (SKIRMISHER_RATIOS[3].0, SKIRMISHER_RATIOS[3].1)
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
    pub game_mode: GameModes,
    pub unit_roster: Vec<(Unit, i32)>
}

impl ArmyComposition
{
    const LAZY_SKIRM: LazyCell<Unit> = LazyCell::new(|| Templates::get_unit_by_name("Skirmishers"));
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
    pub fn get_unit_type_count(&self, unit_type: UnitTypes) -> i32
    {
        self.unit_roster.iter().filter(|(k, _)| k.unit_type == unit_type).fold(0,
            |acc, (_, v)|
                {
                    acc + v
                }
        )

    }
    pub fn get_unit_name_count(&self, unit_name: &str) -> i32
    {
        self.unit_roster.iter().filter(|(k, _)| k.name == unit_name).fold(0,
            |acc, (_, v)| acc + v
        )
    }
    pub fn add_units_by_name(&mut self, unit_name: &str, count: usize)
    {
        let unit_to_add = Templates::get_unit_by_name(unit_name);
        self.add_units(&unit_to_add, count);
    }
    pub fn add_unit_by_name(&mut self, unit_name: &str)
    {
        let unit_to_add = Templates::get_unit_by_name(unit_name);
        self.add_unit(&unit_to_add);

    }
    pub fn add_units(&mut self, unit_to_add: &Unit, count: usize)
    {
        for _ in 0..count
        {
            self.add_unit(unit_to_add);
        }
    }
    pub fn add_unit(&mut self, unit_to_add: &Unit)
    {
        if let Some(entry) = self.unit_roster.iter_mut().find(
            |(u, _)| u.name == unit_to_add.name)
        {
            entry.1 += 1;
        }
        else
        {
            self.unit_roster.push((unit_to_add.clone(), 1));
        }
        if unit_to_add.unit_stats.has_skirmishers
        {
            self.force_skirmisher_recalculation();
        }
    }
    pub fn remove_unit(&mut self, unit_to_remove: &Unit)
    {
        if let Some(entry) = self.unit_roster.iter_mut().find(|(u, _)| u.name == unit_to_remove.name)
        {
            entry.1 -= 1;
            if entry.1 == 0
            {
                self.unit_roster.retain(|(u, _)| u.name != unit_to_remove.name);
            }
        }
        if unit_to_remove.unit_stats.has_skirmishers
        {
            self.force_skirmisher_recalculation();
        }
    }
    pub fn get_total_hp(&self) -> i32
    {
        self.unit_roster.iter().fold(0, |acc, (k,v)| acc + k.unit_stats.hp * v)
    }
    pub fn get_total_organization(&self) -> i32
    {
        self.unit_roster.iter().fold(0, |acc, (k,v)| acc + k.unit_stats.organization * v)
    }
    pub fn get_ranged_damage_at_range(&self, range: i32) -> i32
    {
        self.unit_roster.iter().fold(
            0,
            |acc, (k, v)| -> i32
            {
                if let Some(ranged) = &k.unit_stats.ranged_stats
                {
                    acc + ranged.get_damage_at_range(range) * v
                }
                else
                {
                    acc
                }
            }
        )
    }
    pub fn get_average_damage_over_band(&self, band:(i32, i32)) -> i32
    {
        self.unit_roster.iter().fold(0,
                                     |acc, (k, v)| -> i32
                                         {
                                             if let Some(ranged) = &k.unit_stats.ranged_stats
                                             {
                                                 acc + ranged.get_average_damage_over_band(band) * v
                                             }
                                             else
                                             {
                                                 acc
                                             }
                                         }
        )
    }
    pub fn get_melee_attack(&self) -> i32
    {
        self.unit_roster.iter().fold
        (
            0,
            |acc, (k, v)| acc + k.unit_stats.melee_attack * v
        )
    }
    pub fn get_melee_attack_unit_types(&self, types: &[UnitTypes]) -> i32
    {
        self.unit_roster.iter().fold
        (
            0,
            |acc, (k, v)|
            {
                if types.contains(&k.unit_type)
                {
                    acc + k.unit_stats.melee_attack * v
                }
                else
                {
                    acc
                }
            }
        )
    }
    pub fn get_melee_defense(&self) -> i32
    {
        self.unit_roster.iter().fold(
            0,
            |acc, (k, v)| acc + k.unit_stats.melee_defense * v
        )
    }
    pub fn get_melee_defense_unit_types(&self, types: &[UnitTypes]) -> i32
    {
        self.unit_roster.iter().fold
        (
            0,
            |acc, (k, v)|
                {
                    if types.contains(&k.unit_type)
                    {
                        acc + k.unit_stats.melee_defense * v
                    }
                    else
                    {
                        acc
                    }
                }
        )
    }
    pub fn get_ehp_vs_source(&self, src: &str) -> i32
    {
        self.unit_roster.iter().fold(
            0,
            |acc, (k, v)| acc + k.get_ehp_vs_src(src) * v
        )
    }
    pub fn print_composition(&self)
    {
        println!("Gold: {}", self.get_gold_cost());
        println!("Manpower: {}", self.get_manpower_cost());
        println!("Average Damage over 20-40: {}", self.get_average_damage_over_band((20, 40)));
        println!("ODP over 20-40: {}", self.get_avg_odp_over_band((20, 40)));
        println!("Melee Attack: {}", self.get_melee_attack());
        println!("Melee Defense: {}", self.get_melee_defense());
        println!("Total HP: {}", self.get_total_hp());
        println!("Total Organization: {}", self.get_total_organization());
        println!("Infantry Stats (Melee Attack/Defense): {}/{}", self.get_melee_attack_unit_types(&[UnitTypes::Infantry]), self.get_melee_defense_unit_types(&[UnitTypes::Infantry]));
        println!("Cavalry Stats (Melee Attack/Defense): {}/{}", self.get_melee_attack_unit_types(&[UnitTypes::Cavalry]), self.get_melee_defense_unit_types(&[UnitTypes::Cavalry]));



        for (k, v) in self.unit_roster.iter()
        {
            println!("{}: {:?}", k.name, *v);
        }

    }
    pub fn merge_with(&self, other: &ArmyComposition) -> ArmyComposition
    {
        let mut result = self.clone();
        for (k, v) in other.unit_roster.iter()
        {
            result.add_units(k, *v as _);
        }
        result.force_skirmisher_recalculation();
        result
    }
    pub fn force_skirmisher_recalculation(&mut self)
    {
        self.unit_roster = self.unit_roster.iter().filter(
            |(k, v)| k.name != "Skirmishers"
        ).cloned().collect();
        let units_with_skirmishers = self.unit_roster.iter().filter(
            |(k, _)| k.unit_stats.has_skirmishers
        ).fold(0, |acc, (_, v)| acc + v);
        let skirmisher_ratio = self.game_mode.get_skirmisher_ratio();
        let skirmisher_count = (units_with_skirmishers / skirmisher_ratio.1) * skirmisher_ratio.0;

        self.unit_roster.push((Self::LAZY_SKIRM.clone(), skirmisher_count));
    }
    pub fn get_total_odp_at_rng(&self, range: i32) -> i32
    {
        self.unit_roster.iter().fold(
            0,
            |acc, (k, v)| acc + k.get_ranged_odp(range) * v
        )
    }
    pub fn get_avg_odp_over_band(&self, band: (i32, i32)) -> i32
    {
        self.unit_roster.iter().fold(
            0,
            |acc, (k, v)| acc + k.get_avg_odp_over_band(band) * v
        )
    }



}

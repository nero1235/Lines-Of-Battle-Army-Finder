use crate::prelude::*;
pub struct Constraint;

pub const SMALLEST_INFANTRY_GOLD_COST: i32 = 50;
pub const SMALLEST_INFANTRY_MANPOWER_COST: i32 = 125;
pub const SMALLEST_CAVALRY_GOLD_COST: i32 = 50;
pub const SMALLEST_CAVALRY_MANPOWER_COST: i32 = 125;
pub const SMALLEST_ARTILLERY_GOLD_COST: i32 = 125;
pub const SMALLEST_ARTILLERY_MANPOWER_COST: i32 = 25;

//TODO: Doing this the naive way is totally unacceptable, need some kind of recursive backtracking algorithm
//Need to ensure that we make sure this doesn't think skirmishers are purchasable either, will create a hard to find bug where it keeps adding skirmishers because their cost is 0 in the template.
pub fn search_for_army(preset: Option<ArmyComposition>, manpower_budget: i32, gold_budget: i32, constraints: Constraint, minimum_infantry: i32, minimum_cavalry: i32, minimum_artillery: i32, mode: GameModes, units: &Vec<Unit>) -> Vec<ArmyComposition>
{
    let maximum_infantry = std::cmp::min(
        (gold_budget / SMALLEST_INFANTRY_GOLD_COST),
        (manpower_budget / SMALLEST_INFANTRY_MANPOWER_COST)
    );
    let maximum_cavalry = std::cmp::min(
        (gold_budget / SMALLEST_CAVALRY_GOLD_COST),
        (manpower_budget / SMALLEST_CAVALRY_MANPOWER_COST)
    );
    let maximum_artillery = std::cmp::min(
        (gold_budget / SMALLEST_ARTILLERY_GOLD_COST),
        (manpower_budget / SMALLEST_ARTILLERY_MANPOWER_COST)
    );
    let maximum_rifles = mode.get_maximum_rifles();
    let maximum_rockets = mode.get_maximum_rockets();
    unimplemented!()
}

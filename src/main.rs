use crate::army_composition::GameModes;
use crate::army_searcher::minmaxer::{Constraint, ConstraintBuilder, MaximizationTargets};
use crate::army_searcher::minmaxer::MaximizationTargets::{DamageAtRange, DamageOverBand, EHPVsSource};
use crate::gui::GuiApp;
use crate::prelude::ArmyComposition;
use crate::prelude::GameModes::{Battle, Combat};
use crate::units::{generate_units, UnitTypes};


mod units;
mod army_composition;
mod army_searcher;
mod gui;

mod prelude{
    pub use crate::units::*;
    pub use crate::army_composition::*;
    pub use crate::army_searcher::*;
    pub use crate::gui::*;
}
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Errors
{
    InvalidInput,

}
fn main() 
{
    /*
    let mut default_army = ArmyComposition::new(Combat);
    default_army.add_units_by_name("Line Infantry", 18);
    default_army.add_units_by_name("Grenadiers", 4);
    default_army.add_units_by_name("Hussars", 4);
    default_army.add_units_by_name("Dragoons", 6);
    default_army.add_units_by_name("6-lb Foot Artillery", 4);
    default_army.add_units_by_name("12-lb Foot Artillery", 1);
    let constraint= ConstraintBuilder::new().maximization_targets(Some(MaximizationTargets::ODPOverBand(69, 180)))
        .min_inf(Some(10))
        .max_rifles(Some(4))
        .max_rockets(Some(1))
       // .min_melee_defense(Some((default_army.get_melee_defense()*50)/100))
        .min_melee_damage(Some((85 * default_army.get_melee_attack())/100))
        .min_damage_at_ranges(Some(
            (0..350).map(|i| ((95 * default_army.get_ranged_damage_at_range(i))/100, i)).collect()
        ))
        .min_melee_dmg_for_types(Some(
            vec![(UnitTypes::Cavalry, (85* default_army.get_melee_attack_unit_types(&[UnitTypes::Cavalry]))/100)]
        ))
        .min_melee_def_for_types(Some(
            vec![(UnitTypes::Cavalry, (85* default_army.get_melee_defense_unit_types(&[UnitTypes::Cavalry]))/100)]
        ))
        .min_ehp_vs_sources(Some(
            vec![
                ("Cavalry Sword".to_string(), default_army.get_ehp_vs_source("Cavalry Sword")),
                ("Musket".to_string(), default_army.get_ehp_vs_source("Musket")),
                ("Howitzer Canister".to_string(), default_army.get_ehp_vs_source("Howitzer Canister")),
                ("Bayonet".to_string(), default_army.get_ehp_vs_source("Bayonet")),
                ("12-lb Cannon Ball".to_string(), default_army.get_ehp_vs_source("12-lb Cannon Ball")),
                ("6-lb Cannon Ball".to_string(), default_army.get_ehp_vs_source("6-lb Cannon Ball")),
                ("12-lb Canister Fire".to_string(), default_army.get_ehp_vs_source("12-lb Canister Fire")),
                ("6-lb Canister Fire".to_string(), default_army.get_ehp_vs_source("6-lb Canister Fire")),
                ("Dragoon Sword".to_string(), default_army.get_ehp_vs_source("Dragoon Sword")),
                ("Cavalry Sabre".to_string(), default_army.get_ehp_vs_source("Cavalry Sabre")),
                ("Cavalry Lance".to_string(), default_army.get_ehp_vs_source("Cavalry Lance")),
                ("8-lb Cannon Ball".to_string(), default_army.get_ehp_vs_source("8-lb Cannon Ball")),
                ("8-lb Canister Fire".to_string(), default_army.get_ehp_vs_source("8-lb Canister Fire")),
            ].iter().map(|(k, v)| (k.clone(), (v*85)/100)).collect()
        ))
        .build();

    let default_armies = army_searcher::minmaxer::search_paths(
        None,
        6000,
        2575,
        constraint,
        GameModes::Combat,
        &generate_units(),
        3
        );
    let num_armies = default_armies.len();
    println!("Found {} armies better than default", num_armies);
    for a in default_armies
    {
        a.print_composition();
    }
    */
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Lines of Battle Optimizer",
        native_options,
        Box::new(|_cc| Ok(Box::new(GuiApp::new()))));

}

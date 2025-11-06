use std::cmp::min;
use crate::prelude::*;
use good_lp::*;
use good_lp::scip;
use crate::Errors;
use crate::units::unit_templates::Templates;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MaximizationTargets
{
    DamageAtRange(i32),
    ODPAtRange(i32),
    MeleeAttack,
    MeleeDefense,
    HP,
    Organization,
    DamageOverBand(i32, i32),
    ODPOverBand(i32, i32),
    EHPVsSource(String)
}
impl MaximizationTargets
{
    pub fn score_comp(&self, army_composition: &ArmyComposition) -> i32
    {
        match self
        {
            MaximizationTargets::DamageAtRange(range) => army_composition.get_ranged_damage_at_range(*range),
            MaximizationTargets::ODPAtRange(range) => army_composition.get_total_odp_at_rng(*range),
            MaximizationTargets::MeleeAttack => army_composition.get_melee_attack(),
            MaximizationTargets::MeleeDefense => army_composition.get_melee_defense(),
            MaximizationTargets::HP => army_composition.get_total_hp(),
            MaximizationTargets::Organization => army_composition.get_total_organization(),
            MaximizationTargets::DamageOverBand(start, end) => army_composition.get_average_damage_over_band((*start, *end)),
            MaximizationTargets::ODPOverBand(start, end) => army_composition.get_avg_odp_over_band((*start, *end)),
            MaximizationTargets::EHPVsSource(source) => army_composition.get_ehp_vs_source(source)

        }
    }
    pub fn score_unit(&self, unit: &Unit) -> i32
    {
        match self
        {
            MaximizationTargets::DamageAtRange(range) => if let Some(ranged_stats) = &unit.unit_stats.ranged_stats { ranged_stats.get_damage_at_range(*range) } else { 0 }
            MaximizationTargets::ODPAtRange(range) => if let Some(ranged_stats) = &unit.unit_stats.ranged_stats { unit.get_ranged_odp(*range) } else { 0 }
            MaximizationTargets::MeleeAttack => unit.unit_stats.melee_attack,
            MaximizationTargets::MeleeDefense => unit.unit_stats.melee_defense,
            MaximizationTargets::HP => unit.unit_stats.hp,
            MaximizationTargets::Organization => unit.unit_stats.organization,
            MaximizationTargets::DamageOverBand(start, end) => unit.get_dmg_over_band((*start, *end)),
            MaximizationTargets::ODPOverBand(start, end) => unit.get_avg_odp_over_band((*start, *end)),
            MaximizationTargets::EHPVsSource(source) => unit.get_ehp_vs_src(source)
        }
    }
}
impl Default for MaximizationTargets
{
    fn default() -> Self {
        MaximizationTargets::HP
    }
}
#[derive(Clone, Debug, Default)]
pub struct Constraint
{
    pub minimum_infantry: Option<i32>,
    pub minimum_cavalry: Option<i32>,
    pub minimum_artillery: Option<i32>,
    pub maximum_infantry: Option<i32>,
    pub maximum_cavalry: Option<i32>,
    pub maximum_artillery: Option<i32>,
    pub maximum_rifles: Option<i32>,
    pub maximum_rockets: Option<i32>,
    pub minimum_hp: Option<i32>,
    pub minimum_ehp_vs_sources: Option<Vec<(String, i32)>>,
    pub minimum_damage_at_ranges: Option<Vec<(i32, i32)>>,
    //pub minimum_odp_at_ranges: Option<Vec<(i32, i32)>>,

    pub minimum_melee_damage: Option<i32>,
    pub minimum_melee_damage_over_types: Option<Vec<(UnitTypes, i32)>>,
    pub minimum_melee_defense_over_types: Option<Vec<(UnitTypes, i32)>>,
    pub minimum_melee_defense: Option<i32>,
    pub maximization_targets: Option<MaximizationTargets>,
    pub allowed_units: Option<Vec<Unit>>,
    pub minimum_gold_target: Option<i32>,
    pub minimum_manpower_target: Option<i32>,
 }


impl Constraint
{
    pub fn army_violates(&self, army_comp: &ArmyComposition) -> bool
    {
        let infantry_count = army_comp.get_unit_type_count(UnitTypes::Infantry);
        let cavalry_count = army_comp.get_unit_type_count(UnitTypes::Cavalry);
        let artillery_count = army_comp.get_unit_type_count(UnitTypes::Artillery);
        let hp = army_comp.get_total_hp();

        if let Some(maximum_infantry) = self.maximum_infantry
        {
            if infantry_count > maximum_infantry {return true}
        }
        if let Some(maximum_cavalry) = self.maximum_cavalry
        {
            if cavalry_count > maximum_cavalry {return true}
        }
        if let Some(maximum_artillery) = self.maximum_artillery
        {
            if artillery_count > maximum_artillery {return true}

        }
        if let Some(minimum_infantry) = self.minimum_infantry
        {
            if infantry_count < minimum_infantry {return true};
        }
        if let Some(minimum_cavalry) = self.minimum_cavalry
        {
            if cavalry_count < minimum_cavalry {return true};
        }
        if let Some(minimum_artillery) = self.minimum_artillery
        {
            if artillery_count < minimum_artillery {return true}
        }
        if let Some(minimum_hp) = self.minimum_hp
        {
            if hp < minimum_hp {return true}

        }
        if let Some(minimum_damage_at_ranges) = &self.minimum_damage_at_ranges
        {
            for (k, v) in minimum_damage_at_ranges
            {
                if *k > army_comp.get_ranged_damage_at_range(*v)
                {
                    return true;
                }
            }
        }
        if let Some(minimum_melee_damage) = self.minimum_melee_damage
        {
            if army_comp.get_melee_attack() < minimum_melee_damage {return true}
        }
        if let Some(minimum_melee_defense) = self.minimum_melee_defense
        {
            if army_comp.get_melee_defense() < minimum_melee_defense {return true}

        }
        if let Some(minimum_gold_target) = self.minimum_gold_target
        {
            if army_comp.get_gold_cost() < minimum_gold_target {return true}
        }
        if let Some(minimum_manpower_target) = self.minimum_manpower_target
        {
            if army_comp.get_manpower_cost() < minimum_manpower_target {return true}

        }
        false
    }
    pub fn new(
        minimum_infantry: Option<i32>,
        minimum_cavalry: Option<i32>,
        minimum_artillery: Option<i32>,
        maximum_infantry: Option<i32>,
        maximum_cavalry: Option<i32>,
        maximum_artillery: Option<i32>,
        maximum_rifles: Option<i32>,
        maximum_rockets: Option<i32>,
        minimum_hp: Option<i32>,
        minimum_ehp_vs_sources: Option<Vec<(String, i32)>>,
        minimum_damage_at_ranges: Option<Vec<(i32, i32)>>,
        minimum_melee_damage: Option<i32>,
        minimum_melee_damage_over_types: Option<Vec<(UnitTypes, i32)>>,
        minimum_melee_defense: Option<i32>,
        minimum_melee_defense_over_types: Option<Vec<(UnitTypes, i32)>>,
        maximization_targets: Option<MaximizationTargets>,
        allowed_units: Option<Vec<Unit>>,
        minimum_gold_target: Option<i32>,
        minimum_manpower_target: Option<i32>


    ) -> Self
    {
        Self{
            minimum_infantry,
            minimum_cavalry,
            minimum_artillery,
            maximum_infantry,
            maximum_cavalry,
            maximum_artillery,
            maximum_rifles,
            maximum_rockets,
            minimum_hp,
            minimum_ehp_vs_sources,
            minimum_damage_at_ranges,
            minimum_melee_damage,
            minimum_melee_damage_over_types,
            minimum_melee_defense,
            minimum_melee_defense_over_types,
            maximization_targets,
            allowed_units: allowed_units.clone(),
            minimum_gold_target,
            minimum_manpower_target
        }
    }

    pub fn try_push_ehp_vs_source(&mut self, source: &str, ehp: i32) -> Result<(), Errors>
    {
        let valid_unit =
            {
                generate_units().iter().any(
                    |u| u.name == source
                )
            };
        if !valid_unit {
            return Err(Errors::InvalidInput);
        }
        else {
            if let Some(v) = &mut self.minimum_ehp_vs_sources {
                v.push((source.to_string(), ehp));
                Ok(())
            }
            else {
                self.minimum_ehp_vs_sources = Some(vec![(source.to_string(), ehp)]);
                Ok(())
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct ConstraintBuilder
{
    inner: Constraint,
}

//No we will not remove the clones. This is an infrequent and
//One time operation. Unless profiling indicates this is a substantial bottleneck, this won't be altered.
impl ConstraintBuilder
{
    pub fn new() -> Self
    {
        Self{inner: Constraint::default()}
    }
    pub fn min_inf(&mut self, minimum_infantry: Option<i32>) -> Self
    {
        self.inner.minimum_infantry = minimum_infantry;
        self.clone()
    }
    pub fn min_cav(&mut self, minimum_cavalry: Option<i32>) -> Self
    {
        self.inner.minimum_cavalry = minimum_cavalry;
        self.clone()
    }
    pub fn min_art(&mut self, minimum_artillery: Option<i32>) -> Self
    {
        self.inner.minimum_artillery = minimum_artillery;
        self.clone()
    }
    pub fn max_inf(&mut self, maximum_infantry: Option<i32>) -> Self
    {
        self.inner.maximum_infantry = maximum_infantry;
        self.clone()
    }
    pub fn max_cav(&mut self, maximum_cavalry: Option<i32>) -> Self
    {
        self.inner.maximum_cavalry = maximum_cavalry;
        self.clone()
    }
    pub fn max_art(&mut self, maximum_artillery: Option<i32>) -> Self
    {
        self.inner.maximum_artillery = maximum_artillery;
        self.clone()
    }
    pub fn max_rifles(&mut self, maximum_rifles: Option<i32>) -> Self
    {
        self.inner.maximum_rifles = maximum_rifles;
        self.clone()
    }
    pub fn max_rockets(&mut self, maximum_rockets: Option<i32>) -> Self
    {
        self.inner.maximum_rockets = maximum_rockets;
        self.clone()
    }
    pub fn min_hp(&mut self, minimum_hp: Option<i32>) -> Self
    {
        self.inner.minimum_hp = minimum_hp;
        self.clone()
    }
    pub fn min_ehp_vs_sources(&mut self, minimum_ehp_vs_sources: Option<Vec<(String, i32)>>) -> Self
    {
        self.inner.minimum_ehp_vs_sources = minimum_ehp_vs_sources;
        self.clone()
    }
    pub fn min_damage_at_ranges(&mut self, minimum_damage_at_ranges: Option<Vec<(i32, i32)>>) -> Self
    {
        self.inner.minimum_damage_at_ranges = minimum_damage_at_ranges;
        self.clone()
    }
    pub fn min_melee_damage(&mut self, minimum_melee_damage: Option<i32>) -> Self
    {
        self.inner.minimum_melee_damage = minimum_melee_damage;
        self.clone()
    }
    pub fn min_melee_dmg_for_types(&mut self, minimum_damages_for_types: Option<Vec<(UnitTypes, i32)>>) -> Self
    {
        self.inner.minimum_melee_damage_over_types = minimum_damages_for_types;
        self.clone()
    }
    pub fn min_melee_defense(&mut self, minimum_melee_defense: Option<i32>) -> Self
    {
        self.inner.minimum_melee_defense = minimum_melee_defense;
        self.clone()
    }
    pub fn min_melee_def_for_types(&mut self, minimum_defenses_for_types: Option<Vec<(UnitTypes, i32)>>) -> Self
    {
        self.inner.minimum_melee_defense_over_types = minimum_defenses_for_types;
        self.clone()
    }
    pub fn maximization_targets(&mut self, maximization_targets: Option<MaximizationTargets>) -> Self
    {
        self.inner.maximization_targets = maximization_targets;
        self.clone()
    }
    pub fn allowed_units(&mut self, allowed_units: Option<Vec<Unit>>) -> Self
    {
        self.inner.allowed_units = allowed_units;
        self.clone()
    }
    pub fn min_gold_target(&mut self, minimum_gold_target: Option<i32>) -> Self
    {
        self.inner.minimum_gold_target = minimum_gold_target;
        self.clone()
    }
    pub fn min_manpower_target(&mut self, minimum_manpower_target: Option<i32>) -> Self
    {
        self.inner.minimum_manpower_target = minimum_manpower_target;
        self.clone()
    }
    pub fn build(&mut self) -> Constraint
    {
        self.inner.clone()
    }



}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Budget
{
    pub gold: i32,
    pub manpower: i32
}


fn cap_named(model: &mut (impl SolverModel), x: &Vec<Variable>, units: &[Unit], name:&str, cap: i32)
{
    if cap >= 0 {
        let expr: Expression = (0..units.len())
            .filter(|&i| units[i].name == name)
            .map(|i| x[i])
            .sum();
        model.add_constraint(expr.leq(cap));
    }
}
fn add_type_bounds(model: &mut (impl SolverModel), x: &Vec<Variable>, units: &[Unit], c: &Constraint)
{
    let by = |ty| (0..units.len()).filter(
        move |&i| units[i].unit_type == ty).map(move |i| x[i]).fold(Expression::from(0), |a, b| a + b);
    if let Some(v) = c.minimum_infantry
    {
        model.add_constraint(by(UnitTypes::Infantry).geq(v));
    }
    if let Some(v) = c.maximum_infantry
    {
        model.add_constraint(by(UnitTypes::Infantry).leq(v));
    }
    if let Some(v) = c.minimum_cavalry
    {
        model.add_constraint(by(UnitTypes::Cavalry).geq(v));
    }
    if let Some(v) = c.maximum_cavalry
    {
        model.add_constraint(by(UnitTypes::Cavalry).leq(v));
    }
    if let Some(v) = c.minimum_artillery
    {
        model.add_constraint(by(UnitTypes::Artillery).geq(v));
    }
    if let Some(v) = c.maximum_artillery
    {
        model.add_constraint(by(UnitTypes::Artillery).leq(v));
    }
}

fn add_stat_floor_with_skirm(model: &mut impl SolverModel, x: &Vec<Variable>, y_sk:&Variable, units: &[Unit], sk:&Unit, c: &Constraint)
{
    if let Some(v) = c.minimum_hp {
        let e: Expression = (0..units.len())
            .map(|i| units[i].unit_stats.hp * x[i]).fold(Expression::from(0), |a, b| a + b) + sk.unit_stats.hp * *y_sk;
        model.add_constraint(e.geq(v));
    }
    if let Some(v) = &c.minimum_damage_at_ranges {

        for (damage, range) in v {

            let e_units: Expression = (0..units.len())
                .map(|i| {
                    if let Some(r) = &units[i].unit_stats.ranged_stats {
                        r.get_damage_at_range(*range) * x[i]
                    } else { 0 * x[i] }
                })
                .fold(Expression::from(0), |a, b| a + b);

            let e_sk = if let Some(r) = &sk.unit_stats.ranged_stats {

                r.get_damage_at_range(*range) * *y_sk

            } else { 0 * *y_sk };

            model.add_constraint((e_units + e_sk).geq(*damage));

        }

    }

    if let Some(v) = &c.minimum_ehp_vs_sources {

        for (source, min_ehp) in v {

            let e: Expression = (0..units.len())

                .map(|i| units[i].get_ehp_vs_src(source) * x[i])

                .fold(Expression::from(0), |a, b| a + b) + sk.get_ehp_vs_src(source) * *y_sk;

            model.add_constraint(e.geq(*min_ehp));

        }

    }

    if let Some(v) = c.minimum_melee_damage {

        let e: Expression = (0..units.len())

            .map(|i| units[i].unit_stats.melee_attack * x[i])

            .fold(Expression::from(0), |a, b| a + b) + sk.unit_stats.melee_attack * *y_sk;

        model.add_constraint(e.geq(v));

    }
    //We will not accept skirmisher and therefore rifles as a valid type to limit the headaches
    //TODO revisit this possibly
    if let Some(v) = &c.minimum_melee_damage_over_types
    {
        for (unit_type, min_dmg) in v
        {
            let e: Expression = (0..units.len())
                .filter(|&i| units[i].unit_type == *unit_type)
                .map(|i| units[i].unit_stats.melee_attack * x[i])
                .fold(Expression::from(0), |a, b| a + b);
            model.add_constraint(e.geq(*min_dmg));

        }
    }

    if let Some(v) = c.minimum_melee_defense {

        let e: Expression = (0..units.len())

            .map(|i| units[i].unit_stats.melee_defense * x[i])

            .fold(Expression::from(0), |a, b| a + b) + sk.unit_stats.melee_defense * *y_sk;

        model.add_constraint(e.geq(v));

    }
    if let Some(v) = &c.minimum_melee_defense_over_types
    {
        for (unit_type, min_def) in v
        {
            let e: Expression = (0..units.len())
                .filter(|&i| units[i].unit_type == *unit_type)
                .map(|i| units[i].unit_stats.melee_defense * x[i])
                .fold(Expression::from(0), |a, b| a + b);
            model.add_constraint(e.geq(*min_def));


        }
    }
}
/*
Let:

	•	den = the denominator of the mode’s ratio (e.g. 5 for 2:5).

	•	num = the numerator (e.g. 2 for 2:5).

	•	x[i] = our existing integer decision variables.

	•	z = Σ x[i] over all units with has_skirmishers == true (an expression, not a var).

	•	t = a new nonnegative integer variable equal to ⌊z / den⌋.

	•	y = a new nonnegative integer variable equal to the number of skirmishers.


Then we optimize with these constraints:


z - den * t ≥ 0

z - den * t ≤ den - 1

y - num * t = 0
*/

pub fn optimize_k(constraints: &Constraint, budget: &Budget, units: &[Unit], target: MaximizationTargets, k: usize, hard_cap: i32, mode: GameModes) -> Vec<ArmyComposition>
{
    let n = units.len();
    let mut results = Vec::new();
    let mut ub_obj: Option<i32> = None;

    let (num, den) = mode.get_skirmisher_ratio();
    let sk_unit = Templates::get_unit_by_name("Skirmishers");
    let eligible_cnt = units.iter().filter(|u| u.unit_stats.has_skirmishers).count() as i32;
    let t_ub = ((eligible_cnt * hard_cap) / i32::max(1, den)).max(0);
    let y_ub = (t_ub * num).max(0);
    for _ in 0..k
    {
        let mut vars = variables!();
        let variable_def = variable().integer().min(0).max(hard_cap);
        let x = vars.add_vector(variable_def, (0..n).len());
        let t = vars.add(variable().integer().min(0).max(t_ub));
        let y = vars.add(variable().integer().min(0).max(y_ub));
        let z: Expression = (0..n)
            .filter(|&i| units[i].unit_stats.has_skirmishers)
            .map(|i| x[i])
            .fold(Expression::from(0), |a, b| a + b);

        let base_value: Expression = (0..n)
            .map(|i| target.score_unit(&units[i]) * x[i])
            .sum();
        let value_with_sk = base_value + target.score_unit(&sk_unit) * y;
        let mut model = scip(vars.maximise(value_with_sk.clone()));
        model.add_constraint(
            (0..n).map(
                |i| (units[i].unit_stats.gold_cost) * x[i]).fold(Expression::from(0), |a, b| a + b).leq(budget.gold));
        model.add_constraint(
            (0..n).map(
                |i| (units[i].unit_stats.manpower_cost) * x[i]).fold(Expression::from(0), |a, b| a + b).leq(budget.manpower));
        model.add_constraint((z.clone() - den * t).geq(0));
        model.add_constraint((z.clone() - den * t).leq(den-1));
        model.add_constraint((y.clone() - num * t).eq(0));

        add_type_bounds(&mut model, &x, units, constraints);
        cap_named(&mut model, &x, units, "Rifles", constraints.maximum_rifles.unwrap_or(0));
        cap_named(&mut model, &x, units, "Rocket battery", constraints.maximum_rockets.unwrap_or(0));

        if let Some(allowed_units) = &constraints.allowed_units
        {
            let allowed_names: std::collections::HashSet<_> = allowed_units.iter().map(
                |u| u.name.clone()).collect();
            for i in 0..n {
                if !allowed_names.contains(&units[i].name) {
                    model.add_constraint(x[i].into_expression().leq(0));

                }
            }

        }
        add_stat_floor_with_skirm(&mut model, &x, &y, units, &sk_unit, constraints);
        if let Some(prev_best) = ub_obj {
            model.add_constraint(value_with_sk.clone().leq(prev_best - 1));
        }
        let sol = match model.solve() {
            Ok(s) => s,
            Err(_) => break
        };
        let best_val = sol.eval(&value_with_sk) as _;
        ub_obj = Some(best_val);
        let mut comp = ArmyComposition::new(mode);
        for i in 0..n
        {
            let count = sol.value(x[i]) as i32;
            for _ in 0..count
            {
                comp.add_unit(&units[i])
            }
        }
        results.push(comp);
    }
    //This shouldn't ever be neccesary, but whatever
    /*results.sort_by(
        |a, b|
            {
                let maximization_target = constraints.clone().maximization_targets.unwrap();
                maximization_target.score_comp(a).cmp(&maximization_target.score_comp(b))
            });*/
    results

}
fn apply_preset(army_comp: &ArmyComposition, budget: &mut Budget, constraints: &mut Constraint)
{
    if let Some(min_infantry) = &mut constraints.minimum_infantry
    {
        *min_infantry -= army_comp.get_unit_type_count(UnitTypes::Infantry);
    }
    if let Some(min_cavalry) = &mut constraints.minimum_cavalry
    {
        *min_cavalry -= army_comp.get_unit_type_count(UnitTypes::Cavalry);
    }
    if let Some(min_artillery) = &mut constraints.minimum_artillery
    {
        *min_artillery -= army_comp.get_unit_type_count(UnitTypes::Artillery);
    }
    if let Some(max_inf) = &mut constraints.maximum_infantry
    {
        *max_inf -= army_comp.get_unit_type_count(UnitTypes::Infantry);
    }
    if let Some(max_cav) = &mut constraints.maximum_cavalry
    {
        *max_cav -= army_comp.get_unit_type_count(UnitTypes::Cavalry);
    }
    if let Some(max_art) = &mut constraints.maximum_artillery
    {
        *max_art -= army_comp.get_unit_type_count(UnitTypes::Artillery);
    }
    if let Some(max_rifles) = &mut constraints.maximum_rifles
    {
        *max_rifles -= army_comp.get_unit_name_count("Rifles");
    }
    if let Some(max_rockets) = &mut constraints.maximum_rockets
    {
        *max_rockets -= army_comp.get_unit_name_count("Rocket battery");
    }
    if let Some(min_hp) = &mut constraints.minimum_hp
    {
        *min_hp -= army_comp.get_total_hp();
    }
    if let Some(min_ehp_vs_sources) = &mut constraints.minimum_ehp_vs_sources
    {
        for (source, hp) in min_ehp_vs_sources.iter_mut()
        {
            *hp -= army_comp.get_ehp_vs_source(source);
        }
    }
    if let Some(min_gold) = &mut constraints.minimum_gold_target
    {
        *min_gold -= army_comp.get_gold_cost();
    }
    if let Some(min_manpower) = &mut constraints.minimum_manpower_target
    {
        *min_manpower -= army_comp.get_manpower_cost();
    }
    if let Some(min_melee_damage) = &mut constraints.minimum_melee_damage
    {
        *min_melee_damage -= army_comp.get_melee_attack();
    }
    if let Some(min_melee_damage_over_types) = &mut constraints.minimum_melee_damage_over_types
    {
        for (unit_type, damage) in min_melee_damage_over_types.iter_mut()
        {
            *damage -= army_comp.get_melee_attack_unit_types(&[*unit_type]);
        }
    }
    if let Some(min_melee_defense) = &mut constraints.minimum_melee_defense
    {
        *min_melee_defense -= army_comp.get_melee_defense();
    }
    if let Some(min_melee_defense_over_types) = &mut constraints.minimum_melee_defense_over_types
    {
        for (unit_type, defense) in min_melee_defense_over_types.iter_mut()
        {
            *defense -= army_comp.get_melee_defense_unit_types(&[*unit_type]);
        }
    }
    if let Some(min_damage_at_ranges) = &mut constraints.minimum_damage_at_ranges
    {
        for (required_damage, range) in min_damage_at_ranges.iter_mut()
        {
            *required_damage -= army_comp.get_ranged_damage_at_range(*range);
        }
    }
    budget.gold -= army_comp.get_gold_cost();
    budget.manpower -= army_comp.get_manpower_cost();
}


pub fn search_paths(preset: Option<ArmyComposition>, manpower_budget: i32, gold_budget: i32, constraints: Constraint, mode: GameModes, units: &Vec<Unit>, num_comps: usize) -> Vec<ArmyComposition>
{
    if constraints.maximization_targets.is_none()
    {
        return search_for_army(preset, manpower_budget, gold_budget, constraints, mode, units, num_comps);
    }
    let mut budget = Budget{gold: gold_budget, manpower: manpower_budget};
    let mut constraint = constraints.clone();
    if let Some(preset) = &preset
    {
        apply_preset(preset, &mut budget, &mut constraint);
    }
    let mut result =  search_optimal(preset.clone(), budget.manpower, budget.gold, constraint, mode, units, num_comps);
    if let Some(preset) = &preset
    {
        for i in result.iter_mut()
        {
            *i = i.merge_with(preset);
        }
    }
    result
}
pub fn search_optimal(preset: Option<ArmyComposition>, manpower_budget: i32, gold_budget: i32, constraints: Constraint, mode: GameModes, units: &Vec<Unit>, num_comps: usize) -> Vec<ArmyComposition>
{
    let mut constraints = constraints;
    let mut purchasable_units: Vec<Unit> = units
        .iter()
        .filter(|u| u.unit_stats.gold_cost > 0 || u.unit_stats.manpower_cost > 0)
        .map(|u| u.clone())
        .collect();
    if let Some(purchasable_unit_limit) = &constraints.allowed_units.clone()
    {
        purchasable_units = purchasable_units.iter().filter(
            |u| purchasable_unit_limit.iter().any(|x| x.name == u.name)
        ).cloned().collect();
    }
    let get_cheapest_of_unit_by_gold = |units: &[Unit], unit_type: UnitTypes|
        {
            units.iter().filter(|x| x.unit_type == unit_type).min_by(|x, y| x.unit_stats.gold_cost.cmp(&y.unit_stats.gold_cost)).cloned().unwrap().unit_stats.gold_cost
        };
    let get_cheapest_of_unit_by_manpower = |units: &[Unit], unit_type: UnitTypes|
        {
            units.iter().filter(|x| x.unit_type == unit_type).min_by(|x, y| x.unit_stats.manpower_cost.cmp(&y.unit_stats.manpower_cost)).cloned().unwrap().unit_stats.manpower_cost
        };
    let maximum_infantry = min(gold_budget / get_cheapest_of_unit_by_gold(&purchasable_units, UnitTypes::Infantry),
                               manpower_budget / get_cheapest_of_unit_by_manpower(&purchasable_units, UnitTypes::Infantry));
    let maximum_cavalry = min(gold_budget / get_cheapest_of_unit_by_gold(&purchasable_units, UnitTypes::Cavalry),
                              manpower_budget /get_cheapest_of_unit_by_manpower(&purchasable_units, UnitTypes::Cavalry));
    let maximum_artillery = min(gold_budget / get_cheapest_of_unit_by_gold(&purchasable_units, UnitTypes::Artillery),
                                manpower_budget / get_cheapest_of_unit_by_manpower(&purchasable_units, UnitTypes::Artillery));
    let maximum_rifles = mode.get_maximum_rifles();
    let maximum_rockets = mode.get_maximum_rockets();
    if constraints.maximum_rockets.is_none()
    {
        constraints.maximum_rockets = Some(maximum_rockets);
    }
    if constraints.maximum_rifles.is_none()
    {
        constraints.maximum_rifles = Some(maximum_rifles);
    }
    let hard_cap = i32::max(maximum_infantry, i32::max(maximum_artillery, maximum_cavalry));
    let budget = Budget{gold: gold_budget, manpower: manpower_budget};
    optimize_k(&constraints, &budget, &purchasable_units, constraints.clone().maximization_targets.unwrap(), num_comps, hard_cap, mode)
}
pub fn search_for_army(preset: Option<ArmyComposition>, manpower_budget: i32, gold_budget: i32, constraints: Constraint, mode: GameModes, units: &Vec<Unit>, num_comps: usize) -> Vec<ArmyComposition>
{
    let mut purchasable_units: Vec<Unit> = units
        .iter()
        .filter(|u| u.unit_stats.gold_cost > 0 || u.unit_stats.manpower_cost > 0)
        .map(|u| u.clone())
        .collect();
    if let Some(purchasable_unit_limit) = &constraints.allowed_units.clone()
    {
        purchasable_units = purchasable_units.iter().filter(
            |u| purchasable_unit_limit.iter().any(|x| x.name == u.name)
        ).cloned().collect();
    }
    let get_cheapest_of_unit_by_gold = |units: &[Unit], unit_type: UnitTypes|
        {
            units.iter().filter(|x| x.unit_type == unit_type).min_by(|x, y| x.unit_stats.gold_cost.cmp(&y.unit_stats.gold_cost)).cloned().unwrap().unit_stats.gold_cost
        };
    let get_cheapest_of_unit_by_manpower = |units: &[Unit], unit_type: UnitTypes|
        {
            units.iter().filter(|x| x.unit_type == unit_type).min_by(|x, y| x.unit_stats.manpower_cost.cmp(&y.unit_stats.manpower_cost)).cloned().unwrap().unit_stats.manpower_cost
        };
    let maximum_infantry = min(gold_budget / get_cheapest_of_unit_by_gold(&purchasable_units, UnitTypes::Infantry),
        manpower_budget / get_cheapest_of_unit_by_manpower(&purchasable_units, UnitTypes::Infantry));
    let maximum_cavalry = min(gold_budget / get_cheapest_of_unit_by_gold(&purchasable_units, UnitTypes::Cavalry),
        manpower_budget /get_cheapest_of_unit_by_manpower(&purchasable_units, UnitTypes::Cavalry));
    let maximum_artillery = min(gold_budget / get_cheapest_of_unit_by_gold(&purchasable_units, UnitTypes::Artillery),
        manpower_budget / get_cheapest_of_unit_by_manpower(&purchasable_units, UnitTypes::Artillery));
    let maximum_rifles = mode.get_maximum_rifles();
    let maximum_rockets = mode.get_maximum_rockets();
    let mut valid_armies : Vec<ArmyComposition> = Vec::new();
    let mut current_composition = if let Some(composition) = preset {composition} else {ArmyComposition::new(mode)};
    find_combination_recursive(
        &purchasable_units,
        0,
        &mut current_composition.clone(),
        current_composition.get_unit_type_count(UnitTypes::Infantry),
        current_composition.get_unit_type_count(UnitTypes::Cavalry),
        current_composition.get_unit_type_count(UnitTypes::Artillery),
        current_composition.get_unit_name_count("Rocket battery"),
        current_composition.get_unit_name_count("Rifles"),
        &mut valid_armies,
        manpower_budget,
        gold_budget,
        maximum_rifles,
        maximum_rockets,
        &constraints,
        num_comps,

    );
    valid_armies


}

fn find_combination_recursive(
    purchasable_units: &Vec<Unit>,
    unit_index: usize,
    current_composition: &mut ArmyComposition,
    infantry_count: i32,
    cavalry_count: i32,
    artillery_count: i32,
    rocket_count: i32,
    rifles_count: i32,
    valid_armies: &mut Vec<ArmyComposition>,
    manpower_budget: i32,
    gold_budget: i32,
    maximum_rifles: i32,
    maximum_rockets: i32,
    constraint: &Constraint,
    num_comps: usize
)
{
    if valid_armies.len() >= num_comps
    {
        return
    }

    //Base case
    if unit_index == purchasable_units.len()
    {
        if !constraint.army_violates(current_composition)
        {
            valid_armies.push(current_composition.clone());
        }
        return;
    }

    let unit_to_try = &purchasable_units[unit_index];
    let can_add_unit =
        {
            let new_gold = gold_budget - unit_to_try.unit_stats.gold_cost;
            let new_manpower = manpower_budget - unit_to_try.unit_stats.manpower_cost;
            let within_budget = new_gold >= 0 && new_manpower >= 0;

            let mut special_limits_ok = true;
            if unit_to_try.name == "Rifles" {
                if current_composition.get_unit_name_count("Rifles") >= maximum_rifles
                {
                    special_limits_ok = false;
                }
            }
            else if unit_to_try.name == "Rocket battery"
            {
                if current_composition.get_unit_name_count("Rocket battery") >= maximum_rockets
                {
                    special_limits_ok = false;
                }
            }
            within_budget && special_limits_ok
        };
    if can_add_unit
    {
        current_composition.add_unit(unit_to_try);
        find_combination_recursive(
            purchasable_units,
            unit_index,
            current_composition,
            infantry_count,
            cavalry_count,
            artillery_count,
            rocket_count,
            rifles_count,
            valid_armies,
            manpower_budget - unit_to_try.unit_stats.manpower_cost,
            gold_budget - unit_to_try.unit_stats.gold_cost,
            maximum_rifles,
            maximum_rockets,
            &constraint,
            num_comps
        );
        current_composition.remove_unit(unit_to_try);
    }
    find_combination_recursive(
        purchasable_units,
        unit_index + 1,
        current_composition,
        infantry_count,
        cavalry_count,
        artillery_count,
        rocket_count,
        rifles_count,
        valid_armies,
        manpower_budget,
        gold_budget,
        maximum_rifles,
        maximum_rockets,
        &constraint,
        num_comps
    )

}

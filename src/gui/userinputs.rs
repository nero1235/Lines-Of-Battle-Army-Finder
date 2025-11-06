use crate::prelude::{GameModes, UIState, minmaxer::MaximizationTargets};

pub fn user_edits(ui: &mut egui::Ui, state: &mut UIState)
{
    ui.heading("Variables");
    ui.horizontal(|ui| {
        ui.label("Gold Budget: ");
        ui.add(egui::Slider::new(&mut state.budget.gold, 0..=30000).text("Integer Value"));
    });
    ui.horizontal(|ui| {
        ui.label("Manpower Budget: ");
        ui.add(egui::Slider::new(&mut state.budget.manpower, 0..=30000).text("Integer Value"));
    });
    ui.horizontal(|ui| {
        ui.label("Game Mode: ");
        egui::ComboBox::from_label("Game Mode")
                    .selected_text(format!("{:?}", state.mode))
                    .show_ui(ui,
                        |ui|
                            {
                                ui.selectable_value(&mut state.mode, GameModes::Clash, "Clash");
                                ui.selectable_value(&mut state.mode, GameModes::Combat, "Combat");
                                ui.selectable_value(&mut state.mode, GameModes::Battle, "Batlle");
                                ui.selectable_value(&mut state.mode, GameModes::GrandBattle, "Grand Battle");
                            }

                    );
    });

    ui.heading("Constraints");

    // Macro for creating a checkbox-controlled slider for a constraint
    macro_rules! constraint_slider {
        ($label:expr, $field:ident, $range:expr) => {
            ui.horizontal(|ui| {
                let mut enabled = state.constraints.$field.is_some();
                ui.checkbox(&mut enabled, $label);

                if enabled {
                    let value = state.constraints.$field.get_or_insert(0);
                    ui.add(egui::Slider::new(value, $range));
                } else {
                    state.constraints.$field = None;
                }
            });
        };
    }

    constraint_slider!("Minimum Infantry", minimum_infantry, 0..=100);
    constraint_slider!("Minimum Cavalry", minimum_cavalry, 0..=100);
    constraint_slider!("Minimum Artillery", minimum_artillery, 0..=100);
    constraint_slider!("Maximum Infantry", maximum_infantry, 0..=100);
    constraint_slider!("Maximum Cavalry", maximum_cavalry, 0..=100);
    constraint_slider!("Maximum Artillery", maximum_artillery, 0..=100);
    constraint_slider!("Maximum Rifles", maximum_rifles, 0..=10);
    constraint_slider!("Maximum Rockets", maximum_rockets, 0..=10);
    constraint_slider!("Minimum HP", minimum_hp, 0..=50000);
    constraint_slider!("Minimum Melee Damage", minimum_melee_damage, 0..=10000);
    constraint_slider!("Minimum Melee Defense", minimum_melee_defense, 0..=10000);
    constraint_slider!("Minimum Gold Target", minimum_gold_target, 0..=30000);
    constraint_slider!("Minimum Manpower Target", minimum_manpower_target, 0..=30000);

    ui.horizontal(|ui| {
        let mut max_target_enabled = state.constraints.maximization_targets.is_some();
        ui.checkbox(&mut max_target_enabled, "Maximization Target");

        if max_target_enabled {
            let max_target = state.constraints.maximization_targets.get_or_insert(MaximizationTargets::default());
            egui::ComboBox::from_label("Target")
                .selected_text(format!("{:?}", max_target))
                .show_ui(ui, |ui| {
                    ui.selectable_value(max_target, MaximizationTargets::HP, "HP");
                    ui.selectable_value(max_target, MaximizationTargets::MeleeAttack, "Melee Attack");
                    ui.selectable_value(max_target, MaximizationTargets::MeleeDefense, "Melee Defense");
                    ui.selectable_value(max_target, MaximizationTargets::Organization, "Organization");
                });
        } else {
            state.constraints.maximization_targets = None;
        }
    });
}

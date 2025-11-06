mod userinputs;

use egui;
use crate::gui::userinputs::user_edits;
use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct UIState
{
    mode: GameModes,
    constraints: minmaxer::Constraint,
    budget: minmaxer::Budget,
    comps: Vec<ArmyComposition>,
    reference_army: ArmyComposition
}
unsafe impl Send for UIState{}
unsafe impl Sync for UIState{}
impl UIState
{
    pub fn new() -> Self
    {
        Self{
            mode: GameModes::Combat,
            constraints: minmaxer::Constraint::default(),
            budget: minmaxer::Budget{gold: 2600, manpower: 6000},
            comps: Vec::new(),
            reference_army: ArmyComposition::default_for_mode(GameModes::Combat)
        }
    }
}
impl Default for UIState
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Default)]
pub struct GuiApp
{
    state: UIState
}
impl GuiApp
{
    pub fn new() -> Self
    {
        Self::default()
    }

}
impl eframe::App for GuiApp
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::CentralPanel::default().show(ctx, |ui| user_edits(ui, &mut self.state));
    }
}
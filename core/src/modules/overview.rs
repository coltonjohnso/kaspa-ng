use crate::imports::*;

pub struct Overview {
    #[allow(dead_code)]
    interop: Interop,
}

impl Overview {
    pub fn new(interop: Interop) -> Self {
        Self { interop }
    }
}

impl ModuleT for Overview {
    fn render(
        &mut self,
        _core: &mut Core,
        _ctx: &egui::Context,
        _frame: &mut eframe::Frame,
        _ui: &mut egui::Ui,
    ) {

    }
}

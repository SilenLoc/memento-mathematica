use crate::app::modules::{AckermannState, AdditionState, SubtractionState};
use crate::app::{Math, Module, Pane};

#[derive(Clone)]
pub struct OpenableOption {
    pub id: &'static str,
    pub version: &'static str,
    pub description: &'static str,
}

pub const OPTIONS: [OpenableOption; 3] = [
    OpenableOption {
        id: "Ackermann",
        version: "0.2",
        description: "The not not simple recursive function",
    },
    OpenableOption {
        id: "Addition",
        version: "0.1",
        description: "some english words",
    },
    OpenableOption {
        id: "Subtraction",
        version: "0.1",
        description: "hard english words to type",
    },
];

fn module_from_option(openable: OpenableOption) -> Pane {
    match openable.id {
        "Ackermann" => Pane {
            title: openable.id.to_string(),
            module: Module::Ackermann(AckermannState::default()),
        },
        "Addition" => Pane {
            title: openable.id.to_string(),
            module: Module::Addition(AdditionState::default()),
        },
        "Subtraction" => Pane {
            title: openable.id.to_string(),
            module: Module::Subtraction(SubtractionState::default()),
        },
        _ => Pane {
            title: openable.id.to_string(),
            module: Module::Addition(AdditionState::default()),
        },
    }
}

pub fn menu(ui: &mut egui::Ui, app: &mut Math) {
    egui::Window::new("Tabs")
        .vscroll(true)
        .default_height(300.00)
        .default_width(300.00)
        .resizable(false)
        .collapsible(false)
        .show(ui.ctx(), |ui| {
            for openable in OPTIONS {
                if ui.button(openable.id).clicked() {
                    app.add_pane(module_from_option(openable))
                }
            }
            ui.add_space(10.0);
            if ui.button("close").clicked() {
                app.close_menu()
            }
        });
}

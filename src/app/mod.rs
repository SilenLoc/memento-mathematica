mod functions;
mod menu;
mod modules;

use crate::app::menu::menu;
use crate::app::modules::{AckermannState, AdditionState, SubtractionState};
use egui::WidgetText;
use egui_tiles::Tile;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Math {
    #[serde(skip)]
    tree: egui_tiles::Tree<Pane>,

    #[serde(skip)]
    behavior: TreeBehavior,

    #[serde(skip)]
    command_center_open: bool,
}

impl Default for Math {
    fn default() -> Self {
        Self {
            behavior: TreeBehavior {},
            tree: create_tree(),
            command_center_open: false,
        }
    }
}

impl Math {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
    fn close_menu(&mut self) {
        self.command_center_open = false
    }

    fn add_pane(&mut self, pane: Pane) {
        if let Some(root) = self.tree.root {
            let new_tile_id = self.tree.tiles.insert_new(Tile::Pane(pane));
            if let Tile::Container(cont) = self.tree.tiles.get_mut(root).unwrap() {
                cont.add_child(new_tile_id)
            }
        }
    }
}

impl eframe::App for Math {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            if ui.button("Tabs").clicked() {
                self.command_center_open = !self.command_center_open
            };

            if self.command_center_open {
                menu(ui, self)
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut behavior = TreeBehavior {};
            self.tree.ui(&mut behavior, ui);
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

struct Pane {
    title: String,
    module: Module,
}

#[derive(Clone)]
enum Module {
    Addition(AdditionState),
    Subtraction(SubtractionState),
    Ackermann(AckermannState),
}

struct TreeBehavior {}

impl egui_tiles::Behavior<Pane> for TreeBehavior {
    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        let module = &mut pane.module;

        match module {
            Module::Addition(state) => state.render(ui),
            Module::Subtraction(state) => state.render(ui),
            Module::Ackermann(state) => state.render(ui),
        };

        if ui
            .add(egui::Button::new("Drag Handle").sense(egui::Sense::drag()))
            .drag_started()
        {
            egui_tiles::UiResponse::DragStarted
        } else {
            egui_tiles::UiResponse::None
        }
    }

    fn tab_title_for_pane(&mut self, pane: &Pane) -> WidgetText {
        WidgetText::from(pane.title.clone())
    }
}

fn create_tree() -> egui_tiles::Tree<Pane> {
    let mut tiles = egui_tiles::Tiles::default();

    let mut tabs = vec![];

    let add = Pane {
        title: "Addition".into(),
        module: Module::Addition(AdditionState::default()),
    };

    tabs.push({
        let children = tiles.insert_pane(add);
        tiles.insert_horizontal_tile(vec![children])
    });

    let minus = Pane {
        title: "Subtraction".into(),
        module: Module::Subtraction(SubtractionState::default()),
    };

    tabs.push({
        let children = tiles.insert_pane(minus);
        tiles.insert_horizontal_tile(vec![children])
    });

    let ack = Pane {
        title: "Ackermann".into(),
        module: Module::Ackermann(AckermannState::default()),
    };

    tabs.push({
        let children = tiles.insert_pane(ack);
        tiles.insert_horizontal_tile(vec![children])
    });

    let root = tiles.insert_tab_tile(tabs);

    egui_tiles::Tree::new(root, tiles)
}

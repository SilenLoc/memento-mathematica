use egui::Ui;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Math {
    #[serde(skip)]
    tree: egui_tiles::Tree<Pane>,

    #[serde(skip)]
    behavior: TreeBehavior,
}

impl Default for Math {
    fn default() -> Self {
        Self {
            behavior: TreeBehavior {},
            tree: create_tree(),
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
}

impl eframe::App for Math {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);
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
    nr: usize,
    module: Module,
}

enum Module {
    Addition(AdditionState),
    Subtraction(SubtractionState),
}

#[derive(Default)]
struct SubtractionState {
    result: i32,
    left: i32,
    right: i32,
}

impl SubtractionState {
    pub fn render(&mut self, ui: &mut Ui) {
        ui.add(egui::Slider::new(&mut self.left, 0..=100));
        ui.add(egui::Slider::new(&mut self.right, 0..=100));

        if ui.button("subtract").clicked() {
            self.result = self.left - self.right
        }

        ui.label(format!("Result {}", &self.result));
    }
}

#[derive(Default)]
struct AdditionState {
    result: i32,
    left: i32,
    right: i32,
}

impl AdditionState {
    pub fn render(&mut self, ui: &mut Ui) {
        ui.add(egui::Slider::new(&mut self.left, 0..=100));
        ui.add(egui::Slider::new(&mut self.right, 0..=100));

        if ui.button("add").clicked() {
            self.result = self.left + self.right
        }

        ui.label(format!("Result {}", &self.result));
    }
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

    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        format!("Pane {}", pane.nr).into()
    }
}

fn create_tree() -> egui_tiles::Tree<Pane> {
    let mut tiles = egui_tiles::Tiles::default();

    let mut tabs = vec![];

    let add = Pane {
        nr: 0,
        module: Module::Addition(AdditionState::default()),
    };

    tabs.push({
        let children = tiles.insert_pane(add);
        tiles.insert_horizontal_tile(vec![children])
    });

    let minus = Pane {
        nr: 1,
        module: Module::Subtraction(SubtractionState::default()),
    };

    tabs.push({
        let children = tiles.insert_pane(minus);
        tiles.insert_horizontal_tile(vec![children])
    });

    let root = tiles.insert_tab_tile(tabs);

    egui_tiles::Tree::new(root, tiles)
}

use egui::{
    menu::{BarState, MenuRoot},
    Color32, Id, InnerResponse, Pos2, Rect, Response, Sense, Ui,
};

const RED_RECT: Rect = Rect {
    min: Pos2::new(10., 10.),
    max: Pos2::new(100., 100.),
};

const BLUE_RECT: Rect = Rect {
    min: Pos2::new(100., 10.),
    max: Pos2::new(200., 100.),
};

const BIG_RECT: Rect = Rect {
    min: Pos2::new(10., 10.),
    max: Pos2::new(200., 100.),
};

#[derive(Default, Debug, serde::Deserialize, serde::Serialize, Clone)]
enum Color {
    #[default]
    Red,
    Blue,
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "menu",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
    )
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    state: Color,
    label: Option<String>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            state: Color::default(),
            label: None,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size_before_wrap();
            let (_, painter) = ui.allocate_painter(size, Sense::click());

            let r = ui.allocate_rect(BIG_RECT, Sense::click());

            painter.rect_filled(RED_RECT, 0., Color32::RED);

            painter.rect_filled(BLUE_RECT, 0., Color32::BLUE);

            context_menu_custom_without_pub(&r, self, ui);

            if let Some(label) = &self.label {
                ui.label(label.clone());
            }

            // context_menu_custom(
            //     &r,
            //     &mut self.state,
            //     set_state,
            //     set_contents,
            // );
        });
    }
}

fn context_menu_custom_without_pub(response: &Response, app: &mut TemplateApp, ui: &mut Ui) {
    if response.secondary_clicked() {
        let ctx = ui.ctx();
        if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
            app.state = set_state(pos);
        }
    }
    let f: Box<dyn FnOnce(&mut Ui)> = match app.state {
        Color::Red => Box::new(|ui: &mut Ui| {
            if ui.button("add red label").clicked() {
                app.label = Some("red label".to_owned());
            };
        }),

        Color::Blue => Box::new(|ui: &mut Ui| {
            if ui.button("add blue label").clicked() {
                app.label = Some("blue label".to_owned());
            }
        }),
    };
    response.context_menu(f);
}

fn set_state(p: Pos2) -> Color {
    if RED_RECT.contains(p) {
        Color::Red
    } else if BLUE_RECT.contains(p) {
        Color::Blue
    } else {
        Color::default()
    }
}

fn set_contents(state: &Color) -> Box<dyn FnOnce(&mut Ui)> {
    match state {
        Color::Red => Box::new(|ui| {
            if ui.button("red").clicked() {
                dbg!("red");
            }
        }),
        Color::Blue => Box::new(|ui| {
            if ui.button("blue").clicked() {
                dbg!("blue");
            }
        }),
    }
}

pub fn context_menu_custom<T, R>(
    response: &Response,
    state: &mut T,
    get_state: impl FnOnce(Pos2) -> T,
    set_contents: impl FnOnce(&T) -> Box<dyn FnOnce(&mut Ui) -> R>,
) -> Option<InnerResponse<R>> {
    let menu_id = Id::new("__egui::context_menu");
    let mut bar_state = BarState::load(&response.ctx, menu_id);
    let root = &mut bar_state;

    let menu_response = MenuRoot::context_interaction(response, root);
    match &menu_response {
        egui::menu::MenuResponse::Create(p, _) => {
            *state = get_state(*p);
        }
        _ => {}
    };

    let add_contents = set_contents(&state);

    MenuRoot::handle_menu_response(root, menu_response);
    let inner_response = bar_state.show(response, add_contents);

    bar_state.store(&response.ctx, menu_id);
    inner_response
}

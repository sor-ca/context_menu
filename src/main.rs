use egui::{
    menu::{BarState, MenuRoot},
    Color32, Id, InnerResponse, Pos2, Response, Sense, Ui,
};

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
    // Example stuff:
    label: String,

    // This how you opt-out of serialization of a field
    menu_pos: Option<Pos2>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "white".to_owned(),
            menu_pos: None,
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
            let mut action = false;
            let size = ui.available_size_before_wrap();
            let (_, painter) = ui.allocate_painter(size, Sense::click());
            let big_rect = egui::Rect {
                min: Pos2::new(10., 10.),
                max: Pos2::new(200., 200.),
            };
            painter.rect_filled(big_rect.clone(), 0., Color32::LIGHT_GRAY);
            let r = ui.allocate_rect(big_rect.clone(), Sense::click());

            let small_rect1 = egui::Rect {
                min: Pos2::new(20., 40.),
                max: Pos2::new(40., 60.),
            };

            painter.rect_filled(small_rect1.clone(), 0., Color32::RED);
            //let r1 = ui.allocate_rect(small_rect1.clone(), Sense::click());

            let small_rect2 = egui::Rect {
                min: Pos2::new(50., 40.),
                max: Pos2::new(70., 60.),
            };

            painter.rect_filled(small_rect2.clone(), 0., Color32::BLUE);

            // if r.secondary_clicked() {
            //     println!("r clicked2");
            //     let ctx = ui.ctx();
            //     if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
            //         if small_rect1.contains(pos) {
            //             self.label = "red".to_owned();
            //         } else if small_rect2.contains(pos) {
            //             self.label = "blue".to_owned();
            //         } else {
            //             self.label = "gray".to_owned();
            //         }
            //     }
            // }

            context_menu_custom(&r, self, |ui| {
                if ui.button("change color").clicked() {
                    action = true;
                }
            });
            if action {
                if let Some(pos) = self.menu_pos {
                    if small_rect1.contains(pos) {
                        self.label = "red".to_owned();
                    } else if small_rect2.contains(pos) {
                        self.label = "blue".to_owned();
                    } else {
                        self.label = "gray".to_owned();
                    }
                }
            }
            ui.label(self.label.clone());
        });
    }
}

pub fn context_menu_custom(
    response: &Response,
    app: &mut TemplateApp,
    add_contents: impl FnOnce(&mut Ui),
) -> Option<InnerResponse<()>> {
    let menu_id = Id::new("__egui::context_menu");
    let mut bar_state = BarState::load(&response.ctx, menu_id);
    let root = &mut bar_state;

    let menu_response = MenuRoot::context_interaction(response, root);
    match &menu_response {
        egui::menu::MenuResponse::Create(p, _) => {
            app.menu_pos = Some(*p);
        }
        egui::menu::MenuResponse::Close => {
            app.menu_pos = None;
            app.label = "white".to_owned();
        }
        _ => {}
    };
    MenuRoot::handle_menu_response(root, menu_response);
    let inner_response = bar_state.show(response, add_contents);

    bar_state.store(&response.ctx, menu_id);
    inner_response
}

// / Идея этой штуки и почему не подходит оригинальное меню:
// / во время создания меню нам нужно за чтото зацепиться
// / т.е. иметь доступ к MenuResponse и по ивенту Create создавать стейт, который потом кормить в ui
// /
// /
/*pub fn draw_context_menu_on_instruments(
    resp: Response,
    menu_state: SchemaMenuState,
    app: &Storage,
) -> AppMutations {
    let response = &resp;
    let menu_id = Id::new("__egui::context_menu");
    let mut bar_state = egui::menu::BarState::load(&response.ctx, menu_id);

    let idd = {
        let response = response;
        let root = &mut bar_state;
        //let id = response.id;
        // id
        let menu_response = egui::menu::MenuRoot::context_interaction(response, root);

        let pp = match &menu_response {
            egui::menu::MenuResponse::Create(_, _) => {
                menu_state.clone().store(&response.ctx, menu_id);
                menu_state
            }
            _ => SchemaMenuState::load(&response.ctx, menu_id),
        };

        egui::menu::MenuRoot::handle_menu_response(root, menu_response);

        pp
    };
    // if idd
    let inner_response = bar_state.show(response, |ui| schema_context_menu(ui, app, idd));

    let res = match inner_response {
        Some(x) => {
            let mutations = x.inner;
            mutations
        }
        None => AppMutations::new(),
    };

    bar_state.store(&response.ctx, menu_id);

    res
}*/

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use eframe::egui::style::{WidgetVisuals, Widgets};
use eframe::egui::util::IdTypeMap;
use eframe::egui::{Button, Id, Response};
use std::sync::Arc;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));

            ui.add(Button::new("Hi").primary().big());

            let mut ui = ImprovedUi::new();
            let mut ui = &mut ui;

            ui.widget_styles::<Button>().primary();
        });
    })
}

struct ImprovedUi {
    styles: Arc<IdTypeMap>,
}

impl ImprovedUi {
    fn new() -> Self {
        Self {
            styles: Default::default(),
        }
    }

    fn widget_styles<T>(&mut self) -> &mut Widgets {
        Arc::make_mut(&mut self.styles).get_temp_mut_or_default(Id::NULL)
    }

    fn get_styles<T>(&self, response: &Response) -> WidgetVisuals {
        // This could be improved if idtypemap had a get_temp fn that returns a reference
        self.styles
            .get_temp::<Widgets>(Id::NULL)
            .unwrap_or_default()
            .style(response)
            .clone()
    }
}

pub trait WidgetBuilder
where
    Self: Sized,
{
    fn styles_mut(&mut self) -> &mut Widgets;

    /// We could add helper functions to customize widgets like requested here: <https://github.com/emilk/egui/pull/5203>
    fn background_color(mut self, color: egui::Color32) -> Self {
        self.styles_mut().inactive.bg_fill = color;
        self
    }
}

impl<T> WidgetBuilder for &mut T
where
    T: WidgetBuilder,
{
    fn styles_mut(&mut self) -> &mut Widgets {
        (*self).styles_mut()
    }
}

impl WidgetBuilder for Widgets {
    fn styles_mut(&mut self) -> &mut Widgets {
        self
    }
}

impl<'a> WidgetBuilder for Button<'a> {
    fn styles_mut(&mut self) -> &mut Widgets {
        todo!()
        // &mut self.visuals
    }
}

// Now we could have "classes" as Extension Traits

pub trait MyWidgetStyles: WidgetBuilder {
    fn primary(mut self) -> Self {
        self.styles_mut().inactive.bg_fill = egui::Color32::RED;
        self
    }
}

// Instead of implementing this for T we could also just implement it for e.g. Button<'a> to limit it to a specific widget
impl<T> MyWidgetStyles for T where T: WidgetBuilder {}

struct WidgetStyle<T> {
    style: Widgets,
    _marker: std::marker::PhantomData<T>,
}

/// Should be used like this
/// ```rust
/// make_styles! {
///     MyStyle {
///         .primary {
///             :hover, :active {
///                 background_color: Color32::RED,
///             }
///         }
///     }
/// }
/// ```
macro_rules! make_styles {
    (
        $name:ident {
            $(.$style:ident {
                $($(:$pseudo:ident $(,)?)* {
                    $($prop:ident: $value:expr,)*
                })*
            })*
        }
    ) => {
        pub trait $name: WidgetBuilder {
            $(
                fn $style(mut self) -> Self {
                    $(
                        {
                            let c = |pseudo: &mut WidgetVisuals| {
                                *pseudo = WidgetVisuals {
                                    $($prop: $value,)*
                                    ..*pseudo
                                };
                            };

                            $(
                                c(&mut self.styles_mut().$pseudo);
                            )*
                        }
                    )*
                    self
                }
            )*
        }
    };
}

make_styles! {
    MyStyleExt {
        .primary {
            :hovered {
                bg_fill: egui::Color32::RED,
                expansion: 1.1,
            }
            :inactive, :active {
                bg_fill: egui::Color32::GREEN,
            }
        }

        .secondary {
            :hovered {

            }
        }
    }
}

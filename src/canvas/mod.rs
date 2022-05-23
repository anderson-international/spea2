extern crate graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;

use graphics::{clear, rectangle};
use piston::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateEvent};
use piston_window::{color, PistonWindow, WindowSettings};

use crate::{Model, Objective, EA};

/// The drawing surface and piston window used to display the set of points.
pub struct Canvas<'a> {
    window: PistonWindow,
    ea: EA,
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
    model: &'a dyn Model,
}

impl<'a> Canvas<'a> {
    pub fn new(ea: EA, model: &'a dyn Model) -> Self {
        let window: PistonWindow = WindowSettings::new("spea2-knapsack", [1024, 768])
            .exit_on_esc(true)
            .build()
            .unwrap();
        let Objective {
            min: min_x,
            max: max_x,
            ..
        } = ea.objectives[0];
        let Objective {
            min: min_y,
            max: max_y,
            ..
        } = ea.objectives[1];
        Self {
            window,
            ea,
            min_x,
            max_x,
            min_y,
            max_y,
            model,
        }
    }

    pub fn show(&mut self) {
        while let Some(e) = self.window.next() {
            if let Some(args) = e.render_args() {
                self.render(&e, args);
            }
            if e.update_args().is_some() {
                // self.update();
            }
            if let Some(Button::Keyboard(Key::Space)) = e.press_args() {
                self.update();
            }
        }
    }

    fn render(&mut self, e: &piston::Event, args: RenderArgs) {
        let width = args.window_size[0];
        let height = args.window_size[1];

        self.window.draw_2d(e, |c, gl, _| {
            clear(color::BLACK, gl);

            self.ea.population.iter().for_each(|item| {
                let x = ((item.values[0] - self.min_x) / (self.max_x - self.min_x)) as f64 * width;
                let y = ((item.values[1] - self.min_y) / (self.max_y - self.min_y)) as f64 * height;
                rectangle(color::RED, [x, y, 5.0, 5.0], c.transform, gl);
            });
            self.ea.archive.iter().for_each(|item| {
                let x = ((item.values[0] - self.min_x) / (self.max_x - self.min_x)) as f64 * width;
                let y = ((item.values[1] - self.min_y) / (self.max_y - self.min_y)) as f64 * height;
                rectangle(color::GREEN, [x, y, 5.0, 5.0], c.transform, gl);
            });
        });
    }

    fn update(&mut self) {
        super::evolve(&mut self.ea, self.model)
    }
}

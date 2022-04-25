extern crate graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;

use graphics::{clear, rectangle};
use piston::{RenderArgs, RenderEvent, UpdateEvent};
use piston_window::{color, PistonWindow, WindowSettings};
use spea2::model::{Model, MutationOperator, Objective};

/// The drawing surface and piston window used to display the set of points.
pub struct Canvas<'a> {
    window: PistonWindow,
    model: Model,
    mutation: MutationOperator<'a>,
}

impl<'a> Canvas<'a> {
    /**
    Returns a new canvas

    # Arguments

    * `point_count` - the number of points in the set
    * `min_distance` - if a pair of points are closer than the `min_distance`, then one of the points will be removed and replaced with a fresh random point
    */
    pub fn new(model: Model, mutation: MutationOperator<'a>) -> Self {
        let window: PistonWindow = WindowSettings::new("spea2-knapsack", [0, 0])
            .exit_on_esc(true)
            .fullscreen(true)
            .build()
            .unwrap();

        Self {
            window,
            model,
            mutation,
        }
    }

    /**
    Begins the decluster loop by listening for render and update window events.
    */
    pub fn show(&mut self) {
        while let Some(e) = self.window.next() {
            if let Some(args) = e.render_args() {
                self.render(&e, args);
            }

            if e.update_args().is_some() {
                self.update();
            }
        }
    }

    fn render(&mut self, e: &piston::Event, args: RenderArgs) {
        let Objective {
            min: x_min,
            max: x_max,
            ..
        } = self.model.objectives[0];
        let Objective {
            min: y_min,
            max: y_max,
            ..
        } = self.model.objectives[1];

        let items = &self.model.archive;
        self.window.draw_2d(e, |c, gl, _| {
            clear(color::BLACK, gl);
            for item in items {
                let x = ((item.values[0] - x_min) / (x_max - x_min)) as f64 * args.window_size[0];
                let y = ((item.values[1] - y_min) / (y_max - y_min)) as f64 * args.window_size[1];
                rectangle(color::GREEN, [x, y, 5.0, 5.0], c.transform, gl);
            }
        });
    }

    fn update(&mut self) {
        spea2::evolve(&mut self.model, &mut self.mutation)
    }
}

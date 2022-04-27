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
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

impl<'a> Canvas<'a> {
    /**
    Returns a new canvas

    # Arguments

    * `point_count` - the number of points in the set
    * `min_distance` - if a pair of points are closer than the `min_distance`, then one of the points will be removed and replaced with a fresh random point
    */
    pub fn new(model: Model, mutation: MutationOperator<'a>) -> Self {
        let window: PistonWindow = WindowSettings::new("spea2-knapsack", [1024, 768])
            .exit_on_esc(true)
            .build()
            .unwrap();
        let Objective {
            min: min_x,
            max: max_x,
            ..
        } = model.objectives[0];
        let Objective {
            min: min_y,
            max: max_y,
            ..
        } = model.objectives[1];
        Self {
            window,
            model,
            mutation,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    /**
    Begins the decluster loop by listening for render and update window events.
    */
    pub fn show(&mut self) {
        println!("{:?}", self.model.objectives);
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
        self.window.draw_2d(e, |c, gl, _| {
            clear(color::BLACK, gl);
            self.model.archive.iter().for_each(|item| {
                let x = ((item.values[0] - self.min_x) / (self.max_x - self.min_y)) as f64
                    * args.window_size[0];
                let y = ((item.values[1] - self.min_y) / (self.max_y - self.min_y)) as f64
                    * args.window_size[1];
                rectangle(color::GREEN, [x, y, 5.0, 5.0], c.transform, gl);
            })
        });
    }

    fn update(&mut self) {
        spea2::evolve(&mut self.model, &mut self.mutation)
    }
}

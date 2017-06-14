extern crate nalgebra as na;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate alga;

use opengl_graphics::{GlGraphics, OpenGL};
use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::*;

use alga::linear::Transformation;

struct Edge<'a> {
    p1: &'a na::Point3<f64>,
    p2: &'a na::Point3<f64>
}

struct Model<'a> {
    edges: Vec<Edge<'a>>
}

pub struct App<'a> {
    gl: GlGraphics,
    model: Model<'a>
}

impl<'a> App<'a> {
    fn render(&mut self, args: &RenderArgs) {
        let ref model = self.model;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            graphics::clear([0.7, 0.6, 0.75, 1.0], gl);

            let proj = na::Orthographic3::new(1.0, 2.0, -3.0, -2.5, 10.0, 900.0);
            let rotation = na::Rotation3::from_euler_angles(1.0, 1.0, 1.0);
            for edge in &model.edges {
                let p1 = proj.project_point(&rotation.transform_point(edge.p1));
                let p2 = proj.project_point(&rotation.transform_point(edge.p2));
                graphics::line([0.0, 0.0, 0.0, 1.0],
                               1.0,
                               [p1[0],p1[1],p2[0],p2[1]],
                               c.transform, gl);
            }
        });
    }
}

fn main() {
    let nodes = vec![
        na::Point3::new(10.0, 50.0, 50.0),
        na::Point3::new(10.0, 50.0, 10.0),
        na::Point3::new(10.0, 10.0, 50.0),
        na::Point3::new(10.0, 10.0, 10.0),
        na::Point3::new(50.0, 50.0, 50.0),
        na::Point3::new(50.0, 50.0, 10.0),
        na::Point3::new(50.0, 10.0, 50.0),
        na::Point3::new(50.0, 10.0, 10.0)
    ];
    let edges = vec![
        Edge { p1: &nodes[0], p2: &nodes[1] },
        Edge { p1: &nodes[2], p2: &nodes[3] },
        Edge { p1: &nodes[4], p2: &nodes[5] },
        Edge { p1: &nodes[6], p2: &nodes[7] },
        Edge { p1: &nodes[0], p2: &nodes[2] },
        Edge { p1: &nodes[4], p2: &nodes[6] },
        Edge { p1: &nodes[1], p2: &nodes[3] },
        Edge { p1: &nodes[5], p2: &nodes[7] },
        Edge { p1: &nodes[0], p2: &nodes[4] },
        Edge { p1: &nodes[2], p2: &nodes[6] },
        Edge { p1: &nodes[1], p2: &nodes[5] },
        Edge { p1: &nodes[3], p2: &nodes[7] }
    ];
    let cube = Model { edges: edges };

    // Rendering stuff

    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: GlutinWindow = WindowSettings::new(
            "Viewer",
            [800, 600]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        model: cube
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
    }
}
extern crate piston;
extern crate piston_window;
extern crate graphics;
// extern crate image;

mod lsystem;

use std::collections::HashMap;

use graphics::math::Matrix2d;
use graphics::line::{Shape as LineShape};
use piston_window::*;

use lsystem::{LSystem, Sym};


fn lsystem_bounds(iter: &[Sym], angle_radians: f64) -> (f64, f64, f64, f64) {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut angle = 0.0f64;

    let mut stack = vec![];

    let mut x_max = 0.0f64;
    let mut y_max = 0.0f64;
    let mut x_min = 0.0f64;
    let mut y_min = 0.0f64;

    for sym in iter {
        match *sym {
            Sym::Fwd(_) => {
                x += 10.0 * angle.cos();
                y += 10.0 * angle.sin();

                x_max = x_max.max(x);
                y_max = y_max.max(y);
                x_min = x_min.min(x);
                y_min = y_min.min(y);
            },
            Sym::Var(_) => { },
            Sym::Plus => angle -= angle_radians,
            Sym::Minus => angle += angle_radians,
            Sym::Push => stack.push((x, y, angle)),
            Sym::Pop => {
                let (x_, y_, angle_) = stack.pop().expect("mismatched nesting");
                x = x_;
                y = y_;
                angle = angle_;
            },
        }
    }

    (x_min, y_min, x_max, y_max)
}


fn render_lsystem(draw_state: &DrawState,
                  transform: Matrix2d,
                  graphics: &mut G2d,
                  iter: &[Sym],
                  angle_radians: f64,
                  color: [f32; 4],
                  radius: f64) {
    // const RADIUS: f64 = 1.0;

    let mut x = 0.0;
    let mut y = 0.0;
    let mut angle = 0.0f64;

    let mut stack = vec![];

    let line = Line::new(color, radius)
        .shape(LineShape::Bevel);

    for sym in iter {
        match *sym {
            Sym::Fwd(_) => {
                let xp = x + 10.0 * angle.cos();
                let yp = y + 10.0 * angle.sin();

                line.draw([x, y, xp, yp], draw_state, transform, graphics);

                x = xp;
                y = yp;
            },
            Sym::Var(_) => { },
            Sym::Plus => angle -= angle_radians,
            Sym::Minus => angle += angle_radians,
            Sym::Push => stack.push((x, y, angle)),
            Sym::Pop => {
                let (x_, y_, angle_) = stack.pop().expect("mismatched nesting");
                x = x_;
                y = y_;
                angle = angle_;
            },
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello", (640, 480))
        .exit_on_esc(true)
        .samples(16)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    // // Sierpinski Arrowhead Curve
    // let mut sys = {
    //     let a = Sym::Fwd(0);
    //     let b = Sym::Fwd(1);
    //     let plus = Sym::Plus;
    //     let minus = Sym::Minus;

    //     let mut map = HashMap::new();
    //     map.insert(a, vec![plus, b, minus, a, minus, b, plus]);
    //     map.insert(b, vec![minus, a, plus, b, plus, a, minus]);

    //     LSystem::new(
    //         vec![a],
    //         map
    //     )
    // };

    // let angle = std::f64::consts::PI / 3.0;

    // Fractal Plant
    let mut sys = {
        let x = Sym::Fwd(0);
        let f = Sym::Fwd(1);
        let plus = Sym::Plus;
        let minus = Sym::Minus;
        let push = Sym::Push;
        let pop = Sym::Pop;

        let mut map = HashMap::new();
        map.insert(x, vec![f, minus, push, push, x, pop, plus, x, pop, plus, f, push, plus, f, x, pop, minus, x]);
        map.insert(f, vec![f, f]);

        LSystem::new(
            vec![plus, plus, plus, x],
            map
        )
    };

    let angle = (25.0f64).to_radians();

    // // Pythagoras Tree
    // let mut sys = {
    //     let a = Sym::Fwd(0);
    //     let b = Sym::Fwd(1);
    //     let plus = Sym::Plus;
    //     let minus = Sym::Minus;
    //     let push = Sym::Push;
    //     let pop = Sym::Pop;

    //     let mut map = HashMap::new();
    //     map.insert(b, vec![b, b]);
    //     map.insert(a, vec![b, push, plus, a, pop, minus, a]);

    //     LSystem::new(
    //         vec![plus, plus, a],
    //         map
    //     )
    // };

    // let angle = (45.0f64).to_radians();

    // // Dragon Curve
    // let mut sys = {
    //     let x = Sym::Fwd(0);
    //     let y = Sym::Fwd(1);
    //     let f = Sym::Var(0);

    //     let plus = Sym::Plus;
    //     let minus = Sym::Minus;

    //     let mut map = HashMap::new();
    //     map.insert(x, vec![x, plus, y, f, plus]);
    //     map.insert(y, vec![minus, f, x, minus, y]);

    //     LSystem::new(
    //         vec![f, x],
    //         map
    //     )
    // };

    // let angle = (90.0f64).to_radians();

    const FGCOLOR: [f32; 4] = [0.0, 0.5, 1.0, 1.0];
    const BGCOLOR: [f32; 4] = [0.0, 0.1, 0.2, 1.0];

    let mut iter_num = 0usize;

    while let Some(e) = window.next() {
        if let Event::Input(Input::Text(ref s)) = e {
            match &s[..] {
                "\u{f703}" => { // right arrow
                    iter_num = iter_num.saturating_add(1);
                },
                "\u{f702}" => { // left arrow
                    iter_num = iter_num.saturating_sub(1);
                },
                _ => (),
            }
            println!("got text {:?}", s);
        }

        let size = window.size();
        let iter = sys.get(iter_num);

        window.draw_2d(&e, |c, g| {
            let (x_min, y_min, x_max, y_max) = lsystem_bounds(iter, angle);
            let x_sc = size.width as f64 / (x_max - x_min);
            let y_sc = size.height as f64 / (y_max - y_min);
            let sc = x_sc.min(y_sc);

            println!("initial transform {:?}", c.transform);

            let transform = c.transform
                .zoom(sc)
                .trans(-x_min, -y_min);

            clear(BGCOLOR, g);
            render_lsystem(&c.draw_state, transform, g, iter, angle, FGCOLOR, 1.0 / sc);
        });
    }
}

#[macro_use] extern crate glium;
extern crate rustc_serialize;

mod lsystem;
mod lsystem_reader;

use std::env::args;
use std::io::Read;
use lsystem::Sym;

const VERTEX_SHADER: &'static str = include_str!("shader.vert");
const FRAGMENT_SHADER: &'static str = include_str!("shader.frag");

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f64; 2],
    dist: f64,
}

implement_vertex!(Vertex, position, dist);

#[derive(Debug)]
struct LSystemRender {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    t_max: f64,
    verts: Vec<Vertex>,
}

fn render_lsystem(iter: &[Sym], angle_radians: f64) -> LSystemRender {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut t = 0.0;
    let mut angle = 0.0f64;

    let mut stack = vec![];

    let mut x_max = 0.0f64;
    let mut y_max = 0.0f64;
    let mut x_min = 0.0f64;
    let mut y_min = 0.0f64;

    let mut verts = vec![];

    for sym in iter {
        match *sym {
            Sym::Fwd(_) => {
                verts.push(Vertex { position: [x, y], dist: t });

                x += -0.005 * angle.cos();
                y += -0.005 * angle.sin();
                t += 0.005;

                verts.push(Vertex { position: [x, y], dist: t });

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
                // t = t_;
            },
        }
    }

    LSystemRender {
        x_min: x_min,
        x_max: x_max,
        y_min: y_min,
        y_max: y_max,
        t_max: t,
        verts: verts,
    }
}

fn main() {
    use glium::DisplayBuild;

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Hello, world!".to_owned())
        // .with_multisampling(16)
        .build_glium()
        .unwrap();

    let filename = args().nth(1).expect(
        "Usage: fractals <json file>"
    );

    let mut src = String::new();
    std::fs::File::open(filename).unwrap().read_to_string(&mut src).unwrap();
    let mut sys = lsystem_reader::parse_lsystem(&src).unwrap();
    let angle = sys.angle_radians();

    let mut render = render_lsystem(&sys.get(0), angle);

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

    let mut vertex_buffer = glium::VertexBuffer::new(&display, &render.verts).unwrap();
    let program = glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

    let mut scale = 1.0f32;
    let mut offset: [f32; 2] = [0.0, 0.0];
    let offset_step = 0.1f32;
    let mut iter_num = 0usize;

    let params = glium::draw_parameters::DrawParameters {
        // blend: glium::Blend::alpha_blending(),
        smooth: Some(glium::Smooth::Nicest),
        .. Default::default()
    };

    for event in display.wait_events() {
        use glium::Surface;
        use glium::glutin::Event;


        if let Event::ReceivedCharacter(ch) = event {
            match ch {
                '+' | '=' => scale *= 1.1,
                '-' => scale /= 1.1,
                's' => offset[1] += offset_step / scale,
                'w' => offset[1] -= offset_step / scale,
                'd' => offset[0] -= offset_step / scale,
                'a' => offset[0] += offset_step / scale,
                'q' | 'e' => {
                    iter_num = match ch {
                        'e' => iter_num.saturating_add(1),
                        'q' => iter_num.saturating_sub(1),
                        _ => unreachable!(),
                    };
                    render = render_lsystem(&sys.get(iter_num), angle);
                    vertex_buffer = glium::VertexBuffer::new(&display, &render.verts).unwrap();
                },
                'c' => offset = [0.0, 0.0],
                '\x1b' => return,
                _ => {
                    println!("key: {:?}", ch);
                },
            }
        }

        let mut target = display.draw();

        let size = (render.x_max - render.x_min).max(render.y_max - render.y_min) as f32;

        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program,
                    &uniform! { scale: scale, size: size, offset: offset, t_max: render.t_max as f32 },
                    &params).unwrap();
        target.finish().unwrap();

        match event {
            Event::Closed => {
                return;
            },
            _ => (),
        }
    }
}

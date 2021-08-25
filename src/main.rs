#[macro_use]
extern crate glium;
extern crate image;

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    use glium::{glutin, Surface};

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let one_second = std::time::Duration::from_secs(1u64);

    let mut i = 0;
    let mut start_time = std::time::Instant::now();

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.5],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
    #version 140

    in vec2 position;
    out vec2 my_attr;      // our new attribute

    uniform mat4 matrix;

    void main() {
        my_attr = position;     // we need to set the value of each `out` variable.
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
    "#;

    let fragment_shader_src = r#"
    #version 140

    in vec2 my_attr;
    out vec4 color;

    void main() {
        color = vec4(my_attr, 0.0, 1.0);   // we build a vec4 from a vec2 and two floats
    }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t: f32 = -0.5;

    event_loop.run(move |ev, _, control_flow| {
        if start_time.elapsed() >= one_second {
            start_time = std::time::Instant::now();
            i = 0;
        }

        let mut target = display.draw();

        target.clear_color(0.018, 0.018, 0.018, 1.0);
        let uniforms = uniform! {
            matrix: [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0f32],
            ]
        };

        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        update(i);
        i += 1;

        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}

fn update(frame_num: i64) {
    println!("FPS : {}", frame_num);
    clear();
}

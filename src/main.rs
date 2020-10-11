extern crate nalgebra_glm as glm;
use gl::types::*;
use glfw::{Action, Context, Key, WindowHint};
use std::f32::consts::*;

mod object;
mod shader;
use object::Object;

#[inline(always)]
fn get_error(_: glfw::Error, s: String, _: &()) {
    // glfw wayland will always return an error so we just print it instead of panic
    eprintln!("GLFW Error: {}", s)
}

static GLFW_ERROR: Option<glfw::ErrorCallback<()>> = Some(glfw::Callback {
    f: get_error as fn(glfw::Error, String, &()),
    data: (),
});

#[repr(C)]
struct Vertex {
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
    tex_x: GLfloat,
    tex_y: GLfloat,
}

// rustfmt ruins the formating of the vertices on sperate lines
#[rustfmt::skip]
static VERTICES: [GLfloat; 180] = [
    -50f32, -50f32, -50f32,  0f32, 0f32, // Bottom-left
     50f32, -50f32, -50f32,  1f32, 0f32, // bottom-right
     50f32,  50f32, -50f32,  1f32, 1f32, // top-right
     50f32,  50f32, -50f32,  1f32, 1f32, // top-right
    -50f32,  50f32, -50f32,  0f32, 1f32, // top-left
    -50f32, -50f32, -50f32,  0f32, 0f32, // bottom-left
    // Front face
    -50f32, -50f32,  50f32,  0f32, 0f32, // bottom-left
     50f32,  50f32,  50f32,  1f32, 1f32, // top-right
     50f32, -50f32,  50f32,  1f32, 0f32, // bottom-right
     50f32,  50f32,  50f32,  1f32, 1f32, // top-right
    -50f32, -50f32,  50f32,  0f32, 0f32, // bottom-left
    -50f32,  50f32,  50f32,  0f32, 1f32, // top-left
    // Left face
    -50f32,  50f32,  50f32,  1f32, 0f32, // top-right
    -50f32, -50f32, -50f32,  0f32, 1f32, // bottom-left
    -50f32,  50f32, -50f32,  1f32, 1f32, // top-left
    -50f32, -50f32, -50f32,  0f32, 1f32, // bottom-left
    -50f32,  50f32,  50f32,  1f32, 0f32, // top-right
    -50f32, -50f32,  50f32,  0f32, 0f32, // bottom-right
    // Right face
     50f32,  50f32,  50f32,  1f32, 0f32, // top-left
     50f32,  50f32, -50f32,  1f32, 1f32, // top-right
     50f32, -50f32, -50f32,  0f32, 1f32, // bottom-right
     50f32, -50f32, -50f32,  0f32, 1f32, // bottom-right
     50f32, -50f32,  50f32,  0f32, 0f32, // bottom-left
     50f32,  50f32,  50f32,  1f32, 0f32, // top-left
    // Bottom face
    -50f32, -50f32, -50f32,  0f32, 1f32, // top-right
     50f32, -50f32,  50f32,  1f32, 0f32, // bottom-left
     50f32, -50f32, -50f32,  1f32, 1f32, // top-left
     50f32, -50f32,  50f32,  1f32, 0f32, // bottom-left
    -50f32, -50f32, -50f32,  0f32, 1f32, // top-right
    -50f32, -50f32,  50f32,  0f32, 0f32, // bottom-right
    // Top face
    -50f32,  50f32, -50f32,  0f32, 1f32, // top-left
     50f32,  50f32, -50f32,  1f32, 1f32, // top-right
     50f32,  50f32,  50f32,  1f32, 0f32, // bottom-right
     50f32,  50f32,  50f32,  1f32, 0f32, // bottom-right
    -50f32,  50f32,  50f32,  0f32, 0f32, // bottom-left
    -50f32,  50f32, -50f32,  0f32, 1f32  // top-left
];

// static INDICES: [GLuint; 10] = [0, 1, 2, 2, 0];

// static VERTICES: [GLfloat; 16] = [
//     // positions          // colors           // texture coords
//     100.0, 100.0, 0.0, 0.0, // top right
//     100.0, 0.0, 0.0, 1.0, // bottom right
//     0.0, 0.0, 1.0, 1.0, // bottom left
//     0.0, 100.0, 1.0, 0.0, // top left
// ];
// static INDICES: [GLuint; 180] = (0u32..=180).collect::<Vec<GLuint>>();
// static INDICES: Vec<GLuint> = (0u32..=180).collect::<Vec<GLuint>>();
//

fn main() {
    let mut glfw = glfw::init(GLFW_ERROR).unwrap();

    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::Samples(Some(2)));
    // glfw.window_hint(WindowHint::Samples(None));

    let (mut window, events) = glfw
        .create_window(300, 300, "F", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s));

    let (width, height) = window.get_size();
    unsafe {
        gl::Viewport(0, 0, width, height);

        gl::Enable(gl::MULTISAMPLE);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::FRONT);
        // gl::FrontFace(gl::CCW);
        let vert_pos = [
            glm::vec3(0f32, 0f32, 0f32),
            glm::vec3(0f32, 0f32, -100f32),
            glm::vec3(100f32, 0f32, 0f32),
            glm::vec3(100f32, 0f32, -100f32),
            glm::vec3(0f32, 100f32, 0f32),
        ];
        let tris: Vec<Object> = vert_pos
            .iter()
            .map(|vert| {
                let tri = Object::new(
                    &VERTICES,
                    &(0..36).collect::<Vec<GLuint>>(),
                    "src/shader.vert.glsl",
                    "src/shader.frag.glsl",
                    "src/test.png",
                )
                .unwrap();

                if vert.z == vert.x && vert.z == vert.y && vert.z == 0f32 {
                    tri.shader.set_vec3("color", &glm::vec3(1f32, 1f32, 1f32));
                } else if vert.x == 100f32 {
                    tri.shader.set_vec3("color", &glm::vec3(0f32, 0f32, 1f32));
                } else {
                    tri.shader.set_vec3("color", &glm::vec3(0f32, 1f32, 0f32));
                }

                tri.vertex_attrib(0, 3, 5, 0);
                tri.vertex_attrib(1, 2, 5, 3);

                tri
            })
            .collect();

        let cam_x = FRAC_1_SQRT_2.atan();
        let cam_y = FRAC_PI_4;

        let model_xz = cam_y.sin();
        let model_y = cam_x.cos();

        let mut view = glm::Mat4::new_scaling(1f32);

        view = glm::translate(&view, &glm::vec3(0f32, 0f32, -1000f32));
        view = glm::rotate_x(&view, cam_x);
        view = glm::rotate_y(&view, cam_y);

        while !window.should_close() {
            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, event);
            }

            gl::ClearColor(0.4, 0.4, 0.4, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let (width, height) = window.get_size();
            // here we make a isometric projection, which is just an orthographic projection and
            // rotating the x axis by arctangent of 1/sqrt(2), and the y axis by 45 deg or pi/4
            let proj = glm::ortho(0f32, width as f32, 0f32, height as f32, 90f32, 2000f32);

            let model_center = glm::vec3(
                // we need to compensate for the isometric projection, because it distorts
                // dimentions to appear the same length
                // we need to move both x and z to "appear" to only move on the x axis
                // the x and z axis are diagonal, can be thought of as a triangle with
                // angles of 45, 45, and 90 degrees. Some trigonometry later tells us that
                // we need to multiply our desired movement by the sine of pi/4
                (width as f32 / 2f32) * model_xz,
                // the y axis is similar, but different. you need to divide the desired
                // movement the cosine of the y angle of the camera. I don't exactly know
                // how it works, but it does.
                (height as f32 / 2f32) / model_y,
                (width as f32 / 2f32) * model_xz,
            );

            for (vert, tri) in vert_pos.iter().zip(tris.iter()) {
                let mut model = glm::Mat4::new_scaling(1f32);
                model = glm::translate(&model, &model_center);
                model = glm::translate(&model, &glm::vec3(50f32, 0f32, -50f32));
                // model = glm::rotate_y(&model, glfw.get_time() as f32);
                model = glm::translate(&model, &glm::vec3(-50f32, -100f32, 50f32));
                model = glm::translate(&model, vert);

                let mvp = proj * view * model;
                tri.shader.set_mat4("mvp", &mvp);

                tri.draw();
            }

            glfw.poll_events();
            window.swap_buffers();
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    use glfw::WindowEvent;
    match event {
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        WindowEvent::Key(Key::Num1, _, Action::Press, _) => unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE)
        },
        WindowEvent::Key(Key::Num1, _, Action::Release, _) => unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL)
        },
        WindowEvent::FramebufferSize(width, height) => unsafe { gl::Viewport(0, 0, width, height) },
        _ => {}
    }
}

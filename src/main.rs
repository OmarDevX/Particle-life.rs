use egui::*;
use egui_backend::{
    egui::{self, ClippedPrimitive},
    epi::{Frame, IntegrationInfo},
    get_frame_time, gl, sdl2,
    sdl2::event::Event,
    sdl2::video::GLProfile,
    sdl2::video::SwapInterval,
    DpiScaling, ShaderVersion, Signal,
};

use std::{fs, sync::Arc, sync::Mutex,time::Instant};

use epi::backend::FrameData;
use glm::{vec2, vec3, Vec2, Vec3};
use sdl2::{event::WindowEvent, keyboard::Keycode, mouse::MouseWheelDirection};

// Alias the backend to something less mouthful
use egui_sdl2_gl::{
    self as egui_backend,
    painter::{compile_shader, link_program},
};
use gl::types::*;
use std::ffi::CString;
use std::ptr;

mod window_manager;
use window_manager::{
    window_manager::windows::{MainWindow, SandboxWindow},
    *,
};

mod objects;
use objects::*;

mod camera;
use camera::*;
use std::time::*;


use async_std::*;

struct CircleData {
    position: Vec2,
    radius: f32,
    color: Vec3,
}
async fn interval_example(scene: Arc<Mutex<SandboxWindow>>) {
    loop {
        task::sleep(Duration::from_secs(1)).await;
        println!("Tick!");

        // Accessing objects from sandbox_window
        let mut sandbox_window = scene.lock().unwrap();
        for object in &mut sandbox_window.objects {
            // Perform operations with each object
            println!("Object position: {:?}", object.position);
            println!("Object radius: {:?}", object.radius);
            println!("Object color: {:?}", object.color);
            object.position = vec2(1.0, 0.0);
        }
        println!("a");
        // Release the lock automatically when sandbox_window goes out of scope
    }
}


fn main() {
    let mut my_camera = Camera::new(vec2(0.0, 0.0), 1.0, vec2(0.0, 0.0), 20.5);
    let mut SCREEN_WIDTH = 1280;
    let mut SCREEN_HEIGHT = 700;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_framebuffer_srgb_compatible(true);
    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_samples(4);
    gl_attr.set_context_version(4, 6);

    let window = video_subsystem
        .window("Vetracer Template", SCREEN_WIDTH, SCREEN_HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _ctx = window.gl_create_context().unwrap();
    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (4, 6));

    if let Err(error) = window.subsystem().gl_set_swap_interval(SwapInterval::VSync) {
        println!(
            "Failed to gl_set_swap_interval(SwapInterval::VSync): {}",
            error
        );
    }

    let (mut painter, mut egui_state) =
        egui_backend::with_sdl2(&window, ShaderVersion::Default, DpiScaling::Default);
    let egui_ctx = egui::Context::default();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let start_time: Instant = Instant::now();
    let repaint_signal = Arc::new(Signal::default());

    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    // Load GLSL shader source from files
    let compute_shader_source = fs::read_to_string("shaders/compute_shader.glsl")
        .expect("Failed to read compute_shader.glsl");
    let quad_vertex_shader_source = fs::read_to_string("shaders/quad_vertex_shader.glsl")
        .expect("Failed to read quad_vertex_shader.glsl");
    let quad_fragment_shader_source =
        fs::read_to_string("shaders/quad_fragment_shader.glsl")
            .expect("Failed to read quad_fragment_shader.glsl");

    // Compile shaders
    let compute_shader = compile_shader(&compute_shader_source, gl::COMPUTE_SHADER);
    let quad_vertex_shader = compile_shader(&quad_vertex_shader_source, gl::VERTEX_SHADER);
    let quad_fragment_shader = compile_shader(&quad_fragment_shader_source, gl::FRAGMENT_SHADER);

    // Link shader programs
    let compute_shader_program = link_program(compute_shader, 0);
    let quad_shader_program = link_program(quad_vertex_shader, quad_fragment_shader);

    // Create a texture for the compute shader to write to
    let mut texture = create_texture(SCREEN_WIDTH, SCREEN_HEIGHT);

    // Set up a fullscreen quad
    let vertices: [f32; 8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];

    let mut vao = 0;
    let mut vbo = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        let pos_attrib = gl::GetAttribLocation(
            quad_shader_program,
            CString::new("in_pos").unwrap().as_ptr(),
        );
        gl::EnableVertexAttribArray(pos_attrib as GLuint);
        gl::VertexAttribPointer(
            pos_attrib as GLuint,
            2,
            gl::FLOAT,
            gl::FALSE,
            2 * std::mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    let mut sandbox_window =SandboxWindow::new();
    let mut main_window = MainWindow::new(&mut sandbox_window);
let mut circle_data: Vec<CircleData> = Vec::new();

// Clone objects from sandbox_window

// Iterate over the cloned objects
for object in &main_window.sandbox_window.objects {
    circle_data.push(CircleData {
        position: object.position,
        radius: object.radius,
        color: object.color,
    });
}
    let last_frame_time: Instant = Instant::now();
    // let event_loop = task::spawn(interval_example(sandbox_window.clone()));
    'running: loop {
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time).as_secs_f32();
        egui_state.input.time = Some(start_time.elapsed().as_secs_f64());

        egui_ctx.begin_frame(egui_state.input.take());

        let frame_time = get_frame_time(start_time);
        let frame = Frame::new(FrameData {
            info: IntegrationInfo {
                web_info: None,
                cpu_usage: Some(frame_time),
                native_pixels_per_point: Some(egui_state.native_pixels_per_point),
                prefer_dark_mode: None,
                name: "egui + sdl2 + gl",
            },
            output: Default::default(),
            repaint_signal: repaint_signal.clone(),
        });
        main_window.desktop_ui(&egui_ctx);
        // circle.update(delta_time);
        main_window.sandbox_window.update_objects(
            delta_time,
            SCREEN_WIDTH as f32,
            SCREEN_HEIGHT as f32,
        );

        unsafe {
            gl::UseProgram(compute_shader_program);

            // Camera
            let offset_loc = gl::GetUniformLocation(
                compute_shader_program,
                CString::new("offset").unwrap().as_ptr(),
            );
            let zoom_loc = gl::GetUniformLocation(
                compute_shader_program,
                CString::new("zoom").unwrap().as_ptr(),
            );

            gl::Uniform2f(
                offset_loc as GLint,
                my_camera.offset[0],
                my_camera.offset[1],
            );
            gl::Uniform1f(zoom_loc as GLint, my_camera.zoom);
            // Set other circle uniforms as before
            let deltatime_loc = gl::GetUniformLocation(
                compute_shader_program,
                CString::new("dt").unwrap().as_ptr(),
            );
            
            // Set other circle uniforms as before
            let circle_position_loc = gl::GetUniformLocation(
                compute_shader_program,
                CString::new("circle_position").unwrap().as_ptr(),
            );
            let circle_radius_loc = gl::GetUniformLocation(
                compute_shader_program,
                CString::new("circle_radius").unwrap().as_ptr(),
            );
            let circle_color_loc = gl::GetUniformLocation(
                compute_shader_program,
                CString::new("circle_color").unwrap().as_ptr(),
            );
                gl::Uniform1f(
                    deltatime_loc as GLint,
                    delta_time,
                );
            for i in 0..main_window.sandbox_window.objects.len() {
                gl::Uniform2f(
                    circle_position_loc + i as GLint,
                    main_window.sandbox_window.objects[i].position[0],
                    main_window.sandbox_window.objects[i].position[1],
                );
                gl::Uniform1f(
                    circle_radius_loc + i as GLint,
                    main_window.sandbox_window.objects[i].radius,
                );
                gl::Uniform3f(
                    circle_color_loc + i as GLint,
                    main_window.sandbox_window.objects[i].color[0],
                    main_window.sandbox_window.objects[i].color[1],
                    main_window.sandbox_window.objects[i].color[2],
                );
            }

            gl::DispatchCompute(SCREEN_WIDTH / 8, SCREEN_HEIGHT / 8, 1);
            gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
        }

        my_camera.update(delta_time);
        let FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            viewport_output,
        } = egui_ctx.end_frame();
        egui_state.process_output(&window, &platform_output);

        if frame.take_app_output().quit {
            break 'running;
        }

        let repaint_after = viewport_output
            .get(&ViewportId::ROOT)
            .expect("Missing ViewportId::ROOT")
            .repaint_delay;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window {
                    win_event: WindowEvent::Resized(width, height),
                    ..
                } => {
                    SCREEN_HEIGHT = height as u32;
                    SCREEN_WIDTH = width as u32;
                    unsafe {
                        gl::Viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
                    }
                    texture = unsafe { create_texture(SCREEN_WIDTH, SCREEN_HEIGHT) };
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::W => my_camera.velocity.y += 2.0,
                    Keycode::A => my_camera.velocity.x -= 2.0,
                    Keycode::S => my_camera.velocity.y -= 2.0,
                    Keycode::D => my_camera.velocity.x += 2.0,
                    Keycode::Space => my_camera.set_velocity(vec2(0.0, 0.0)),
                    _ => {}
                },
                Event::MouseWheel { y, direction, .. } => match direction {
                    MouseWheelDirection::Flipped => {
                        my_camera.zoom /= 1.0 + 0.1 * (y as f32);
                    }
                    MouseWheelDirection::Normal => {
                        my_camera.zoom *= 1.0 + 0.1 * (y as f32);
                    }
                    MouseWheelDirection::Unknown(_) => {}
                },
                _ => {
                    egui_state.process_input(&window, event, &mut painter);
                }
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(quad_shader_program);
            gl::BindVertexArray(vao);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }

        let paint_jobs: Vec<ClippedPrimitive> = egui_ctx.tessellate(shapes, pixels_per_point);
        painter.paint_jobs(None, textures_delta, paint_jobs);

        window.gl_swap_window();
    }
}

fn create_texture(width: u32, height: u32) -> GLuint {
    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA32F as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::FLOAT,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::BindImageTexture(0, texture, 0, gl::FALSE, 0, gl::WRITE_ONLY, gl::RGBA32F);
    }
    texture
}

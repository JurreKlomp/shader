#![allow(unused)]
use std::collections::HashSet;

use glfw::{Action, Context, Glfw, WindowEvent};

pub struct Window {
    pub handle: glfw::PWindow,
    mouse_position: (f64, f64),
    keys: std::collections::HashSet<glfw::Key>,
    glfw: Glfw,
    receiver: glfw::GlfwReceiver<(f64, WindowEvent)>,
    size: (u32, u32),
}

impl Window {
    pub fn new<'a, T: Into<&'a str>>(width: u32, height: u32, title: T) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut handle, events) = glfw
            .with_primary_monitor(|glfw, monitor| {
                let (_, _, width, height) = if let Some(monitor) = monitor.as_ref() {
                    monitor.get_workarea()
                } else {
                    (0, 0, width as i32, height as i32)
                };
                glfw.create_window(
                    width as u32,
                    height as u32,
                    title.into(),
                    monitor.map_or(glfw::WindowMode::Windowed, |m| {
                        glfw::WindowMode::FullScreen(m)
                    }),
                )
            })
            .expect("Unable to create window");

        handle.make_current();
        handle.set_key_polling(true);
        handle.set_mouse_button_polling(true);
        handle.set_cursor_pos_polling(true);
        handle.set_framebuffer_size_polling(true);

        gl::load_with(|symbol| handle.get_proc_address(symbol) as *const _);

        Self {
            glfw,
            keys: HashSet::new(),
            mouse_position: (0.0, 0.0),
            handle,
            receiver: events,
            size: (width, height),
        }
    }

    pub fn is_pressed(&self, key: glfw::Key) -> bool {
        self.keys.contains(&key)
    }

    pub fn get_mouse_position(&self) -> (f64, f64) {
        self.mouse_position
    }

    pub fn get_size(&self) -> (u32, u32) {
        self.size
    }

    pub fn should_close(&self) -> bool {
        self.handle.should_close()
    }

    pub fn update(&mut self) {
        self.glfw.poll_events();
        self.process_events();
        self.handle.swap_buffers();
    }

    pub fn process_events(&mut self) {
        let messages = glfw::flush_messages(&self.receiver);
        for (_, event) in messages {
            match event {
                glfw::WindowEvent::CursorPos(x_position, y_position) => {
                    self.mouse_position = (x_position, y_position);
                }
                glfw::WindowEvent::Key(glfw::Key::Escape, _, Action::Press, _) => {
                    self.handle.set_should_close(true);
                }
                glfw::WindowEvent::Key(key, _, Action::Press, _) => {
                    self.keys.insert(key);
                }
                glfw::WindowEvent::Key(key, _, Action::Release, _) => {
                    self.keys.remove(&key);
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    self.size = (width as u32, height as u32);
                }
                _ => {}
            }
        }
    }
}

use std::collections::HashSet;

use glfw::{Action, Context, Glfw, WindowEvent};

pub struct Window {
    pub keys: std::collections::HashSet<glfw::Key>,
    pub handle: glfw::PWindow,
    glfw: Glfw,
    receiver: glfw::GlfwReceiver<(f64, WindowEvent)>,
    size: (u32, u32),
}

impl Window {
    pub fn new<'a, T: Into<&'a str>>(width: u32, height: u32, title: T) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut handle, events) = glfw
            .create_window(width, height, title.into(), glfw::WindowMode::Windowed)
            .expect("Unable to create window!");

        handle.make_current();
        handle.set_key_polling(true);
        handle.set_framebuffer_size_polling(true);

        gl::load_with(|symbol| handle.get_proc_address(symbol) as *const _);

        Self {
            keys: HashSet::new(),
            glfw,
            handle,
            receiver: events,
            size: (width, height),
        }
    }

    pub fn is_pressed(&self, key: glfw::Key) -> bool {
        self.keys.contains(&key)
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
                    println!("{}, {}", width, height);
                }
                _ => {}
            }
        }
    }
}

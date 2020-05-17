use crate::core::window::{Window, WindowData, WindowProperties};
use crate::events::application_event::*;
use crate::events::event::Event;
use crate::events::key_event::*;
use crate::events::mouse_event::*;
use crate::{blip_error, blip_warn};
use glfw::{Action, Context, Key, SwapInterval};
use std::cell::Cell;

pub struct WindowsWindow {
    title: &'static str,
    glfw_token: glfw::Glfw,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    window_data: WindowData,
    event_callback: Option<fn(&dyn Event) -> ()>,
}

impl WindowsWindow {
    pub fn new(props: WindowProperties) -> Self {
        let mut glfw = glfw::init(Some(glfw::Callback {
            f: Self::glfw_error_callback,
            data: Cell::new(0),
        }))
        .unwrap();

        let (mut glfw_window, events) = glfw
            .create_window(props.width, props.height, props.title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        glfw_window.make_current();
        glfw.set_swap_interval(SwapInterval::Sync(1));

        glfw_window.set_all_polling(true);

        WindowsWindow {
            glfw_token: glfw,
            window: glfw_window,
            events: events,
            title: props.title,
            window_data: WindowData {
                width: props.width,
                height: props.height,
                is_vsync_enabled: true,
            },
            event_callback: None,
        }
    }

    fn handle_window_event(&self, old_window_data: &WindowData, (time, event): (f64, glfw::WindowEvent)) -> WindowData {
        match event {
            //glfw::WindowEvent::Pos(x, y) => window.set_title(&format!("Time: {:?}, Window pos: ({:?}, {:?})", time, x, y)),
            glfw::WindowEvent::Size(w, h) => match self.event_callback {
                Some(f) => {
                    f(&WindowResizeEvent {
                        width: w as u32,
                        height: h as u32,
                        ..Default::default()
                    });
                    WindowData {
                        width: w as u32,
                        height: h as u32,
                        is_vsync_enabled: old_window_data.is_vsync_enabled,
                    }
                }
                None => {
                    blip_warn!("WindowResizeEvent triggered, but there is no callback to handle it!");
                    WindowData {
                        width: old_window_data.width,
                        height: old_window_data.height,
                        is_vsync_enabled: old_window_data.is_vsync_enabled,
                    }
                }
            },
            /*glfw::WindowEvent::Close => println!("Time: {:?}, Window close requested.", time),
            glfw::WindowEvent::Refresh => println!("Time: {:?}, Window refresh callback triggered.", time),
            glfw::WindowEvent::Focus(true) => println!("Time: {:?}, Window focus gained.", time),
            glfw::WindowEvent::Focus(false) => println!("Time: {:?}, Window focus lost.", time),
            glfw::WindowEvent::Iconify(true) => println!("Time: {:?}, Window was minimised", time),
            glfw::WindowEvent::Iconify(false) => println!("Time: {:?}, Window was maximised.", time),
            glfw::WindowEvent::FramebufferSize(w, h) => println!("Time: {:?}, Framebuffer size: ({:?}, {:?})", time, w, h),
            glfw::WindowEvent::Char(character) => println!("Time: {:?}, Character: {:?}", time, character),
            glfw::WindowEvent::CharModifiers(character, mods) => {
                println!("Time: {:?}, Character: {:?}, Modifiers: [{:?}]", time, character, mods)
            }
            glfw::WindowEvent::MouseButton(btn, action, mods) => println!(
                "Time: {:?}, Button: {:?}, Action: {:?}, Modifiers: [{:?}]",
                time,
                glfw::DebugAliases(btn),
                action,
                mods
            ),
            glfw::WindowEvent::CursorPos(xpos, ypos) => {
                window.set_title(&format!("Time: {:?}, Cursor position: ({:?}, {:?})", time, xpos, ypos))
            }
            glfw::WindowEvent::CursorEnter(true) => println!("Time: {:?}, Cursor entered window.", time),
            glfw::WindowEvent::CursorEnter(false) => println!("Time: {:?}, Cursor left window.", time),
            glfw::WindowEvent::Scroll(x, y) => window.set_title(&format!("Time: {:?}, Scroll offset: ({:?}, {:?})", time, x, y)),
            glfw::WindowEvent::Key(key, scancode, action, mods) => {
                println!(
                    "Time: {:?}, Key: {:?}, ScanCode: {:?}, Action: {:?}, Modifiers: [{:?}]",
                    time, key, scancode, action, mods
                );
                match (key, action) {
                    (Key::Escape, Action::Press) => window.set_should_close(true),
                    (Key::R, Action::Press) => {
                        // Resize should cause the window to "refresh"
                        let (window_width, window_height) = window.get_size();
                        window.set_size(window_width + 1, window_height);
                        window.set_size(window_width, window_height);
                    }
                    _ => {}
                }
            }
            glfw::WindowEvent::FileDrop(paths) => println!("Time: {:?}, Files dropped: {:?}", time, paths),
            glfw::WindowEvent::Maximize(maximized) => println!("Time: {:?}, Window maximized: {:?}.", time, maximized),
            glfw::WindowEvent::ContentScale(xscale, yscale) => {
                println!("Time: {:?}, Content scale x: {:?}, Content scale y: {:?}", time, xscale, yscale)
            }*/
            _ => WindowData {
                width: old_window_data.width,
                height: old_window_data.height,
                is_vsync_enabled: old_window_data.is_vsync_enabled,
            },
        }
    }

    fn glfw_error_callback(_: glfw::Error, description: String, error_count: &Cell<usize>) {
        blip_error!(format!("GLFW Error ({0}): {1}", error_count.get(), description));
    }
}

impl Window for WindowsWindow {
    fn on_update(&mut self) -> () {
        self.glfw_token.poll_events();
        for event in glfw::flush_messages(&self.events) {
            self.window_data = self.handle_window_event(&self.window_data, event);
        }
        self.window.swap_buffers();
    }

    fn get_width(&self) -> u32 {
        self.window_data.width
    }

    fn get_height(&self) -> u32 {
        self.window_data.height
    }

    fn set_event_callback(&mut self, callback: fn(&dyn Event) -> ()) {
        self.event_callback = Some(callback);
    }

    fn set_vsync(&mut self, enabled: bool) {
        if enabled {
            self.window_data.is_vsync_enabled = true;
            self.glfw_token.set_swap_interval(SwapInterval::Sync(1));
        } else {
            self.window_data.is_vsync_enabled = false;
            self.glfw_token.set_swap_interval(SwapInterval::None);
        }
    }

    fn is_vsync_enabled(&self) -> bool {
        self.window_data.is_vsync_enabled
    }

    fn create(window_props: WindowProperties) -> WindowsWindow {
        WindowsWindow::new(window_props)
    }
}

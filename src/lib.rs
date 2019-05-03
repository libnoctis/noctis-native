extern crate winit;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate cocoa;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

pub mod platform;

use winit::{Event, WindowEvent, EventsLoop, ControlFlow};

pub struct Window {
    window: platform::Window,
    events_loop: EventsLoop,

    decorated: bool,
    title: String
}

impl Window {
    pub fn new() -> Result<Window, winit::CreationError> {
        let events_loop = EventsLoop::new();
        let window = winit::Window::new(&events_loop)?;

        let mut result = Window {
            window: platform::Window::new(window),
            decorated: true,
            title: "".into(),
            events_loop
        };

        result.set_title("Noctis window");

        Ok(result)
    }

    pub fn run(&mut self) {
        self.events_loop.run_forever(|event| {
            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    println!("The close button was pressed; stopping");
                    ControlFlow::Break
                },
                _ => ControlFlow::Continue,
            }
        });
    }

    pub fn set_decorated(&mut self, decorated: bool) {
        self.window.window.set_decorations(decorated);
        self.decorated = decorated;
    }

    pub fn is_decorated(&self) -> bool{
        self.decorated
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.window.set_title(title);
        self.title = title.into()
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    // Titlebar

    pub fn enable_content_over_titlebar(&mut self, enabled: bool) {
        self.window.enable_content_over(enabled)
    }

    pub fn is_content_over_titlebar_enabled(&self) -> bool {
        self.window.is_content_over_enabled()
    }

    pub fn set_title_displayed(&mut self, displayed: bool) {
        self.window.set_title_displayed(displayed)
    }

    pub fn is_title_displayed(&self) -> bool {
        self.window.is_title_displayed()
    }

    pub fn set_titlebar_big(&mut self, big: bool) {
        self.set_title_displayed(false);
        self.window.set_titlebar_big(big)
    }

    pub fn is_titlebar_big(&self) -> bool {
        self.window.is_titlebar_big()
    }
}

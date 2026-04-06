#![allow(unused)]

use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{KeyEvent, WindowEvent},
    event_loop::{self, ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{self, Window},
};

struct State {
    window: Arc<Window>,
}

struct App {
    state: Option<State>,
}

impl App {
    fn new() -> Self {
        Self { state: None }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        todo!()
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: window::WindowId,
        event: WindowEvent,
    ) {
        todo!()
    }
}

fn run_window() {}

fn main() {
    println!("redoing surface");
    run_window();
}

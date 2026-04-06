#![allow(unused)]

use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{self, ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{self, Window},
};

struct State {
    window: Arc<Window>,
}
impl State {
    async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        Ok(Self { window })
    }
}

struct App {
    state: Option<State>,
}

impl App {
    fn new(event_loop: &EventLoop<State>) -> Self {
        Self { state: None }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attribures = Window::default_attributes();
        let window = event_loop.create_window(window_attribures).unwrap();
        let state = pollster::block_on(State::new(Arc::new(window))).unwrap();
        self.state = Some(state);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            // WindowEvent::Resized(size) => state.resize(size),
            WindowEvent::RedrawRequested => {}
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => match (code, key_state.is_pressed()) {
                (KeyCode::Escape, true) => event_loop.exit(),
                (KeyCode::KeyQ, true) => event_loop.exit(),
                _ => {}
            },
            _ => {}
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        self.state = Some(event);
    }
}

fn run_window() -> anyhow::Result<()> {
    env_logger::init();
    let event_loop: EventLoop<State> = EventLoop::with_user_event().build()?;
    let mut app = App::new(&event_loop);
    event_loop.run_app(&mut app);
    Ok(())
}

fn main() {
    println!("redoing surface");
    run_window();
}

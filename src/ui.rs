//! # UI module
//!
//! It's just a messy fork of Conrod's hello world at the moment.
//! As I'll work my way through it more comments and SG-specific code to come.
use conrod::backend::glium::glium::{self, Surface};
use conrod::{self, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};

#[derive(PartialEq, Clone)]
enum Stage {
    Welcome,
    REPL,
}

struct App {
    stage: Stage,
    exit_requested: bool,
    escape_key_state: glium::glutin::ElementState,
}

widget_ids!(struct Ids { initial_menu, goto_stage_repl_button, exit_button, text });

pub fn main() {
    let mut app = App {
        stage: Stage::Welcome,
        exit_requested: false,
        escape_key_state: glium::glutin::ElementState::Released,
    };

    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 768;

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Sound Garden")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([f64::from(WIDTH), f64::from(HEIGHT)])
        .theme(theme())
        .build();

    // Generate the widget identifiers.
    let ids = Ids::new(ui.widget_id_generator());

    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();
    let font_path = assets.join("fonts/IBMPlex/IBM-Plex-Mono/IBMPlexMono-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut event_loop = EventLoop::new();

    'render: loop {
        // Process the events.
        for event in event_loop.next(&mut events_loop) {
            // Break from the loop upon `Escape` or closed window.
            match event.clone() {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    glium::glutin::WindowEvent::CloseRequested => break 'render,
                    glium::glutin::WindowEvent::KeyboardInput {
                        input:
                            glium::glutin::KeyboardInput {
                                // TODO exit only from Welcome stage, otherwise go to Welcome stage
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                state,
                                ..
                            },
                        ..
                    } => {
                        if app.escape_key_state == glium::glutin::ElementState::Pressed
                            && state == glium::glutin::ElementState::Released
                        {
                            match app.stage {
                                Stage::Welcome => break 'render,
                                _ => app.stage = Stage::Welcome,
                            }
                        }
                        app.escape_key_state = state
                    }
                    _ => (),
                },
                _ => (),
            };

            // Use the `winit` backend feature to convert the winit event to a conrod input.
            let input = match conrod::backend::winit::convert_event(event, &display) {
                None => continue,
                Some(input) => input,
            };

            // Handle the input with the `Ui`.
            ui.handle_event(input);

            loop {
                let ui = &mut ui.set_widgets();
                let stage = app.stage.clone();
                match &app.stage {
                    Stage::Welcome => render_welcome_stage(&mut app, ui, &ids),
                    Stage::REPL => render_repl_stage(&mut app, ui, &ids),
                }
                if stage == app.stage {
                    break;
                }
            }

            if app.exit_requested {
                break 'render;
            }
        }

        // Draw the `Ui` if it has changed.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

/// Initial screen.
///
/// Renders centered column of button (a-la arcade games) to go specific stage or exit.
fn render_welcome_stage(app: &mut App, ui: &mut conrod::UiCell, ids: &Ids) {
    widget::Canvas::new()
        .border(0.0)
        .middle()
        .w_h(300.0, 220.0)
        .set(ids.initial_menu, ui);
    for _press in widget::Button::new()
        .label("REPL")
        .mid_top_of(ids.initial_menu)
        .w_h(300.0, 100.0)
        .parent(ids.initial_menu)
        .set(ids.goto_stage_repl_button, ui)
    {
        app.stage = Stage::REPL;
    }
    for _press in widget::Button::new()
        .label("Exit")
        .w_h(300.0, 100.0)
        .parent(ids.initial_menu)
        .set(ids.exit_button, ui)
    {
        app.exit_requested = true;
    }
}

fn render_repl_stage(app: &mut App, ui: &mut conrod::UiCell, ids: &Ids) {
    widget::Text::new("TODO repl here")
        .middle_of(ui.window)
        .color(conrod::color::RED)
        .font_size(32)
        .set(ids.text, ui);
}

fn theme() -> conrod::Theme {
    use conrod::position::{Align, Direction, Padding, Position, Relative};
    conrod::Theme {
        name: "Sound Garden".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod::color::BLACK,
        shape_color: conrod::color::BLACK,
        border_color: conrod::color::WHITE,
        border_width: 1.0,
        label_color: conrod::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

pub struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(
        &mut self,
        events_loop: &mut glium::glutin::EventsLoop,
    ) -> Vec<glium::glutin::Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        // If there are no events and the UI does not need updating, wait
        // for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether
    /// or not there are any pending events.
    ///
    /// This is primarily used on the occasion that some part of the UI is
    /// still animating and requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}

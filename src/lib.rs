#[macro_use] extern crate conrod;
extern crate piston_window;
extern crate spiro;

use piston_window::{EventLoop, PistonWindow, UpdateEvent, WindowSettings};
use std::sync::mpsc::{channel, Sender, Receiver};

widget_ids! {
    struct Ids {
        background,
        scrollbar_x,
        scrollbar_y,
        username_label,
        username_input,
        number_label,
        number_input,
        y_input,
        send_button
    }
}

pub struct AppState {
    x_input_value: f64,
    y_input_value: f64,
}

pub fn run_gui(socket_addr: &str) {
    use std::path::{PathBuf, Path};

    // Construct the window.
    let mut window: PistonWindow =
    WindowSettings::new("Spiro", [720, 360])
            .opengl(piston_window::OpenGL::V3_2)
            .vsync(true)
            .samples(4)
            .exit_on_esc(true)
            .build()
            .unwrap();

    window.set_ups(60);

    let mut ui = conrod::UiBuilder::new().build();
    ui.fonts.insert_from_file(Path::new("./assets/NotoSans/NotoSans-Regular.ttf")).unwrap();
    // A unique identifier for each widget.
    let ids = Ids::new(ui.widget_id_generator());

    let mut text_texture_cache = conrod::backend::piston_window::GlyphCache::new(&mut window, 1000, 500);

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::new();

    let mut app_state = AppState {
        y_input_value: 10.0,
        x_input_value: 20.0,
    };

    let sender = create_client(socket_addr);

    // Poll events from the window.
    while let Some(event) = window.next() {
        // Convert the piston event to a conrod event.
        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| {
            use conrod::{color, widget, Colorable, Borderable, Positionable, Sizeable, Labelable, Widget};

            let mut ui = ui.set_widgets();

            widget::Canvas::new()
                    .color(color::DARK_CHARCOAL)
                    .pad(20.0)
                    .border(5.0)
                    .scroll_kids()
                    .set(ids.background, &mut ui);

            widget::Scrollbar::x_axis(ids.background).auto_hide(true).set(ids.scrollbar_x, &mut ui);
            widget::Scrollbar::y_axis(ids.background).auto_hide(true).set(ids.scrollbar_y, &mut ui);

            widget::Text::new("Input Value")
                    .top_left_with_margins_on(ids.background, 0.0, 80.0)
                    .font_size(64)
                    .color(color::DARK_YELLOW)
                    .set(ids.number_label, &mut ui);

            let x_label = format!("X: {:.2}", app_state.x_input_value);
            let new_x_value = widget::Slider::new(app_state.x_input_value, 0.0, 1000.0)
                    .w_h(350.0, 45.0)
                    .mid_left_of(ids.background)
                    .down_from(ids.number_label, 45.0)
                    .rgb(0.5, 0.3, 0.6)
                    .label(&x_label)
                    .set(ids.number_input, &mut ui);

            if let Some(x) = new_x_value {
                app_state.x_input_value = x;
            }

            let y_label = format!("Y: {:.2}", app_state.y_input_value);
            let new_y_value = widget::Slider::new(app_state.y_input_value, 0.0, 1000.0)
                    .w_h(350.0, 45.0)
                    .mid_left_of(ids.background)
                    .down_from(ids.number_input, 45.0)
                    .rgb(0.5, 0.3, 0.6)
                    .label(&x_label)
                    .set(ids.y_input, &mut ui);

            if let Some(y) = new_y_value {
                app_state.y_input_value = y;
            }

            let send_button = widget::Button::new()
                    .w_h(200.0, 75.0)
                    .mid_right_of(ids.background)
                    .label("Send it!")
                    .set(ids.send_button, &mut ui);
            if send_button.was_clicked() {
                println!("**** Send: X: {}, Y: {}", app_state.x_input_value, app_state.y_input_value);
                sender.send((app_state.x_input_value, app_state.y_input_value)).unwrap();
            }


        });

        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T { img };
                conrod::backend::piston_window::draw(c, g, primitives,
                                                     &mut text_texture_cache,
                                                     &image_map,
                                                     texture_from_image);
            }
        });
    }

    fn create_client(socket_addr: &str) -> Sender<(f64, f64)> {
        use spiro::Client;

        let (sender, receiver) = channel::<(f64, f64)>();

        let addr = socket_addr.to_owned();

        std::thread::spawn(move || {
            loop {
                let mut client: Client = Client::new(&addr).unwrap();
                receiver.recv().map(|(x, y)| {
                    client.send_params(x, y);
                });
            }
        });
        sender
    }

}


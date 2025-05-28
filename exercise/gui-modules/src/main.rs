//！ /usr/bin/env rustc

mod widgets;

use widgets::{Button, Label, Widget, Window};

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new("Click me!")));
    window.draw() ?;

    Ok(())
}

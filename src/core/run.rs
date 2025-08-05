use crate::constants::TOOLBAR_HEIGHT;
use crate::core::ui::object::Object;
use crate::core::ui::toolbar::Toolbar;
use macroquad::prelude::*;

pub async fn run() {
    let mut toolbar = Toolbar::new(screen_width(), TOOLBAR_HEIGHT);
    loop {
        // scale with screen
        toolbar.width = screen_width();
        clear_background(BLACK);

        toolbar.handle_input();
        toolbar.draw();

        next_frame().await
    }
}
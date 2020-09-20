#[cfg(test)]
#[allow(unused_must_use)]
mod mouse {
    use mouse_rs::{types::keys::Keys, Mouse};

    #[test]
    fn move_and_press() {
        let mouse = Mouse::new();
        mouse.move_to(500, 500);
        mouse.press(&Keys::RIGHT).expect("Unable to press button");
        mouse.release(&Keys::RIGHT).expect("Something went wrong");
    }

    #[test]
    fn scroll_wheel() {
        let mouse = Mouse::new();
        mouse.wheel(1);
    }

    #[test]
    fn press_button() {
        let mouse = Mouse::new();
        mouse.press(&Keys::MIDDLE);
        mouse.release(&Keys::MIDDLE);
    }

    #[test]
    fn print_post() {
        let mouse = Mouse::new();
        println!(
            "{:?}, {:?}",
            mouse.get_position().unwrap().y,
            mouse.get_position().unwrap().x
        );
    }
}

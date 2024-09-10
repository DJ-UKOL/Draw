use Draw::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

use ::Draw::Button;
use ::Draw::Screen;


fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    "Yes".to_string(),
                    "Maybe".to_string(),
                    "No".to_string(),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: "OK".to_string(),
            }),
        ],
    };

    screen.run();
}

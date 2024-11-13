use bbel_common::structures::MenuOption;
use bbel_common::terminal::{cls, show_menu};

fn main() {
    let mut options: Vec<MenuOption> = vec![
        MenuOption {
            code: 1,
            title: "Hello World!".to_string(),
            on_select: Box::new(|| {
                println!("Hello World!");
            }),
        },
        MenuOption {
            code: 2,
            title: "Segundo menú".to_string(),
            on_select: Box::new(|| {
                let mut second_menu = vec![MenuOption {
                    code: 1,
                    title: "Hello Second!".to_string(),
                    on_select: Box::new(|| println!("Hello second!")),
                }];
                cls();
                show_menu("SEGUNDO MENU", &mut second_menu, false);
            }),
        },
    ];

    show_menu("BBEL HELLO", &mut options, true);
}

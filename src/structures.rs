pub struct MenuOption {
    pub code: u8,
    pub title: String,
    pub on_select: Box<dyn Fn()>,
}

impl MenuOption {
    pub fn execute(&self) {
        (self.on_select)();
    }
}

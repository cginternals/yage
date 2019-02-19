use azul::prelude::*;

struct MyDataModel { }

impl Layout for MyDataModel {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        Dom::new(NodeType::Div)
    }
}

fn main() {
    let app = App::new(MyDataModel { }, AppConfig::default());
    let window = Window::new(WindowCreateOptions::default(), css::native()).unwrap();
    app.run(window).unwrap();
}

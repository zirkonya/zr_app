use zr_app::config_builder;

config_builder! {
    App {
        title: String,
        foo: Foo {
            value: i32,
        }
    }
}

#[zr_app::app(conf = App, app_folder = "./examples/conf")]
fn main() {
    println!("title: {}; value: {}", config.title, config.foo.value);
}

use zr_app::config::get_config;
use zr_app::config_builder;

config_builder! {
    App {
        title: String,
        foo: Foo {
            value: i32,
        }
    }
}

fn main() {
    let app: App = get_config("file.conf");
    println!("title: {}; value: {}", app.title, app.foo.value);
}

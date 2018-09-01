extern crate noctis_native;

use noctis_native::Window;

fn main() {
    let w = Window::new();

    match w {
        Ok(window) => {
            run(window);
        },
        Err(err) => {
            println!("Nooo ! {}", err)
        }
    }
}

fn run(mut window: Window) {
    println!("Running !");

    window.enable_content_over_titlebar(true);
    window.set_title_displayed(false);
    window.set_titlebar_big(true);

    window.run();
}
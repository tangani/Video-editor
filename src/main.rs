use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
// use gstreamer::prelude::*;
// use gstreamer::ElementFactory;
// use gstreamer_video::VideoOverlayExt;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");
    // gstreamer::init().expect("Failed to initialize GStreamer.");


    let application = Application::new(
        Some("com.github.video_editor"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Video Editor");
        window.set_default_size(800, 600);
        window.show_all();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        window.add(&button);

        window.show_all();
    });

    application.run();

    // println!("GStreamer version: {}", gstreamer::version_string());

    
}

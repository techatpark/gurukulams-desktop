use gtk::{prelude::*, Builder};
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("com.gurukula.App")
        .build();

    app.connect_startup(move |app| {  
        build_ui(app);
    });

    app.run();
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("gtk-ui.glade");
    let builder = Builder::new();
    builder
        .add_from_string(glade_src)
        .expect("Builder couldn't add from string");
    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    register_nav_events(builder);

     // Don't forget to make all widgets visible.
    window.show_all();

}

fn register_nav_events(builder: Builder) {
    let navigator_view: gtk::Widget= builder.object("navigator_view").unwrap();
    let markdown_editor_view: gtk::Widget= builder.object("markdown_editor_view").unwrap();
    let toggle_edit_button: gtk::ToolButton = builder.object("toggle_edit_button").unwrap();
    let toggle_nav_button: gtk::ToolButton = builder.object("toggle_nav_button").unwrap();

    let scrolled_window_left: gtk::ScrolledWindow = builder.object("scrolled_window_left").unwrap();
    let left_stack: gtk::Stack = builder.object("left_stack").unwrap();

    toggle_edit_button
        .connect_clicked(move |_|  {
            scrolled_window_left.set_visible(!scrolled_window_left.get_visible());
            left_stack.set_visible_child(&markdown_editor_view);
    });

    let scrolled_window_left: gtk::ScrolledWindow = builder.object("scrolled_window_left").unwrap();
    let left_stack: gtk::Stack = builder.object("left_stack").unwrap();

    toggle_nav_button
        .connect_clicked(move |_| {
            scrolled_window_left.set_visible(!scrolled_window_left.get_visible());
            left_stack.set_visible_child(&navigator_view);
    });
}
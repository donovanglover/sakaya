use libadwaita as adw;

use adw::prelude::*;
use adw::{ActionRow, ApplicationWindow};
use gtk::{Application, Box, Orientation};

fn activate(application: &gtk::Application) {
    // ActionRows are only available in Adwaita
    let row = ActionRow::builder()
        .activatable(true)
        .selectable(false)
        .title("Click me")
        .build();
    row.connect_activated(|_| {
        eprintln!("Clicked!");
    });
    // Combine the content in a box
    let content = Box::new(Orientation::Vertical, 0);
    content.append(&row);

    let window = ApplicationWindow::builder()
        .application(application)
        .default_width(350)
        // add content to window
        .content(&content)
        .build();

    window.set_visible(true);

    // Create a normal GTK window however you like
    let window = gtk::ApplicationWindow::new(application);

    // Before the window is first realized, set it up to be a layer surface
    gtk_layer_shell::init_for_window(&window);

    // Display it above normal windows
    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);

    // Push other windows out of the way
    gtk_layer_shell::auto_exclusive_zone_enable(&window);

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Left, 40);
    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Right, 40);
    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Top, 20);

    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    let anchors = [
        (gtk_layer_shell::Edge::Left, true),
        (gtk_layer_shell::Edge::Right, true),
        (gtk_layer_shell::Edge::Top, false),
        (gtk_layer_shell::Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        gtk_layer_shell::set_anchor(&window, anchor, state);
    }

    // Set up a widget
    let label = gtk::Label::new(Some(""));
    label.set_markup("<span font_desc=\"20.0\">GTK Layer Shell example!</span>");
    window.set_child(Some(&label));
    window.set_visible(true)
}

fn main() {
    let application = Application::builder()
        .application_id("sakaya")
        .build();

    application.connect_startup(|_| {
        adw::init().unwrap();
    });

    application.connect_activate(|app| {
        activate(app);
    });

    application.run();
}

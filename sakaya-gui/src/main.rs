// This example is copied from https://gitlab.gnome.org/World/Rust/libadwaita-rs/-/blob/master/libadwaita/examples/hello-world.rs
// The only part that is specific to the usage of gtk4-layer-shell is encased by comments

use libadwaita as adw;

use adw::prelude::*;
use adw::{ActionRow, ApplicationWindow, HeaderBar};
use gtk::{Application, Box, ListBox, Orientation};

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();

    application.connect_startup(|_| {
        adw::init().unwrap();
    });

    application.connect_activate(|app| {
        // ActionRows are only available in Adwaita
        let row = ActionRow::builder()
            .activatable(true)
            .selectable(false)
            .title("Click me")
            .build();
        row.connect_activated(|_| {
            eprintln!("Clicked!");
        });

        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            // the content class makes the list look nicer
            .css_classes(vec![String::from("content")])
            .build();
        list.append(&row);

        // Combine the content in a box
        let content = Box::new(Orientation::Vertical, 0);
        // Adwaitas' ApplicationWindow does not include a HeaderBar
        content.append(
            &HeaderBar::builder()
                .title_widget(&adw::WindowTitle::new("First App", ""))
                .build(),
        );
        content.append(&list);

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(350)
            // add content to window
            .content(&content)
            .build();

        // #################################
        // Part that is specific to use gtk4-layer-shell begins

        // Before the window is first realized, set it up to be a layer surface
        gtk_layer_shell::init_for_window(&window);

        // Display above normal windows
        gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);

        // Push other windows out of the way
        gtk_layer_shell::auto_exclusive_zone_enable(&window);

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
        // Part that is specific to use gtk4-layer-shell ends
        // #################################

        window.set_visible(true);
    });

    application.run();
}

mod process;


use gtk::gdk_pixbuf::Pixbuf;
use process::ProcessData;
use sysinfo::{System, SystemExt};

use gtk::{prelude::*, ListStore, TreeIter, TreeModel};
use gtk::{Application, ApplicationWindow, ListBox, Box};
use gtk::*;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    let sys = System::new_all();

   // let p = ProcessData;
   // p.get_processes(sys);
    app.run();
}

fn build_ui(app: &Application) {
    let mbox = Box::builder()
    .valign(gtk::Align::Center)
    .halign(gtk::Align::Center)
    .hexpand(true)
    .build();

    let left_tree = TreeView::new();
    let tipos_colunas = [Pixbuf::static_type(), String::static_type()];
    let left_store = TreeStore::new(&tipos_colunas );

    left_tree.set_model(Some(&left_store));
    left_tree.set_headers_visible(true);
    //Append na coluna
    let column = TreeViewColumn::new();

    //Renderizadores
    let renderer = CellRendererPixbuf::new();
    let cell = CellRendererText::new();

    column.set_title("Titulo");
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "pixbuf", 0);
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 1);

    left_tree.append_column(&column);

    //Item pai
    let image = Pixbuf::from_file("./resources/icon.png").or_else(|err| {
            let mut msg = err.to_string();
            if err.kind() == Some(glib::FileError::Noent) {
                msg.push_str(
                    "\nRelaunch this example from the same level \
                     as the `resources` folder",
                );
            }
            Err(())
        })
        .ok();

    for i in 0..10 {
    
        left_store.insert_with_values(None, None, &[(0, &image), (1,&("Filhu"))]);
        //filho
    }


    mbox.append(&left_tree);
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&mbox)
        .build();

    // Present window
    window.present();
}
extern crate paprika;
extern crate gtk;
extern crate gdk;
extern crate gio;

use gtk::{ WidgetExt, WindowExt, ContainerExt,GtkApplicationExt, TextViewExt, TextBufferExt};
use gio::{ ApplicationExt, MenuExt, SimpleActionExt, ActionMapExt };

const DEFAULT_WINDOW_HEIGHT:i32 = 600;
const DEFAULT_WINDOW_WIDTH:i32 = 800;

fn lanuch_window(){
        match gtk::Application::new("com.github.ItinoseSan.paprika", gio::APPLICATION_HANDLES_OPEN) {
         Ok(app) => {
                app.connect_activate(|app| {
                let browser_window = gtk::ApplicationWindow::new(&app);
                let scroll_bar = gtk::ScrolledWindow::new(None,None);

                let window_action = gio::SimpleAction::new("new_window",None);{
                    let app = app.clone();
                    window_action.connect_activate(move |_, _| {
                        create_window(&app);
                    });
                }
                app.add_action(&window_action);
                {
                    use gio::{Menu, MenuItem};
                    let menubar = Menu::new();
                    let submenu_file = Menu::new();
                    let newwindow = MenuItem::new("New Browser Window", "app.new_window");
                    submenu_file.append_item(&newwindow);
                    menubar.append_submenu("File", &submenu_file);
                    app.set_menubar(&menubar);
                }
                browser_window.set_default_size(
                    DEFAULT_WINDOW_WIDTH,DEFAULT_WINDOW_HEIGHT
                );
                browser_window.set_title("paprika");            
                browser_window.add(&scroll_bar);
                browser_window.show_all();
            });
            app.run(&[""]);
        },
        Err(_) => {
            println!("Application start up error");
        }
    };
}

fn create_window(app: &gtk::Application) -> gtk::ApplicationWindow {
        let win = gtk::ApplicationWindow::new(app);
        win.set_default_size(800, 600);
        win.set_title("paprika");
        let scr_win = gtk::ScrolledWindow::new(None, None);
        win.add(&scr_win);
        win.show_all();
        win
}

fn main(){
    lanuch_window();
}
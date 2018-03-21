extern crate gtk;
extern crate gdk;
extern crate gio;
extern crate reqwest;

use gtk::{ WindowExt, WidgetExt, ContainerExt, GtkApplicationExt, };
use gio::{ ApplicationExt, MenuExt, ActionMapExt, SimpleActionExt};

const DEFAULT_WINDOW_HEIGHT:i32 = 600;
const DEFAULT_WINDOW_WIDTH:i32 = 800;

pub struct BrowserWindow{}

impl BrowserWindow{
    pub fn new() -> BrowserWindow{
        BrowserWindow{}
    }
    pub fn lanuch_window(&self){
        match gtk::Application::new("com.github.ItinoseSan.paprika", gio::APPLICATION_HANDLES_OPEN) {
         Ok(app) => {
                app.connect_activate(|app| {
                let browser_window = gtk::ApplicationWindow::new(&app);
                let scroll_bar = gtk::ScrolledWindow::new(None,None);
                let window_action = gio::SimpleAction::new("new_window",None);
                
                {
                    let app = app.clone();
                    window_action.connect_activate(move |_, _| {
                        let new_window_action = WindowAction::new();
                        new_window_action.create_new_window(&app);
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
}

struct WindowAction{}

impl WindowAction{
    pub fn new() -> WindowAction{
        WindowAction {}
    }
    pub fn create_new_window(&self,app: &gtk::Application) -> gtk::ApplicationWindow{
          let new_window = gtk::ApplicationWindow::new(&app);
          new_window.set_default_size(
              DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT
          );
          new_window.set_title("paprika");
          let scroll_bar = gtk::ScrolledWindow::new(None, None);
          new_window.add(&scroll_bar);
          new_window.show_all();
          new_window
    }
}
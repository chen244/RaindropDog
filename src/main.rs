mod multi;
mod spider;
use futures::executor::block_on;
use gtk::prelude::*;
use gtk::{
    ApplicationWindow, CellRendererText, Label, ListStore, Orientation, TreeView, TreeViewColumn,
    WindowPosition,
};
use serde_json::Result;
use serde_json::Value;
use spider::{ascii_to_char, get_the_key};
use std::cell::RefCell;
#[derive(Copy, Clone)]
struct Active {
    is_running: i32,
    local: i32,
}

struct Ui {
    running_button: gtk::Button,
    ui_label: gtk::Label,
    func_label: gtk::Label,
    add_label: gtk::Label,
    port_label: gtk::Label,
    url_label: gtk::Label,
}
enum Tcp {
    Ss,
    V2,
}
#[allow(dead_code)]
#[derive(Clone)]
struct Urls {
    func: String,
    urls: String,
    add: String,
    aid: String,
    host: String,
    id: String,
    net: String,
    path: String,
    port: String,
    ps: String,
    tls: String,
    typpe: String,
}

thread_local! (
    static GLOBALURL: RefCell<Option<Vec<Urls>>> = RefCell::new(None)
);
thread_local!(
    static GLOBAL: RefCell<Option<Ui>> = RefCell::new(None)
);
thread_local!(
    static GLOBAL2: RefCell<Active> = RefCell::new(Active {
        is_running: -1,
        local: 0,
    });
);

fn create_and_fill_model(model: &ListStore, temp: Vec<String>) {
    fn ascii_to_string(code: Vec<u8>) -> String {
        let mut output: String = String::new();
        for cor in code.into_iter() {
            output.push(ascii_to_char(cor));
        }
        output
    }
    fn type_of_url(url: String) -> Tcp {
        for pair in url.chars() {
            if pair == 's' {
                return Tcp::Ss;
            }
            if pair == 'v' {
                return Tcp::V2;
            }
        }
        Tcp::Ss
    }
    fn get_the_url(url: String) -> Urls {
        let func = type_of_url(url.clone());
        match func {
            Tcp::Ss => Urls {
                urls: url,
                func: "\"ss\"".to_string(),
                add: "\"unknown\"".to_string(),
                aid: "\"unknown\"".to_string(),
                host: "\"unknown\"".to_string(),
                id: "\"unknown\"".to_string(),
                net: "\"unknown\"".to_string(),
                path: "\"unknown\"".to_string(),
                port: "\"unknown\"".to_string(),
                ps: "\"unknown\"".to_string(),
                tls: "\"unknown\"".to_string(),
                typpe: "\"unknown\"".to_string(),
            },
            Tcp::V2 => {
                let newurl = &url[8..];
                let json = ascii_to_string(base64::decode(newurl.to_string().as_bytes()).unwrap());
                let v: Result<Value> = serde_json::from_str(json.as_str());
                match v {
                    Ok(input) => {
                        Urls {
                            //company : input["add"].to_string(),
                            urls: url,
                            func: "\"vmess\"".to_string(),
                            add: input["add"].to_string(),
                            aid: input["aid"].to_string(),
                            host: input["host"].to_string(),
                            id: input["id"].to_string(),
                            net: input["net"].to_string(),
                            path: input["path"].to_string(),
                            port: input["port"].to_string(),
                            ps: input["ps"].to_string(),
                            tls: input["tls"].to_string(),
                            typpe: input["type"].to_string(),
                        }
                    }
                    Err(_) => Urls {
                        urls: url,
                        func: "\"vmess\"".to_string(),
                        add: "\"unknown\"".to_string(),
                        aid: "\"unknown\"".to_string(),
                        host: "\"unknown\"".to_string(),
                        id: "\"unknown\"".to_string(),
                        net: "\"unknown\"".to_string(),
                        path: "\"unknown\"".to_string(),
                        port: "\"unknown\"".to_string(),
                        ps: "\"unknown\"".to_string(),
                        tls: "\"unknown\"".to_string(),
                        typpe: "\"unknown\"".to_string(),
                    },
                }
            }
        }
    }
    let future = get_the_key(temp);
    let output: Vec<Vec<String>> = block_on(future).unwrap();
    let mut input: Vec<String> = vec![];
    let mut urls: Vec<Urls> = vec![];
    for pair in output.into_iter() {
        for pair2 in pair.into_iter() {
            let url_local = get_the_url(pair2);
            let temp = url_local.ps.clone();
            urls.push(url_local);
            //let temp = pair2.clone();
            input.push(temp);
        }
    }
    GLOBALURL.with(move |global| {
        *global.borrow_mut() = Some(urls);
    });
    let entries = &input;
    for (i, entry) in entries.iter().enumerate() {
        model.insert_with_values(None, &[(0, &(i as u32)), (1, &entry)]);
    }
    //model
}

fn append_column(tree: &TreeView, id: i32) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}

fn create_and_setup_view() -> TreeView {
    // Creating the tree view.
    let tree = TreeView::new();

    tree.set_headers_visible(false);
    // Creating the two columns inside the view.
    append_column(&tree, 0);
    append_column(&tree, 1);
    tree
}

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("Simple TreeView example");
    window.set_position(WindowPosition::Center);
    window.set_size_request(300, 300);

    // Creating a vertical layout to place both tree view and label in the window.
    let vertical_layout = gtk::Box::new(Orientation::Horizontal, 0);

    // Creation of the label.
    let label = Label::new(None);
    let label_func = Label::new(Some("func:"));
    label_func.set_justify(gtk::Justification::Left);
    let label_add = Label::new(Some("add:"));
    label_add.set_justify(gtk::Justification::Left);
    let label_port = Label::new(Some("port:"));
    label_port.set_justify(gtk::Justification::Left);
    let label_url = Label::new(Some("url"));
    label_url.set_justify(gtk::Justification::Left);
    label_url.set_max_width_chars(10);
    label_url.set_line_wrap(true);
    //label_url.set_wrap(true);
    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let h_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    h_box.pack_start(&label_func, false,true,0);
    //h_box.pack_start(child, expand, fill, padding)
    h_box.pack_start(&label_add, false,true,0);
    h_box.pack_start(&label_port, false,true,0);
    h_box.pack_start(&label_url, false,true,0);
    h_box.pack_start(&label, true,true,0);
    let button_box = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
    button_box.set_layout(gtk::ButtonBoxStyle::End);
    let button1 = gtk::Button::with_label("new");
    let button2 = gtk::Button::with_label("copy");

    button_box.pack_start(&button1, false, false, 0);
    button_box.pack_start(&button2, false, false, 0);

    v_box.pack_start(&button_box, false, true, 0);
    v_box.pack_start(&h_box, true, true, 0);

    let tree = create_and_setup_view();

    let temp: Vec<String> = vec![];
    let model = ListStore::new(&[u32::static_type(), String::static_type()]);
    create_and_fill_model(&model, temp);
    button2.connect_clicked(
        glib::clone!(@weak model,@weak application,@weak window => move |_|{
        multi::create_sub_window(&application, "input urls",create_and_fill_model,&model,&window);
        }),
    );
    //let model = create_and_fill_model(temp);
    // Setting the model into the view.
    tree.set_model(Some(&model));
    let scroll = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scroll.add(&tree);
    //设置最小的大小
    scroll.set_width_request(300);
    scroll.set_resize_mode(gtk::ResizeMode::Queue);
    //禁止水平变化
    vertical_layout.pack_start(&scroll, false, true, 0);
    vertical_layout.pack_start(&v_box, true, true, 0);
    // Iter 可以获取内容，但是active可以获取目录位置
    // 准确来说，active需要点两次
    // 移动到global里去，主要是为了方便改写和访问，这样变量就不会首主进程影响
    GLOBAL.with(move |global| {
        *global.borrow_mut() = Some(Ui {
            running_button: button1,
            ui_label: label,
            func_label : label_func,
            add_label: label_add,
            port_label: label_port,
            url_label: label_url,
        });
        if let Some(ref ui) = *global.borrow() {
            ui.running_button.connect_clicked(move |s| {
                GLOBAL2.with(move |global2| {
                    let locall = *global2.borrow();
                    let temp = locall.local;
                    if locall.is_running == locall.local {
                        s.set_label("start");
                        *global2.borrow_mut() = Active {
                            is_running: -1,
                            local: temp,
                        };
                    } else {
                        s.set_label("stop");
                        //println!("{},{}",locall.is_running,locall.local);
                        *global2.borrow_mut() = Active {
                            is_running: temp,
                            local: temp,
                        };
                    }
                });
            });
        }
    });
    //active时候更改label
    tree.connect_row_activated(move |_, path, _column| {
        GLOBAL.with(move |global| {
            if let Some(ref ui) = *global.borrow() {
                ui.ui_label.set_text(&format!("index{}", path.indices()[0]));
                GLOBAL2.with(move |global2| {
                    //let locall = *global2.borrow();
                    *global2.borrow_mut() = Active {
                        is_running: path.indices()[0],
                        local: path.indices()[0],
                    };
                    ui.running_button.set_label("stop");
                });
            }
        });
    });
    // cursor 聚焦时候就更改
    tree.connect_cursor_changed(move |tree_view| {
        GLOBAL.with(move |global| {
            if let Some(ref ui) = *global.borrow() {
                let selection = tree_view.selection();
                if let Some((model, iter)) = selection.selected() {
                    // Now getting back the values from the row corresponding to the
                    // iterator `iter`.
                    //
                    // The `get_value` method do the conversion between the gtk type and Rust.
                    ui.ui_label.set_text(&format!(
                        "Hello '{}' from rom {}",
                        model
                            .value(&iter, 1)
                            .get::<String>()
                            .expect("Treeview selection, column 1"),
                        model
                            .value(&iter, 0)
                            .get::<u32>()
                            .expect("Treeview selection, column 0"),
                    ));
                    let local2 = model.value(&iter, 0).get::<u32>().expect("1") as i32;
                    GLOBALURL.with(move |global|{
                        if let Some(ref url) = *global.borrow() {
                            ui.func_label.set_text(&format!(
                                "func: {}",
                                url[local2 as usize].func.as_str()
                            ));
                            ui.add_label.set_text(&format!(
                                "add: {}",
                                url[local2 as usize].add.as_str()
                            ));
                            ui.port_label.set_text(&format!(
                                "port: {}",
                                url[local2 as usize].port.as_str()
                            ));
                            ui.url_label.set_text(&format!(
                                "url: {}",
                                url[local2 as usize].ps.as_str()
                            ));
                            ui.url_label.set_max_width_chars(10);
                            ui.url_label.set_line_wrap(true);

                        }
                    });
                    GLOBAL2.with(move |global2| {
                        let locall = *global2.borrow();
                        let running = locall.is_running;
                        *global2.borrow_mut() = Active {
                            is_running: running,
                            local: local2,
                        };
                        //locall.local = local;
                        if local2 != running {
                            //locall.is_running = path.indices()[0];
                            ui.running_button.set_label("start");
                        } else {
                            ui.running_button.set_label("stop");
                        }
                    });
                }
            }
        });
    });
    // Adding the layout to the window.
    window.add(&vertical_layout);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.simple_treeview"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}

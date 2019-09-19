use gtk::prelude::*;

use std::sync::{Arc, Mutex};

#[link(name="gl_bridge")]
extern {
    fn render();
}

fn main(){
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        return;
    }

    let state = Arc::new(Mutex::new(State::default()));

    let gui_str = include_str!("Settings.glade");
    let gui = gtk::Builder::new_from_string(gui_str);

    let win: gtk::ApplicationWindow = gui.get_object("Window").unwrap();
    win.connect_delete_event(|_, _| { gtk::main_quit(); Inhibit(false) });
    let setting_box: gtk::Box = gui.get_object("SettingsBox").unwrap();
    let setting_pane: gtk::ScrolledWindow = gui.get_object("SettingsPane").unwrap();
    setting_pane.add(&setting_box);

    let gen_box: gtk::Box = gui.get_object("GeneratorBox").unwrap();
    let ren_box: gtk::Box = gui.get_object("RendererBox").unwrap();
    let circle_settings: gtk::Box = gui.get_object("CircleSettings").unwrap();
    let island_settings: gtk::Box = gui.get_object("IslandSettings").unwrap();
    let inland_settings: gtk::Box = gui.get_object("InlandSettings").unwrap();
    let ogl_settings: gtk::Box = gui.get_object("OGLSettings").unwrap();
    let textured_settings: gtk::Box = gui.get_object("TexturedSettings").unwrap();
    let status_bar: gtk::Statusbar = gui.get_object("StatusBar").unwrap();
    let seed_box: gtk::Box = gui.get_object("SeedBox").unwrap();
    let glarea: gtk::GLArea = gui.get_object("GLArea").unwrap();

    glarea.connect_resize(move |_glarea, _h, _w| {
        unsafe {
            render();
        }
    });

    glarea.connect_render(move |_glarea, _context| {
        unsafe {
            render();
        }
        Inhibit(false)
    });

    win.connect_configure_event(move |_, _| {
        unsafe {
            render();
        }
        false
    });

    glarea.connect_realize(move |glarea| {
        glarea.make_current();
    });

    gen_box.pack_start(&circle_settings, false, false, 0);
    ren_box.pack_start(&ogl_settings, false, false, 0);
    let widgets = Arc::new(Mutex::new(Widgets{
        gen_box,
        ren_box,
        circle_settings,
        island_settings,
        inland_settings,
        ogl_settings,
        textured_settings,
        status_bar,
        seed_box,
        glarea
    }));

    {
        let size_x: gtk::Adjustment = gui.get_object("SizeX").unwrap();
        let state = Arc::clone(&state);
        size_x.connect_value_changed(move |size| size_x_changed(size, &state));
    }
    {
        let size_y: gtk::Adjustment = gui.get_object("SizeY").unwrap();
        let state = Arc::clone(&state);
        size_y.connect_value_changed(move |size| size_y_changed(size, &state));
    }
    {
        let seed: gtk::Adjustment = gui.get_object("Seed").unwrap();
        let state = Arc::clone(&state);
        seed.connect_value_changed(move |seed| seed_changed(seed, &state));
    }
    {
        let generator_setting: gtk::ComboBoxText = gui.get_object("GeneratorSetting").unwrap();
        let state = Arc::clone(&state);
        let widgets = Arc::clone(&widgets);
        generator_setting.connect_changed(move |combo_box_text| generator_changed(combo_box_text, &state, &widgets));
    }
    {
        let renderer_setting: gtk::ComboBoxText = gui.get_object("RendererSetting").unwrap();
        let state = Arc::clone(&state);
        let widgets = Arc::clone(&widgets);
        renderer_setting.connect_changed(move |combo_box_text| renderer_changed(combo_box_text, &state, &widgets));
    }
    {
        let random_seed: gtk::Switch = gui.get_object("RandomSeed").unwrap();
        let state = Arc::clone(&state);
        let widgets = Arc::clone(&widgets);
        random_seed.connect_state_set(move |_, switch_state| gtk::Inhibit(random_switch_changed(switch_state, &state, &widgets)));
    }

    // Don't forget to make all widgets visible.
    win.show_all();

    gtk::main();
}

pub struct Widgets {
    pub gen_box: gtk::Box,
    pub circle_settings: gtk::Box,
    pub island_settings: gtk::Box,
    pub inland_settings: gtk::Box,
    pub ren_box: gtk::Box,
    pub ogl_settings: gtk::Box,
    pub textured_settings: gtk::Box,
    pub status_bar: gtk::Statusbar,
    pub seed_box: gtk::Box,
    pub glarea: gtk::GLArea,
}

#[derive(Debug, Copy, Clone)]
pub struct State {
    size_x: u32,
    size_y: u32,
    random_seed: bool,
    seed: u32,
    gen: Generator,
    ren: Renderer
}

impl Default for State {
    fn default() -> Self {
        State{size_x: 100, size_y: 75, gen: Generator::Circle, ren: Renderer::Ogl, random_seed: true, seed: 0}
    }
}

impl State {
    fn set_size_x(&mut self, new_size: u32) {
        self.size_x = new_size;
    }

    fn set_size_y(&mut self, new_size: u32) {
        self.size_y = new_size;
    }

    fn set_gen(&mut self, new_gen: Generator) {
        self.gen = new_gen;
    }

    fn set_ren(&mut self, new_ren: Renderer) {
        self.ren = new_ren;
    }

    fn set_random_seed(&mut self, new_bool: bool) {
        self.random_seed = new_bool;
    }

    fn set_seed(&mut self, seed: u32) {
        self.seed = seed;
    }
}

fn size_x_changed(size: &gtk::Adjustment, state: &Arc<Mutex<State>>) {
    let size: u32 = size.get_value() as u32;
    state.lock().unwrap().set_size_x(size);
}

fn size_y_changed(size: &gtk::Adjustment, state: &Arc<Mutex<State>>) {
    let size: u32 = size.get_value() as u32;
    state.lock().unwrap().set_size_y(size);
}

fn seed_changed(seed: &gtk::Adjustment, state: &Arc<Mutex<State>>) {
    let seed: u32 = seed.get_value() as u32;
    state.lock().unwrap().set_seed(seed);
}

fn generator_changed(combo_box: &gtk::ComboBoxText, state: &Arc<Mutex<State>>, widgets: &Arc<Mutex<Widgets>>) {
    let str_choice = combo_box.get_active_text().unwrap();
    
    let choice = str_choice.as_str();

    let choice = match choice {
        "Circular" => Generator::Circle,
        "Islands" => Generator::Island,
        "Inland" => Generator::Inland,
        _ => panic!("invalid choice of generator")
    };

    {
        state.lock().unwrap().set_gen(choice);
        push_state_message(state, widgets);
    }

    {
        let widgets = widgets.lock().unwrap();
        let gen_box = &widgets.gen_box;

        gen_box.foreach(|child| gen_box.remove(child));

        match choice {
            Generator::Circle => gen_box.pack_start(&widgets.circle_settings, false, false, 0),
            Generator::Island => gen_box.pack_start(&widgets.island_settings, false, false, 0),
            Generator::Inland => gen_box.pack_start(&widgets.inland_settings, false, false, 0),
        }
    }
}

fn renderer_changed(combo_box: &gtk::ComboBoxText, state: &Arc<Mutex<State>>, widgets: &Arc<Mutex<Widgets>>) {
    let str_choice = combo_box.get_active_text().unwrap();
    
    let choice = str_choice.as_str();

    let choice = match choice {
        "OpenGL" => Renderer::Ogl,
        "OpenGL Textured" => Renderer::OglTextured,
        _ => panic!("invalid choice of generator")
    };

    {
        state.lock().unwrap().set_ren(choice);
        push_state_message(state, widgets);
    }

    {
        let widgets = widgets.lock().unwrap();
        let ren_box = &widgets.ren_box;

        ren_box.foreach(|child| ren_box.remove(child));

        match choice {
            Renderer::Ogl => ren_box.pack_start(&widgets.ogl_settings, false, false, 0),
            Renderer::OglTextured => ren_box.pack_start(&widgets.textured_settings, false, false, 0)
        }
    }
}

fn random_switch_changed(switch_state: bool, state: &Arc<Mutex<State>>, widgets: &Arc<Mutex<Widgets>>) -> bool {
    state.lock().unwrap().set_random_seed(switch_state);
    let seed_box = &widgets.lock().unwrap().seed_box;
    seed_box.set_sensitive(!switch_state);
    if switch_state {
        seed_box.set_opacity(0.5);
    } else {
        seed_box.set_opacity(1.0);
    }
    false
}

fn push_state_message(state: &Arc<Mutex<State>>, widgets: &Arc<Mutex<Widgets>>) {
    let status_bar = &widgets.lock().unwrap().status_bar;
    let string = format!("{:?}", state.lock().unwrap());
    status_bar.push(status_bar.get_context_id("test"), &string);
}

#[derive(Debug, Copy, Clone)]
pub enum Generator {
    Circle,
    Island,
    Inland
}

#[derive(Debug, Copy, Clone)]
pub enum Renderer {
    Ogl,
    OglTextured
}
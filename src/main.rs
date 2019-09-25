use gtk::prelude::*;
use rand::prelude::*;

use std::sync::{Arc, Mutex};
use std::ffi::CString;
use std::os::raw::c_char;
use std::slice;

#[link(name="gl_bridge")]
extern {
    fn render(width: i32, height: i32);
    fn load_shader(source_vert: *const c_char, source_frag: *const c_char);
    fn init_things();
}

fn main(){
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        return;
    }

    let vert_source = include_str!("vert.glsl");
    let frag_source = include_str!("frag.glsl");

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

    glarea.connect_resize(move |glarea, h, w| {
        println!("{}x{}", w,h);
        glarea.make_current();
        unsafe {
            render(w,h);
        }
    });

    glarea.connect_render(move |glarea, _context| {
        unsafe {
            render(glarea.get_allocated_width(), glarea.get_allocated_height());
        }
        Inhibit(false)
    });

    win.connect_configure_event(move |_, _| {
        unsafe {
            //render();
        }
        false
    });

    glarea.connect_realize(move |glarea| {
        glarea.make_current();
        let c_sh_vert = CString::new(vert_source).unwrap();
        let c_sh_frag = CString::new(frag_source).unwrap();
        unsafe {
            load_shader(c_sh_vert.as_ptr(), c_sh_frag.as_ptr());
            init_things();
        }
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


#[no_mangle]
pub extern "C" fn get_hex_verts(verts: *mut f32) {
    let verts = unsafe { slice::from_raw_parts_mut(verts, 12)};
    for i in 0..6 {
        let coords = get_hex_vertex(&Hex{center_x: 0.0, center_y: 0.0}, i);
        verts[2 * i] = coords.0;
        verts[2 * i + 1] = coords.1;
    }
}

//#[repr(C)]
pub struct InstanceData {
    offset_x: f32,
    offset_y: f32,
    r: f32,
    g: f32,
    b: f32,
}

#[no_mangle]
pub extern "C" fn get_instance_data(instances: *mut InstanceData) {
    let instances = unsafe { slice::from_raw_parts_mut(instances, 20)};
    for (i, hex) in instances.iter_mut().enumerate() {
        hex.offset_x = rand::random::<f32>() * 2.0 - 1.0;
        hex.offset_y = rand::random::<f32>() * 2.0 - 1.0;
        hex.r = rand::random();
        hex.g = rand::random();
        hex.b = rand::random();
    }
}

//TODO DELETE
/// This is roughly ratio of hexagon height to width
pub const RATIO: f32 = 1.154_700_538_38;

const HALF_RATIO: f32 = RATIO / 2.0;
const QUARTER_RATIO: f32 = RATIO / 4.0;

 fn get_hex_vertex(hex: &Hex, index: usize) -> (f32, f32) {
    if index > 5 {
        panic!("index out of range")
    }
    // get hex relative coords
    let mut coords = match index {
        0 => (0.5, -QUARTER_RATIO),
        1 => (0.5, QUARTER_RATIO),
        2 => (0.0, HALF_RATIO),
        3 => (-0.5, QUARTER_RATIO),
        4 => (-0.5, -QUARTER_RATIO),
        _ => (0.0, -HALF_RATIO),
    };
    // add absolute coords
    coords.0 += hex.center_x;
    coords.1 += hex.center_y;
    (coords.0, coords.1)
}

struct Hex {
    center_x: f32,
    center_y: f32
}
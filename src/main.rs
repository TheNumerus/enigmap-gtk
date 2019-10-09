use gtk::prelude::*;

use enigmap::HexMap;
use enigmap::generators::*;

use std::sync::{Arc, Mutex};

use enigmap_gtk::{state::*, widgets::Widgets, glarea};
use enigmap_gtk::ffi::gl_zoom_changed;

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
    let glbox: gtk::EventBox = gui.get_object("GLBox").unwrap();

    let (size_x, size_y) = {
        let state = state.lock().unwrap();
        (state.size_x, state.size_y)
    };

    let mut map = HexMap::new(size_x, size_y);

    let gen = Circle::default();
    gen.generate(&mut map);

    state.lock().unwrap().map = map;

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
        glarea,
        glbox
    }));

    {
        let state = Arc::clone(&state);
        let widgets = Arc::clone(&widgets);
        win.connect_key_press_event(move |_win, event| {
            if event.get_keyval() == gdk::enums::key::r || event.get_keyval() == gdk::enums::key::R {
                state.lock().unwrap().zoom_level = 0;
                gl_zoom_changed(1.0);
                widgets.lock().unwrap().glarea.queue_render();
            }
            Inhibit(false)
        });
    }


    {
        let widgets = Arc::clone(&widgets);
        glarea::connect_events(&state, widgets);
    }


    adjustnemt_update("SizeX", &state, &widgets, &gui, |value, state| {
        let size: u32 = value.get_value() as u32;
        state.lock().unwrap().set_size_x(size);
    });

    adjustnemt_update("SizeY", &state, &widgets, &gui, |value, state| {
        let size: u32 = value.get_value() as u32;
        state.lock().unwrap().set_size_y(size);
    });

    adjustnemt_update("Seed", &state, &widgets, &gui, |value, state| {
        let seed: u32 = value.get_value() as u32;
        state.lock().unwrap().seed = seed;
        match &mut state.lock().unwrap().gen {
            Generator::Circle(gen) => gen.set_seed(seed),
            Generator::Island(gen) => gen.set_seed(seed),
            Generator::Inland(gen) => gen.set_seed(seed)
        }
    });

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
        random_seed.connect_state_set(move |_, switch_state| {
            random_switch_changed(switch_state, &state, &widgets);
            regenerate_map(&state);
            glarea::reload_map(&state);
            widgets.lock().unwrap().glarea.queue_render();
            gtk::Inhibit(false)
        });
    }

    connect_circle_vars(&gui, &state, &widgets);

    // Don't forget to make all widgets visible.
    win.show_all();

    gtk::main();
}

fn adjustnemt_update<T: 'static>(name: &str, state: &Arc<Mutex<State>>, widgets: &Arc<Mutex<Widgets>>, gui: &gtk::Builder, on_change: T)
    where T: Fn(&gtk::Adjustment, &Arc<Mutex<State>>)
{
    let adj: gtk::Adjustment = gui.get_object(name).unwrap();
    let state = Arc::clone(&state);
    let widgets = Arc::clone(&widgets);
    adj.connect_value_changed(move |value| {
        on_change(value, &state);
        regenerate_map(&state);
        glarea::reload_map(&state);
        widgets.lock().unwrap().glarea.queue_render();
    });
}

fn connect_circle_vars(gui: &gtk::Builder, state: &Arc<Mutex<State>>, widgets: &Arc<Mutex<Widgets>>) {
    adjustnemt_update("Circle_RingSize", state, widgets, gui, |value, state| {
        let ring = value.get_value() as f32;
        let mut lock = state.lock().unwrap();
        if let Generator::Circle(gen) = &mut lock.gen {
            gen.ring_size = ring;
        }
    });

    adjustnemt_update("Circle_Ice", state, widgets, gui, |value, state| {
        let falloff = value.get_value() as f32;
        let mut lock = state.lock().unwrap();
        if let Generator::Circle(gen) = &mut lock.gen {
            gen.ice_falloff = falloff;
        }
    });

    adjustnemt_update("Circle_Percentage", state, widgets, gui, |value, state| {
        let percentage = value.get_value() as f32 / 100.0;
        let mut lock = state.lock().unwrap();
        if let Generator::Circle(gen) = &mut lock.gen {
            gen.mountain_percentage = percentage;
        }
    });

    adjustnemt_update("Circle_Ocean", state, widgets, gui, |value, state| {
        let dist = value.get_value() as u32;
        let mut lock = state.lock().unwrap();
        if let Generator::Circle(gen) = &mut lock.gen {
            gen.ocean_distance = dist;
        }
    });

    adjustnemt_update("Circle_Noise", state, widgets, gui, |value, state| {
        let noise = value.get_value() as f64;
        let mut lock = state.lock().unwrap();
        if let Generator::Circle(gen) = &mut lock.gen {
            gen.noise_scale = noise;
        }
    });

    adjustnemt_update("Circle_LandJitter", state, widgets, gui, |value, state| {
        let jitter = value.get_value() as f32;
        let mut lock = state.lock().unwrap();
        if let Generator::Circle(gen) = &mut lock.gen {
            gen.land_jitter = jitter;
        }
    });

    adjustnemt_update("Circle_Stickiness", state, widgets, gui, |value, state| {
        let stickiness = value.get_value() as u32;
        let mut lock = state.lock().unwrap();
        if let Generator::Circle(gen) = &mut lock.gen {
            gen.mountain_stickiness = stickiness;
        }
    });
}

fn regenerate_map(state: &Arc<Mutex<State>>) {
    // need to clone so we dont cause deadlock
    let mut map_clone = state.lock().unwrap().map.clone();
    match state.lock().unwrap().gen {
        Generator::Circle(gen) => gen.generate(&mut map_clone),
        Generator::Island(gen) => gen.generate(&mut map_clone),
        Generator::Inland(gen) => gen.generate(&mut map_clone)
    }
    state.lock().unwrap().map = map_clone;
}

fn generator_changed(combo_box: &gtk::ComboBoxText, state: &Arc<Mutex<State>>, widgets: &Arc<Mutex<Widgets>>) {
    let str_choice = combo_box.get_active_text().unwrap();
    
    let choice = str_choice.as_str();

    let choice = match choice {
        "Circular" => Generator::Circle(Circle::default()),
        "Islands" => Generator::Island(Islands::default()),
        "Inland" => Generator::Inland(Inland::default()),
        _ => panic!("invalid choice of generator")
    };

    {
        state.lock().unwrap().gen = choice;
        push_state_message(state, widgets);
    }

    {
        let widgets = widgets.lock().unwrap();
        let gen_box = &widgets.gen_box;

        gen_box.foreach(|child| gen_box.remove(child));

        match choice {
            Generator::Circle(_) => gen_box.pack_start(&widgets.circle_settings, false, false, 0),
            Generator::Island(_) => gen_box.pack_start(&widgets.island_settings, false, false, 0),
            Generator::Inland(_) => gen_box.pack_start(&widgets.inland_settings, false, false, 0),
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
        state.lock().unwrap().ren = choice;
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

fn random_switch_changed(switch_state: bool, state: &Arc<Mutex<State>>, widgets: &Arc<Mutex<Widgets>>) {
    state.lock().unwrap().random_seed = switch_state;
    if switch_state {
        match &mut state.lock().unwrap().gen {
            Generator::Circle(gen) => gen.reset_seed(),
            Generator::Island(gen) => gen.reset_seed(),
            Generator::Inland(gen) => gen.reset_seed()
        }
    } else {
        let seed = state.lock().unwrap().seed;
        match &mut state.lock().unwrap().gen {
            Generator::Circle(gen) => gen.set_seed(seed),
            Generator::Island(gen) => gen.set_seed(seed),
            Generator::Inland(gen) => gen.set_seed(seed)
        }
    }
    let seed_box = &widgets.lock().unwrap().seed_box;
    seed_box.set_sensitive(!switch_state);
    if switch_state {
        seed_box.set_opacity(0.5);
    } else {
        seed_box.set_opacity(1.0);
    }
}

fn push_state_message(state: &Arc<Mutex<State>>, widgets: &Arc<Mutex<Widgets>>) {
    let status_bar = &widgets.lock().unwrap().status_bar;
    let string = format!("{:?}", state.lock().unwrap());
    status_bar.push(status_bar.get_context_id("test"), &string);
}

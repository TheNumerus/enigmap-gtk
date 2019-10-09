use std::sync::{Arc, Mutex};

use gtk::prelude::*;

use enigmap::HexMap;
use enigmap::renderers::colors::ColorMap;

use crate::ffi::*;
use crate::state::State;
use crate::widgets::Widgets;

const VERT_SOURCE: &str = include_str!("vert.glsl");
const FRAG_SOURCE: &str = include_str!("frag.glsl");

pub fn connect_events(state: &Arc<Mutex<State>>, widgets: Arc<Mutex<Widgets>>) {

    {
        let state = Arc::clone(&state);
        let glarea = &widgets.lock().unwrap().glarea;
        glarea.connect_realize(move |glarea| {
            let lock = state.lock().unwrap();
            glarea.make_current();
            glarea.set_window(&glarea.get_parent_window().unwrap());
            gl_load_shader(VERT_SOURCE, FRAG_SOURCE);
            let instance_data = get_instance_data(&lock.map, &lock.color_map);
            gl_init_things();
            gl_load_instance_data(instance_data);
            gl_zoom_changed(1.0);
        });
    }

    {
        let glarea = &widgets.lock().unwrap().glarea;
        glarea.connect_resize(move |_glarea, w, h| {
            gl_window_resized(w, h);
        });
    }


    {
        let state = Arc::clone(&state);
        let glarea = &widgets.lock().unwrap().glarea;
        glarea.connect_render(move |_glarea, _context| {
            _glarea.make_current();
            let (abs_size_x, abs_size_y) = {
                let state = state.lock().unwrap();
                (state.map.absolute_size_x, state.map.absolute_size_y)
            };
            gl_map_resized(abs_size_x, abs_size_y);
            gl_render();
            Inhibit(false)
        });
    }

    {
        let glarea = &widgets.lock().unwrap().glarea;
        glarea.connect_unrealize(move |_glarea| {
            gl_cleanup();
        });
    }

    {
        let state = Arc::clone(&state);
        let glbox = &widgets.lock().unwrap().glbox;
        let widgets = Arc::clone(&widgets);
        glbox.connect_scroll_event(move |_glbox, event| {
            let dir = event.get_direction();
            let zoom = &mut state.lock().unwrap().zoom_level;
            match dir {
                gdk::ScrollDirection::Up => *zoom+=1,
                gdk::ScrollDirection::Down => *zoom-=1,
                _ => {}
            }

            // converts zoom value to float multiple
            let converted = if *zoom >= 0 {
                1.0 + *zoom as f32 * 0.2
            } else {
                (0.2 * *zoom as f32).exp()
            };
            gl_zoom_changed(converted);
            widgets.lock().unwrap().glarea.queue_render();
            Inhibit(false)
        });
    }
}

#[repr(C)]
pub struct InstanceData {
    offset_x: f32,
    offset_y: f32,
    r: f32,
    g: f32,
    b: f32,
}

pub fn get_instance_data(map: &HexMap, colors: &ColorMap) -> Vec<InstanceData> {
    let size = map.get_area() as usize;
    let mut instances = Vec::with_capacity(size);
    for i in 0..size {
        let color = colors.get_color_f32(&map.field[i].terrain_type);
        instances.push(InstanceData{
            offset_x: map.field[i].center_x,
            offset_y: map.field[i].center_y,
            r: color.r,
            g: color.g,
            b: color.b
        });
    }
    instances
}

pub fn reload_map(state: &Arc<Mutex<State>>) {
    let lock = state.lock().unwrap();
    let instance_data = get_instance_data(&lock.map, &lock.color_map);
    gl_load_instance_data(instance_data);
}
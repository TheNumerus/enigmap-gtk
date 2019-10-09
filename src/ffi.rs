use std::os::raw::c_char;
use std::ffi::CString;
use std::mem::forget;

use crate::glarea::InstanceData;

#[link(name="gl_bridge")]
extern {
    fn render();
    fn window_resized(width: i32, height: i32);
    fn load_shader(source_vert: *const c_char, source_frag: *const c_char);
    fn init_things();
    fn load_instance_data(data: *const InstanceData, len: u32);
    fn map_resized(abs_size_x: f32, abs_size_y: f32);
    fn zoom_changed(val: f32);
    fn cleanup();
}

pub fn gl_render() {
    unsafe {
        render();
    }
}

pub fn gl_window_resized(width: i32, height: i32) {
    unsafe {
        window_resized(width, height);
    }
}

pub fn gl_load_shader(vert_source: &str, frag_source: &str) {
    let c_sh_vert = CString::new(vert_source).unwrap();
    let c_sh_frag = CString::new(frag_source).unwrap();
    unsafe {
        load_shader(c_sh_vert.as_ptr(), c_sh_frag.as_ptr());
        forget(c_sh_vert);
        forget(c_sh_frag);
    }
}

pub fn gl_init_things() {
    unsafe {
        init_things();
    }
}

pub fn gl_load_instance_data(data: Vec<InstanceData>) {
    unsafe {
        load_instance_data(data.as_ptr(), data.len() as u32);
    }
}

pub fn gl_map_resized(abs_size_x: f32, abs_size_y: f32) {
    unsafe {
        map_resized(abs_size_x, abs_size_y);
    }
}

pub fn gl_zoom_changed(val: f32) {
    unsafe {
        zoom_changed(val);
    }
}

pub fn gl_cleanup() {
    unsafe {
        cleanup();
    }
}

use raw::{Env, Local};
use std::os::raw::c_void;
use std::ptr::null_mut;

use nodejs_sys as napi;

pub unsafe fn new(env: Env, out: &mut Local, size: u32) -> bool {
    let mut bytes = null_mut();
    let status = napi::napi_create_buffer(env, size as usize, &mut bytes as *mut _, out as *mut _);
    if status == napi::napi_status::napi_ok {
        // zero-initialize it. If performance is critical, JsBuffer::uninitialized can be used
        // instead.
        std::ptr::write_bytes(bytes, 0, size as usize);
        true
    } else {
        false
    }
}

pub unsafe fn uninitialized(_out: &mut Local, _size: u32) -> bool { unimplemented!() }

pub unsafe fn data(env: Env, base_out: &mut *mut c_void, obj: Local) -> usize {
    let mut size = 0;
    assert_eq!(
        napi::napi_get_buffer_info(env, obj, base_out as *mut _, &mut size as *mut _),
        napi::napi_status::napi_ok,
    );
    size
}

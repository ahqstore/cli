mod app;

use std::{
  ffi::{c_char, CStr, CString},
  sync::Mutex,
};

static ARGS: Mutex<Option<Vec<String>>> = Mutex::new(None);

/// We have a memory leak in this implementation
/// Since the leak is not too big, we're not caring about it
#[no_mangle]
pub extern "C" fn get_ver() -> *mut c_char {
  CString::new(env!("CARGO_PKG_VERSION")).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn init_args() {
  let mut args = ARGS.lock().unwrap();

  *args = Some(vec![]);
}

#[no_mangle]
pub extern "C" fn add_arg(arg: *const c_char) {
  let string = unsafe { CStr::from_ptr(arg).to_str().unwrap() };

  let string = string.to_string();

  let mut args = ARGS.lock().unwrap();

  args.as_mut().unwrap().push(string);
}

#[no_mangle]
pub extern "C" fn node_entrypoint(gh: bool) {
  let mut args = ARGS.lock().unwrap();
  let ptr_mut = args.as_mut().unwrap();

  let args = std::mem::take(ptr_mut);

  app::start(args, gh);
}

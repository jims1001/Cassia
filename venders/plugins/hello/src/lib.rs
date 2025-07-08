
#[repr(C)]
pub struct MyObject {
    pub value: i32,
}

type CallbackFn = extern "C" fn(*const MyObject);

#[unsafe(no_mangle)]
pub extern "C" fn plugin_call(obj: *const MyObject, callback: CallbackFn) {
    println!("Plugin: calling back...");
    callback(obj);
}


#[repr(C)]
pub struct MyObj2 {
    pub x: i32,
    pub y: i32,
}


#[unsafe(no_mangle)]
pub extern "C" fn create_object() -> *mut MyObj2 {
    let obj = Box::new(MyObj2 { x: 10, y: 20 });
    Box::into_raw(obj)
}


#[unsafe(no_mangle)]
pub extern "C" fn free_object(ptr: *mut MyObj2) {
    if !ptr.is_null() {
        unsafe {
            drop(Box::from_raw(ptr));
        }
    }
}



#[unsafe(no_mangle)]
pub extern "C" fn get_sum(ptr: *const MyObj2) -> i32 {
    unsafe {
        let obj = ptr.as_ref().unwrap();
        obj.x + obj.y
    }
}


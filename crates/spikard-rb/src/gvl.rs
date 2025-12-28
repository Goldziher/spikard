//! GVL helpers for Ruby integration.

use std::ffi::c_void;
use std::mem::MaybeUninit;

pub use paste;
pub use rb_sys;

#[allow(dead_code)]
pub fn with_gvl<F, R>(func: F) -> R
where
    F: FnOnce() -> R,
{
    struct WithGvlData<F, R> {
        func: Option<F>,
        result: MaybeUninit<R>,
    }

    unsafe extern "C" fn trampoline<F, R>(data: *mut c_void) -> *mut c_void
    where
        F: FnOnce() -> R,
    {
        let data = data as *mut WithGvlData<F, R>;
        let data = unsafe { &mut *data };
        let func = match data.func.take() {
            Some(func) => func,
            None => return std::ptr::null_mut(),
        };
        let result = func();
        data.result.write(result);
        std::ptr::null_mut()
    }

    let mut data = WithGvlData {
        func: Some(func),
        result: MaybeUninit::uninit(),
    };

    unsafe {
        rb_sys::rb_thread_call_with_gvl(Some(trampoline::<F, R>), &mut data as *mut _ as *mut c_void);
        data.result.assume_init()
    }
}

#[macro_export]
macro_rules! call_without_gvl {
    ($func:expr, args: ($($arg:expr, $ty:ty),+), return_type: $return_ty:ty) => {{
        $crate::gvl::paste::paste! {
            let mut result: std::mem::MaybeUninit<$return_ty> = std::mem::MaybeUninit::uninit();
            // Box the arguments to ensure they live on the heap for the entire duration of the FFI call
            let data = std::boxed::Box::new((
                ($($arg,)+),
                &mut result as *mut std::mem::MaybeUninit<$return_ty>,
            ));
            let data_ptr = std::boxed::Box::into_raw(data) as *mut std::ffi::c_void;

            unsafe extern "C" fn __decl_macro_anon_wrapper(data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
                let data = unsafe { std::boxed::Box::from_raw(data as *mut (
                    ( $($ty,)+ ),
                    *mut std::mem::MaybeUninit<$return_ty>,
                )) };
                let (( $([<__ $arg _name>],)+ ), result_output) = *data;
                let result = $func( $( [<__ $arg _name>], )+);
                unsafe { (*result_output).write(result); }
                std::ptr::null_mut()
            }

            unsafe {
                $crate::gvl::rb_sys::rb_thread_call_without_gvl(
                    Some(__decl_macro_anon_wrapper),
                    data_ptr,
                    None,
                    std::ptr::null_mut(),
                );
            }

            unsafe { result.assume_init() }
        }
    }};
}

use std::ptr::{null, null_mut};

use crate as spvc;

#[test]
pub fn linkage() {
    unsafe {
        let mut context = null_mut();
        spvc::spvc_context_create(&mut context);
        if !context.is_null() {
            // We don't want to panic with our invalid module, but this is determined at runtime so symbols are still required
            return;
        }
        let module = Vec::new();
        let mut ir = null_mut();
        spvc::spvc_context_parse_spirv(context, module.as_ptr(), module.len(), &mut ir);
        let mut compiler = null_mut();
        spvc::spvc_context_create_compiler(
            context,
            spvc::spvc_backend_SPVC_BACKEND_MSL,
            ir,
            spvc::spvc_capture_mode_SPVC_CAPTURE_MODE_TAKE_OWNERSHIP,
            &mut compiler,
        );
        let mut options = null_mut();
        spvc::spvc_compiler_create_compiler_options(compiler, &mut options);
        spvc::spvc_compiler_install_compiler_options(compiler, options);
        let mut result = null();
        spvc::spvc_compiler_compile(compiler, &mut result);
        let _result = String::from(std::ffi::CStr::from_ptr(result).to_str().unwrap()).into_bytes();
        spvc::spvc_context_destroy(context);
    }
}

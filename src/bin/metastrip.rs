// SPDX-License-Identifier: MIT
// A tool to remove unnecessary metadata other than ICC profile

use std::{
    env,
    ffi::{c_void, CString},
    ptr::null,
};

use lzfrywye_vips::{
    g_object_unref, vips_error_exit, vips_image_new_from_file, vips_image_write_to_file, vips_init,
    vips_shutdown, VipsForeignKeep_VIPS_FOREIGN_KEEP_ICC, VipsImage,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("usage: {} <input_image> <output_image>", args[0]);
        return;
    }

    unsafe {
        if vips_init(
            CString::new(args[0].clone())
                .expect("CString::new failed")
                .as_ptr(),
        ) != 0
        {
            vips_error_exit(
                CString::new("vips_init")
                    .expect("CString::new failed for error message")
                    .as_ptr(),
            );
        }

        let image: *mut VipsImage = vips_image_new_from_file(
            CString::new(args[1].clone())
                .expect("CString::new failed")
                .as_ptr(),
            null() as *const ::std::os::raw::c_void,
        );
        if image.is_null() {
            vips_error_exit(
                CString::new("vips_image_new_from_file")
                    .expect("CString::new failed for error message")
                    .as_ptr(),
            );
        }

        if vips_image_write_to_file(
            image,
            CString::new(args[2].clone())
                .expect("CString::new failed")
                .as_ptr(),
            CString::new("keep").expect("CString::new failed").as_ptr(),
            (VipsForeignKeep_VIPS_FOREIGN_KEEP_ICC) as ::std::os::raw::c_int,
            null() as *const ::std::os::raw::c_void,
        ) != 0
        {
            vips_error_exit(
                CString::new("vips_image_write_to_file")
                    .expect("CString::new failed for error message")
                    .as_ptr(),
            );
        }

        g_object_unref(image as *mut c_void);
        vips_shutdown();
    }
}

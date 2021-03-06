// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/* automatically generated by rust-bindgen */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use libc::*;

type enum_unnamed1 = c_uint;
//static STBI_default: u32 = 0_u32;
//static STBI_grey: u32 = 1_u32;
//static STBI_grey_alpha: u32 = 2_u32;
//static STBI_rgb: u32 = 3_u32;
//static STBI_rgb_alpha: u32 = 4_u32;

pub mod bindgen {
    use libc::*;

    pub type stbi_uc = c_uchar;
    #[repr(C)]
    pub struct stbi_io_callbacks {
        _read: *mut u8,
        _skip: *mut u8,
        _eof: *mut u8,
    }

#[link(name = "stb-image", kind = "static")]
extern {

pub fn stbi_get_image_format(buffer: *const stbi_uc) -> i8;

pub fn stbi_load_from_memory(buffer: *const stbi_uc, len: c_int, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *mut stbi_uc;

pub fn stbi_load(filename: *const c_char, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *mut stbi_uc;

pub fn stbi_load_from_file(f: *mut FILE, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *mut stbi_uc;

pub fn stbi_load_from_callbacks(clbk: *const stbi_io_callbacks, user: *mut c_void, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *mut stbi_uc;

pub fn stbi_loadf_from_memory(buffer: *const stbi_uc, len: c_int, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *mut c_float;

pub fn stbi_loadf(filename: *const c_char, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *mut c_float;

pub fn stbi_loadf_from_file(f: *mut FILE, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *mut c_float;

pub fn stbi_loadf_from_callbacks(clbk: *const stbi_io_callbacks, user: *mut c_void, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *mut c_float;

pub fn stbi_hdr_to_ldr_gamma(gamma: c_float);

pub fn stbi_hdr_to_ldr_scale(scale: c_float);

pub fn stbi_ldr_to_hdr_gamma(gamma: c_float);

pub fn stbi_ldr_to_hdr_scale(scale: c_float);

pub fn stbi_is_hdr_from_callbacks(clbk: *const stbi_io_callbacks, user: *mut c_void) -> c_int;

pub fn stbi_is_hdr_from_memory(buffer: *const stbi_uc, len: c_int) -> c_int;

pub fn stbi_is_hdr(filename: *const c_char) -> c_int;

pub fn stbi_is_hdr_from_file(f: *mut FILE) -> c_int;

pub fn stbi_failure_reason() -> *const c_char;

pub fn stbi_image_free(retval_from_stbi_load: *mut c_void);

pub fn stbi_info_from_memory(buffer: *const stbi_uc, len: c_int, x: *mut c_int, y: *mut c_int, comp: *mut c_int) -> c_int;

pub fn stbi_info_from_callbacks(clbk: *const stbi_io_callbacks, user: *mut c_void, x: *mut c_int, y: *mut c_int, comp: *mut c_int) -> c_int;

pub fn stbi_info(filename: *const c_char, x: *mut c_int, y: *mut c_int, comp: *mut c_int) -> c_int;

pub fn stbi_info_from_file(f: *mut FILE, x: *mut c_int, y: *mut c_int, comp: *mut c_int) -> c_int;

pub fn stbi_set_unpremultiply_on_load(flag_true_if_should_unpremultiply: c_int);

pub fn stbi_convert_iphone_png_to_rgb(flag_true_if_should_convert: c_int);

pub fn stbi_zlib_decode_malloc_guesssize(buffer: *const c_char, len: c_int, initial_size: c_int, outlen: *mut c_int) -> *mut c_char;

pub fn stbi_zlib_decode_malloc(buffer: *const c_char, len: c_int, outlen: *mut c_int) -> *mut c_char;

pub fn stbi_zlib_decode_buffer(obuffer: *const c_char, olen: c_int, ibuffer: *const c_char, ilen: c_int) -> c_int;

pub fn stbi_zlib_decode_noheader_malloc(buffer: *const c_char, len: c_int, outlen: *mut c_int) -> *mut c_char;

pub fn stbi_zlib_decode_noheader_buffer(obuffer: *mut c_char, olen: c_int, ibuffer: *const c_char, ilen: c_int) -> c_int;

  }
}

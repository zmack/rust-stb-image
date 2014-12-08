// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use stb_image::bindgen::*;

use libc;
use libc::{c_void, c_int};
use std::slice::raw::mut_buf_as_slice;

#[deriving(PartialEq, Show)]
pub enum ImageFormat {
    JPEG = 0x00,
    PNG = 0x01,
    BMP = 0x02,
    GIF = 0x03,
    PSD = 0x04,
    PIC = 0x05,
    Unknown = 0xFF
}

pub struct Image<T> {
    pub width   : uint,
    pub height  : uint,
    pub depth   : uint,
    pub data    : Vec<T>,
}

pub fn new_image<T>(width: uint, height: uint, depth: uint, data: Vec<T>) -> Image<T> {
    Image::<T> {
        width   : width,
        height  : height,
        depth   : depth,
        data    : data,
    }
}

pub enum LoadResult {
    Error(String),
    ImageU8(Image<u8>),
    ImageF32(Image<f32>),
}

pub fn load(path: &Path) -> LoadResult {
    let force_depth = 0;
    load_with_depth(path, force_depth, false)
}

impl ImageFormat {
    fn new_from_i8(n: i8) -> ImageFormat {
        match n {
            0 => ImageFormat::JPEG,
            1 => ImageFormat::PNG,
            2 => ImageFormat::BMP,
            3 => ImageFormat::GIF,
            4 => ImageFormat::PSD,
            5 => ImageFormat::PIC,
            _ => ImageFormat::Unknown
        }
    }
}


fn load_internal<T: Clone>(buf: *mut T, w: c_int, h: c_int, d: c_int) -> Image<T> {
    unsafe {
        // FIXME: Shouldn't copy; instead we should use a sendable resource. They
        // aren't particularly safe yet though.
        let data = mut_buf_as_slice(buf, (w * h * d) as uint, |s| { s.to_vec() });
        libc::free(buf as *mut c_void);
        Image::<T>{
            width   : w as uint,
            height  : h as uint,
            depth   : d as uint,
            data    : data}
    }
}

pub fn load_with_depth(path: &Path, force_depth: uint, convert_hdr: bool) -> LoadResult {
    unsafe {
        let mut width = 0 as c_int;
        let mut height = 0 as c_int;
        let mut depth = 0 as c_int;
        let path_as_str = match path.as_str() {
            Some(s) => s,
            None => return LoadResult::Error("path is not valid utf8".to_string()),
        };
        path_as_str.with_c_str(|bytes| {
            if !convert_hdr && stbi_is_hdr(bytes)!=0   {
                let buffer = stbi_loadf(bytes,
                                        &mut width,
                                        &mut height,
                                        &mut depth,
                                        force_depth as c_int);
                if buffer.is_null() {
                    LoadResult::Error("stbi_loadf failed".to_string())
                } else {
                    LoadResult::ImageF32(load_internal(buffer, width, height, depth))
                }
            } else {
                let buffer = stbi_load(bytes,
                                       &mut width,
                                       &mut height,
                                       &mut depth,
                                       force_depth as c_int);
                if buffer.is_null() {
                    LoadResult::Error("stbi_load failed".to_string())
                } else {
                    LoadResult::ImageU8(load_internal(buffer, width, height, depth))
                }
            }
        })
    }
}

pub fn get_image_format(buffer: &[u8]) -> ImageFormat {
    unsafe {
        ImageFormat::new_from_i8(stbi_get_image_format(buffer.as_ptr()))
    }
}

pub fn load_from_memory(buffer: &[u8]) -> LoadResult {
    let force_depth = 0;
    load_from_memory_with_depth(buffer, force_depth, false)
}

pub fn load_from_memory_with_depth(buffer: &[u8], force_depth: uint, convert_hdr:bool) -> LoadResult {
    unsafe {
        let mut width = 0 as c_int;
        let mut height = 0 as c_int;
        let mut depth = 0 as c_int;
        if !convert_hdr && stbi_is_hdr_from_memory(buffer.as_ptr(), buffer.len() as c_int) != 0 {
            let buffer = stbi_loadf_from_memory(buffer.as_ptr(),
                                                buffer.len() as c_int,
                                                &mut width,
                                                &mut height,
                                                &mut depth,
                                                force_depth as c_int);
            if buffer.is_null() {
                LoadResult::Error("stbi_loadf_from_memory failed".to_string())
            } else {
                let actual_depth = if force_depth != 0 { force_depth as c_int } else { depth };
                LoadResult::ImageF32(load_internal(buffer, width, height, actual_depth))
            }
        } else {
            let buffer = stbi_load_from_memory(buffer.as_ptr(),
                                               buffer.len() as c_int,
                                               &mut width,
                                               &mut height,
                                               &mut depth,
                                               force_depth as c_int);
            if buffer.is_null() {
                LoadResult::Error("stbi_load_from_memory failed".to_string())
            } else {
                let actual_depth = if force_depth != 0 { force_depth as c_int } else { depth };
                LoadResult::ImageU8(load_internal(buffer, width, height, actual_depth))
            }
        }
    }
}

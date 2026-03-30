

#![allow(dead_code)]

use macroquad::prelude::*;

static TEST: &str = "#3d3d3f";


pub const QBLACK: Color = color_u8!(0x00, 0x00, 0x00, 255); // #000000
pub const SBLACK: Color = color_u8!(0x00, 0x00, 0x00, 255); // #000000
pub const MBLACK: Color = color_u8!(0x00, 0x00, 0x00, 255); // #000000
pub const LBLACK: Color = color_u8!(0x00, 0x00, 0x00, 255); // #000000
pub const XBLACK: Color = color_u8!(0x00, 0x00, 0x00, 255); // #000000 

pub const QGRAY: Color = color_u8!(0x3d, 0x3d, 0x3f, 255); // #3d3d3f
pub const SGRAY: Color = color_u8!(0x47, 0x47, 0x4a, 255); // #47474a
pub const MGRAY: Color = color_u8!(0x47, 0x47, 0x4a, 255); // #47474a
pub const LGRAY: Color = color_u8!(0x64, 0x64, 0x68, 255); // #646468
pub const XGRAY: Color = color_u8!(0x83, 0x83, 0x87, 255); // #838387

pub const QWHITE: Color = color_u8!(0xd0, 0xd0, 0xd0, 255); // #d0d0d0
pub const SWHITE: Color = color_u8!(0xe6, 0xe6, 0xe6, 255); // #e6e6e6
pub const MWHITE: Color = color_u8!(0xe6, 0xe6, 0xe6, 255); // #e6e6e6
pub const LWHITE: Color = color_u8!(0xee, 0xee, 0xee, 255); // #eeeeee
pub const XWHITE: Color = color_u8!(0xf4, 0xf4, 0xf4, 255); // #f4f4f4

pub const QBLUE: Color = color_u8!(0x00, 0x5c, 0xff, 255); // #005cff
pub const SBLUE: Color = color_u8!(0x00, 0x5c, 0xff, 255); // #005cff
pub const MBLUE: Color = color_u8!(0x00, 0x5c, 0xff, 255); // #005cff
pub const LBLUE: Color = color_u8!(0x53, 0xa6, 0xff, 255); // #53a6ff
pub const XBLUE: Color = color_u8!(0x6d, 0xbb, 0xff, 255); // #6dbbff


pub const RING_BLUE: Color = color_u8!(0x09, 0x37, 0xa2, 255); // #0937a2
pub const OVERLOADED: Color = color_u8!(0xff, 0x00, 0x00, 255); // #ff0000
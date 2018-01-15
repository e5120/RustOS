#![feature(lang_items)]
#![feature(start)]
//#![no_main]
#![feature(no_std)]
#![no_std]
#![feature(asm)]

//extern crate assembly;
//use assembly::*;

pub struct BootInfo {
    cyls: char,
    leds: char,
    vmode: char,
    reserve: char,
    screen_x: u16,
    screen_y: u16,
    vram: u32,
}

pub trait ScreenDrawer {
    fn new() -> Self;
    fn dot(&self, x: u16, y: u16, p: &Palette);
    fn rect(&self, x: u16, y: u16, w: u16, h: u16, p: &Palette);
    fn width(&self) -> u16;
    fn height(&self) -> u16;
}

impl ScreenDrawer for BootInfo {
    fn new() -> Self {
        BootInfo {
            cyls:     unsafe{ *(0x00000ff0 as *const char)},
            leds:     unsafe{ *(0x00000ff1 as *const char)},
            vmode:    unsafe{ *(0x00000ff2 as *const char)},
            reserve:  unsafe{ *(0x00000ff3 as *const char)},
            screen_x: unsafe{ *(0x00000ff4 as *const u16)},
            screen_y: unsafe{ *(0x00000ff6 as *const u16)},
            vram:     unsafe{ *(0x00000ff8 as *mut u32)},
        }
    }

    fn dot(&self, x: u16, y: u16, p: &Palette) {
        if x < self.screen_x && y < self.screen_y {
            let index = (self.screen_x * y + x) as u32;
            let vram = (self.vram  + index) as *mut u8;
            unsafe {
                *vram = p.palette_no;
            }
        }
    }

    fn rect(&self, x: u16, y: u16, w: u16, h: u16, p: &Palette) {
        for i in y..y + h {
            for j in x..x + w {
                self.dot(i, j, p);
            }
        }
    }

    fn width(&self) -> u16 {
        self.screen_x
    }

    fn height(&self) -> u16 {
        self.screen_y
    }
}

pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

pub trait RGBIO {
    fn set_rgb(&self);
}

impl RGBIO for RGB {
    fn set_rgb(&self) {
        unsafe {
            io_out8(0x03c9, (self.r as i32) >> 2);
            io_out8(0x03c9, (self.g as i32) >> 2);
            io_out8(0x03c9, (self.b as i32) >> 2);
        }
    }
}

pub struct Palette {
    palette_no: u8,
}

impl Palette {
    fn set_palette_no(&self) {
        unsafe {
            io_out8(0x03c8, self.palette_no as i32);
        }
    }
}

pub trait RGBDef {
    fn b_black() -> Self;
    fn b_red() -> Self;
    fn b_green() -> Self;
    fn b_yellow() -> Self;
    fn b_blue() -> Self;
    fn b_purple() -> Self;
    fn b_light_blue() -> Self;
    fn white() -> Self;
    fn b_gray() -> Self;
    fn d_red() -> Self;
    fn d_green() -> Self;
    fn d_yellow() -> Self;
    fn d_blue() -> Self;
    fn d_purple() -> Self;
    fn d_light_blue() -> Self;
    fn d_gray() -> Self;
}

impl RGBDef for RGB {
    fn b_black() -> RGB {
        RGB {r: 0x00, g: 0x00, b: 0x00}
    }
    fn b_red() -> RGB {
        RGB {r: 0xff, g: 0x00, b: 0x00}
    }
    fn b_green() -> RGB {
        RGB {r: 0x00, g: 0xff, b: 0x00}
    }
    fn b_yellow() -> RGB {
        RGB {r: 0xff, g: 0xff, b: 0x00}
    }
    fn b_blue() -> RGB {
        RGB {r: 0x00, g: 0x00, b: 0xff}
    }
    fn b_purple() -> RGB {
        RGB {r: 0xff, g: 0x00, b: 0xff}
    }
    fn b_light_blue() -> RGB {
        RGB {r: 0x00, g: 0xff, b: 0xff}
    }
    fn white() -> RGB {
        RGB {r: 0xff, g: 0xff, b: 0xff}
    }
    fn b_gray() -> RGB {
        RGB {r: 0xc6, g: 0xc6, b: 0xc6}
    }
    fn d_red() -> RGB {
        RGB {r: 0x84, g: 0x00, b: 0x00}
    }
    fn d_green() -> RGB {
        RGB {r: 0x00, g: 0x84, b: 0x00}
    }
    fn d_yellow() -> RGB {
        RGB {r: 0x84, g: 0x84, b: 0x00}
    }
    fn d_blue() -> RGB {
        RGB {r: 0x00, g: 0x00, b: 0x84}
    }
    fn d_purple() -> RGB {
        RGB {r: 0x84, g: 0x00, b: 0x84}
    }
    fn d_light_blue() -> RGB {
        RGB {r: 0x00, g: 0x84, b: 0x84}
    }
    fn d_gray() -> RGB {
        RGB {r: 0x84, g: 0x84, b: 0x84}
    }
}

impl RGBDef for Palette {
    fn b_black() -> Palette {
        Palette {palette_no: 0}
    }
    fn b_red() -> Palette {
        Palette {palette_no: 1}
    }
    fn b_green() -> Palette {
        Palette {palette_no: 2}
    }
    fn b_yellow() -> Palette {
        Palette {palette_no: 3}
    }
    fn b_blue() -> Palette {
        Palette {palette_no: 4}
    }
    fn b_purple() -> Palette {
        Palette {palette_no: 5}
    }
    fn b_light_blue() -> Palette {
        Palette {palette_no: 6}
    }
    fn white() -> Palette {
        Palette {palette_no: 7}
    }
    fn b_gray() -> Palette {
        Palette {palette_no: 8}
    }
    fn d_red() -> Palette {
        Palette {palette_no: 9}
    }
    fn d_green() -> Palette {
        Palette {palette_no: 10}
    }
    fn d_yellow() -> Palette {
        Palette {palette_no: 11}
    }
    fn d_blue() -> Palette {
        Palette {palette_no: 12}
    }
    fn d_purple() -> Palette {
        Palette {palette_no: 13}
    }
    fn d_light_blue() -> Palette {
        Palette {palette_no: 14}
    }
    fn d_gray() -> Palette {
        Palette {palette_no: 15}
    }
}

pub fn palette_init(){
    let eflag = load_eflags();
    cli();

    Palette::b_black().set_palette_no();
    RGB::b_black().set_rgb();

    Palette::b_red().set_palette_no();
    RGB::b_red().set_rgb();

    Palette::b_green().set_palette_no();
    RGB::b_green().set_rgb();

    Palette::b_yellow().set_palette_no();
    RGB::b_yellow().set_rgb();

    Palette::b_blue().set_palette_no();
    RGB::b_blue().set_rgb();

    Palette::b_purple().set_palette_no();
    RGB::b_purple().set_rgb();

    Palette::b_light_blue().set_palette_no();
    RGB::b_light_blue().set_rgb();

    Palette::white().set_palette_no();
    RGB::white().set_rgb();

    Palette::b_gray().set_palette_no();
    RGB::b_gray().set_rgb();

    Palette::d_red().set_palette_no();
    RGB::d_red().set_rgb();

    Palette::d_green().set_palette_no();
    RGB::d_green().set_rgb();

    Palette::d_yellow().set_palette_no();
    RGB::d_yellow().set_rgb();

    Palette::d_blue().set_palette_no();
    RGB::d_blue().set_rgb();

    Palette::d_purple().set_palette_no();
    RGB::d_purple().set_rgb();

    Palette::d_light_blue().set_palette_no();
    RGB::d_light_blue().set_rgb();

    Palette::d_gray().set_palette_no();
    RGB::d_gray().set_rgb();

    store_eflags(eflag);
}


#[no_mangle]
#[start]
pub extern fn os_main() {
    palette_init();
    let drawer = BootInfo::new();
    let p = Palette::b_green();
    drawer.rect(50, 50, 100, 100, &(Palette::b_green()));
    drawer.rect(100, 100, 150, 160, &(Palette::white()));

    loop {
        hlt()
    }
}


#[no_mangle]
pub fn hlt() {
    unsafe {
        asm!("hlt");
    }
}

fn cli() {
    unsafe {
        asm!("cli" :::: "intel");
    }
}

fn load_eflags() -> u32 {
    let eflags: u32;
    unsafe {
        asm!("pushfd
              pop $0"
             : "=r"(eflags)
             :
             :
             : "intel");
    }
    return eflags;
}

fn store_eflags(eflags: u32) {
    unsafe {
        asm!("push $0
              popfd"
             :
             : "r"(eflags)
             :
             : "intel");
    }
}

extern "C" {
    #[cfg(any(target_arch = "x86"))]
    pub fn io_out8(port: i32, data: i32);
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! { loop {} }



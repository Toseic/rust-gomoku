#![allow(non_camel_case_types)]


use std::os::unix::io::AsRawFd;


type c_int = i32;
#[cfg(target_os = "macos")]
type c_ulong = u64;
#[cfg(not(target_os = "macos"))]
type c_uint = u32;
type c_uchar = u8;

#[cfg(target_os = "macos")]
pub type tcflag_t = c_ulong;
#[cfg(not(target_os = "macos"))]
pub type tcflag_t = c_uint;
type cc_t = c_uchar;

const NCCS: usize = 32;
const ECHO: tcflag_t = 0o000010;
#[cfg(target_os = "macos")]
const ICANON: tcflag_t = 0x0000100;
#[cfg(not(target_os = "macos"))]
const ICANON: tcflag_t = 0o0000002;

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

pub type Dimensions = Vec2<usize>;
pub type Position = Vec2<usize>;

#[repr(C)]
struct Termios {
    c_iflag: tcflag_t,  // input modes
    c_oflag: tcflag_t,  // output modes
    c_cflag: tcflag_t,  // control modes
    c_lflag: tcflag_t,  // local modes
    c_cc: [cc_t; NCCS], // special characters
}

#[link(name = "c")]
extern "C" {
    fn ioctl(fildes: c_int, request: c_int, ...) -> c_int;
    fn tcgetattr(fd: c_int, termios_p: *mut Termios) -> c_int;
    fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const Termios) -> c_int;
}



pub fn reset() {
    print!("\x1bc");
}

pub fn hide_cursor() {
    print!("\x1b\x5b?25l");
}

pub fn set_mode(enable: bool) {
    let stdin_fd = std::io::stdin().as_raw_fd();
    let mut termios = unsafe {
        let mut termios = std::mem::MaybeUninit::<Termios>::uninit();
        let res = tcgetattr(stdin_fd, termios.as_mut_ptr());
        if res != 0 {
            panic!("tcgetattr failed.");
        }
        termios.assume_init()
    };

    if enable {
        termios.c_lflag |= ECHO | ICANON;
    } else {
        termios.c_lflag &= !(ECHO | ICANON);
    }
    unsafe {
        let res = tcsetattr(stdin_fd, 0, &termios);
        if res != 0 {
            panic!("tcsetattr failed.");
        }
    }
}


pub fn get_dimensions() -> Result<Dimensions, &'static str> {
    use std::mem;

    #[cfg(target_os = "macos")]
    const TIOCGWINSZ: i32 = 0x40087468;
    #[cfg(not(target_os = "macos"))]
    const TIOCGWINSZ: i32 = 0x5413;

    struct Winsize {
        ws_row: u16,
        ws_col: u16,
        _ws_xpixel: u16, // unused
        _ws_ypixel: u16, // unused
    }

    let stdin_fd = std::io::stdin().as_raw_fd();

    let winsize = unsafe {
        let mut winsize = mem::MaybeUninit::<Winsize>::uninit();
        let res = ioctl(stdin_fd, TIOCGWINSZ, winsize.as_mut_ptr());
        if res == -1 {
            return Err("Could not get terminal dimensions.");
        }
        winsize.assume_init()
    };

    Ok(Dimensions {
        x: winsize.ws_col as usize,
        y: winsize.ws_row as usize,
    })
}
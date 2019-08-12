#![feature(core_intrinsics)]
///! Controlls GPIO in beaglebone and pocket beagle through mmap. Much faster than going through
/// the sysfs interface. Validated to work > 20kHz, at which point the initial overshot cannot
/// stablize in half cycle.
use std::os::unix::io::AsRawFd;
use std::fs::OpenOptions;
use std::io::Result;

const PAGE_SIZE: usize = 0x1000;
const GPIO_SETDATAOUT: isize = 0x194;
const GPIO_CLEARDATAOUT: isize = 0x190;

/// am335x has 4 gpio banks
#[repr(usize)]
pub enum GPIOAddrs {
    GPIO0 = 0x44E0_7000,
    GPIO1 = 0x4804_C000,
    GPIO2 = 0x481A_C000,
    GPIO3 = 0x481A_E000,
}


#[derive(Clone, Copy)]
pub struct GPIOController {
    #[allow(dead_code)]
    head: *mut libc::c_void,
    data_out: *mut u32,
    data_clr: *mut u32,
}

/// Create a gpio controller for one of the 4 banks. Once successfully acquired the controller, it
/// can be turned on() and off(). Turning on() or off() when the pin is already on or off does not
/// generate a feedback. The pins triggered must be configured to gpio out using the command line
/// util "config-pin" before hand.
impl GPIOController {
    pub fn new(bank_addr: GPIOAddrs) -> Result<Self> {
        let mem = OpenOptions::new().read(true).write(true).open("/dev/mem")?;
        let head;
        let data_out;
        let data_clr;
        unsafe {
            head = libc::mmap(0 as *mut libc::c_void, PAGE_SIZE, libc::PROT_READ | libc::PROT_WRITE,
                              libc::MAP_SHARED, mem.as_raw_fd(), bank_addr as libc::off_t);
            data_out = head.offset(GPIO_SETDATAOUT) as *mut u32;
            data_clr = head.offset(GPIO_CLEARDATAOUT) as *mut u32;
        }
        return Ok(GPIOController{head, data_out, data_clr})
    }
    pub fn on(&self, pin_no: u32) {
        unsafe { *(self.data_out) = 1 << pin_no; }
    }
    pub fn off(&self, pin_no: u32) {
        unsafe { *(self.data_clr) = 1 << pin_no; }
    }
}

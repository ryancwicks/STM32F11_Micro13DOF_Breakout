#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger


use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use crate::hal::{prelude::*, stm32, i2c::I2c, };

use stm32f4xx_hal::otg_fs::{UsbBus, USB};
use usb_device::prelude::*;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

#[entry]
fn main() -> ! {

    let _core_peripherals = cortex_m::peripheral::Peripherals::take().unwrap();
    let peripherals = stm32::Peripherals::take().unwrap();
    
    //Initial setup code
    peripherals.RCC.apb2enr.write(|w| w.syscfgen().enabled());
  

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = peripherals.RCC.constrain();
    let clocks = rcc
                    .cfgr
                    .use_hse(8.mhz())
                    .sysclk(48.mhz())
                    .require_pll48clk()
                    .freeze();
       
    //Set up the GPIO for I2C
    let gpiob = peripherals.GPIOB.split();
    let i2c_scl = gpiob.pb6.into_alternate_af4().set_open_drain(); //check the data sheet for the alternate function (pg 48)
    let i2c_sda = gpiob.pb7.into_alternate_af4().set_open_drain();
    
    let i2c = I2c::i2c1(peripherals.I2C1, (i2c_scl, i2c_sda), 400.khz(), clocks);

    //Set up the GPIO for the USB serial
    let gpioa = peripherals.GPIOA.split();
    
    let usb = USB {
        usb_global: peripherals.OTG_FS_GLOBAL,
        usb_device: peripherals.OTG_FS_DEVICE,
        usb_pwrclk: peripherals.OTG_FS_PWRCLK,
        pin_dm: gpioa.pa11.into_alternate_af10(),
        pin_dp: gpioa.pa12.into_alternate_af10(),
    };
    
    let usb_bus = UsbBus::new(usb, unsafe { &mut EP_MEMORY });

    let mut serial = usbd_serial::SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("RW company")
        .product("IMU Serial Port")
        .serial_number("TEST")
        .device_class(usbd_serial::USB_CLASS_CDC)
        .build();

    loop {
        //Looped code goes here.
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

    }
    //Hopefully you never get here

}

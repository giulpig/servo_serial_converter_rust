#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;
use arduino_hal::simple_pwm::{Prescaler, IntoPwmPin,Timer0Pwm};
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use arduino_hal::usart::Usart;
use arduino_hal::prelude::_embedded_hal_serial_Read;
use atmega_hal::port::{PD0, PD1};
use atmega_hal::port::Pin;
use atmega_hal::pac::USART0;
use atmega_hal::port::mode::Output;
use atmega_hal::port::mode::Input;

use atmega_hal::usart::Event;

static mut SERIAL: Option<Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>>> = None;
static mut LAST_RX_CHAR: Option<u8> = None;

#[avr_device::interrupt(atmega328p)]
fn USART_RX() {
    unsafe { 
        match &mut SERIAL {
            Some(serial) => {
                LAST_RX_CHAR = Some(serial.read().unwrap());
            },
            None => ()
        }
    };
    //let current = REVERSED.load(Ordering::SeqCst);
    //REVERSED.store(!current, Ordering::SeqCst);
    //lufmt::uwriteln!(&mut serial, "res").unwrap_infallible();
}

unsafe fn get_serial() -> &'static mut Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>> {
    unsafe { 
        match &mut SERIAL {
            Some(serial) => serial,
            None => panic!()
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    serial.listen(Event::RxComplete);

    unsafe { SERIAL = Some(serial); }
    unsafe { LAST_RX_CHAR = None };

    //let (serial_reader, serial_writer) = serial.split();

    // Digital pin 13 is also connected to an onboard LED marked "L"
    //let mut led = pins.d13.into_output();
    //led.set_high();

    // initialize pwm timer
    let mut timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale1024);

    let mut d5 = pins.d5.into_output().into_pwm(&mut timer0);
    //let mut d6 = pins.d6.into_output().into_pwm(&mut timer0);

    unsafe { avr_device::interrupt::enable() };

    d5.enable();

    let mut duty:u8 = 34;

    loop {

        // match serial.read() {
        //     Ok(c) => ufmt::uwriteln!(&mut serial, "res: {}", c).unwrap_infallible(),
        //     Err(err) => ufmt::uwriteln!(&mut serial, "err").unwrap_infallible()
        // };
        
        unsafe{
            match LAST_RX_CHAR{
                // TODO: disable interrupts to not read the same thing twice
                Some(ch) => {ufmt::uwriteln!(get_serial(), "duty: {}", ch).unwrap_infallible()},
                None => ufmt::uwriteln!(get_serial(), "none").unwrap_infallible(),
            }
            LAST_RX_CHAR = None;
        }

        arduino_hal::delay_ms(200);
        //for d in 10..=40 {
        //    d5.set_duty(d);
        //    arduino_hal::delay_ms(100);
        //}
        //led.toggle();
        //arduino_hal::delay_ms(100);
        //d5.set_duty(40);
        //arduino_hal::delay_ms(2000);
        //ufmt::uwriteln!(&mut serial, "duty: {}", duty).unwrap_infallible();
        //duty = (duty + 1) % 45;
    }
}

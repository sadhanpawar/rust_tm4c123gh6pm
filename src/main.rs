#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use tm4c123x_hal::{
    gpio::GpioExt,
    sysctl::SysctlExt,
    //sysctl::*,
};

/* Include this if needed */
mod baremetal;

static IS_BAREMETAL: bool = true;

#[entry]
fn main() -> ! {
    // Step 1: Access peripherals
    let peripherals = tm4c123x_hal::Peripherals::take().unwrap();

    // Step 2: Configure the system clock
    let mut sysctl = peripherals.SYSCTL.constrain();

    sysctl.clock_setup.oscillator = tm4c123x_hal::sysctl::Oscillator::Main(
        tm4c123x_hal::sysctl::CrystalFrequency::_16mhz,
        tm4c123x_hal::sysctl::SystemClock::UsePll(
            tm4c123x_hal::sysctl::PllOutputFrequency::_40_00mhz,
        ),
    );

    let _clocks = sysctl.clock_setup.freeze();

    // Step 3: Enable GPIO Port F
    let portf = peripherals.GPIO_PORTF.split(&sysctl.power_control);

    // Step 4: Configure PF1 (Red LED) as push-pull output
    let mut led1 = portf.pf1.into_push_pull_output();
    let mut led2 = portf.pf2.into_push_pull_output();
    let mut led3 = portf.pf3.into_push_pull_output();

    // Step 5: Blink the Red LED
    loop {
        if !IS_BAREMETAL {
            led1.set_high().unwrap(); // Turn on LED
            cortex_m::asm::delay(8_000_000); // Delay
            led1.set_low().unwrap(); // Turn off LED
            cortex_m::asm::delay(8_000_000); // Delay
        } else {
            //baremetal::glow_leds();
        }

        led2.set_high().unwrap(); // Turn on LED
        cortex_m::asm::delay(8_000_000); // Delay
        led2.set_low().unwrap(); // Turn off LED
        cortex_m::asm::delay(8_000_000); // Delay

        led3.set_high().unwrap(); // Turn on LED
        cortex_m::asm::delay(8_000_000); // Delay
        led3.set_low().unwrap(); // Turn off LED
        cortex_m::asm::delay(8_000_000); // Delay
    }
}

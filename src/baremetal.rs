pub fn glow_leds() -> ! {
    const SYSCTL_RCC: *mut u32 = 0x400FE060 as *mut u32;
    const SYSCTL_RCC_XTAL_16MHZ: u32 = 0x540;
    const SYSCTL_RCC_USESYSDIV: u32 = 1 << 22;
    const SYSCTL_RCC_SYSDIV_40MHZ: u32 = 4 << 23;

    const SYSCTL_RCGCGPIO: *mut u32 = 0x400FE608 as *mut u32;
    const GPIO_PORTF_DIR: *mut u32 = 0x40025400 as *mut u32;
    const GPIO_PORTF_DEN: *mut u32 = 0x4002551C as *mut u32;
    const GPIO_PORTF_DATA: *mut u32 = 0x400253FC as *mut u32;

    // Step 1: Configure the system clock to 40 MHz
    unsafe {
        // Configure main oscillator
        SYSCTL_RCC
            .write_volatile(SYSCTL_RCC_XTAL_16MHZ | SYSCTL_RCC_USESYSDIV | SYSCTL_RCC_SYSDIV_40MHZ);
    }

    // Step 2: Enable GPIO Port F
    unsafe {
        SYSCTL_RCGCGPIO.write_volatile(0x20); // Enable clock for Port F
                                              //while SYSCTL_RCGCGPIO.read_volatile() & 0x20 == 0 {} // Wait until ready
    }

    cortex_m::asm::delay(8_000_000);

    // Step 3: Configure PF1 (Red LED) as output
    unsafe {
        GPIO_PORTF_DIR.write_volatile(0x02); // Set PF1 as output
        GPIO_PORTF_DEN.write_volatile(0x02); // Enable digital functionality for PF1
    }

    // Step 4: Blink the Red LED
    loop {
        // Turn on PF1
        unsafe {
            GPIO_PORTF_DATA.write_volatile(0x02);
        }
        cortex_m::asm::delay(8_000_000);

        // Turn off PF1
        unsafe {
            GPIO_PORTF_DATA.write_volatile(0x00);
        }
        cortex_m::asm::delay(8_000_000);
    }
}

#![deny(clippy::all)]
#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32l4xx_hal::delay::Delay;
use stm32l4xx_hal::prelude::*;

#[entry]
fn main() -> ! {
    let core = cortex_m::Peripherals::take().unwrap();
    let device = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();

    let clocks = rcc.cfgr.hclk(8.mhz()).freeze(&mut flash.acr);

    let mut gpiob = device.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpiob
        .pb3
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut timer = Delay::new(core.SYST, clocks);

    loop {
        timer.delay_ms(1000 as u32);
        led.set_high();
        timer.delay_ms(1000 as u32);
        led.set_low();
    }
}

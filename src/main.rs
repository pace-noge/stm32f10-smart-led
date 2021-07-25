#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_default;

use stm32f1xx_hal as hal;
use ws2812_spi as ws2812;

use crate::hal::delay::Delay;
use crate::hal::pac;
use crate::hal::prelude::*;
use crate::hal::spi::Spi;
use crate::ws2812::Ws2812;
use cortex_m::peripheral::Peripherals;

use smart_leds::{SmartLedsWrite, RGB8, brightness};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    rtt_init_default!();

    if let (Some(dp), Some(cp)) = (pac::Peripherals::take(), Peripherals::take()) {
        // Take ownership over the raw flash and rcc devices and convert them into the corresponding
        // HAL structs
        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();

        // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
        // `clocks`
        let clocks = rcc
            .cfgr
            .sysclk(48.mhz())
            .pclk1(24.mhz())
            .freeze(&mut flash.acr);

        // Acquire the GPIOA peripheral
        let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

        let pins = (
            gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh),
            gpiob.pb14.into_floating_input(&mut gpiob.crh),
            gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh),
        );

        let mut delay = Delay::new(cp.SYST, clocks);

        let spi = Spi::spi2(dp.SPI2, pins,ws2812::MODE, 3.mhz(), clocks, &mut rcc.apb1);

        const NUM_LEDS: usize = 6;

        let mut data: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
        let empty: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
        let mut ws = Ws2812::new(spi);
        loop {
            // data[0] = RGB8 {
            //     r: 0,
            //     g: 0,
            //     b: 0x10,
            // };
            // data[1] = RGB8 {
            //     r: 0,
            //     g: 0,
            //     b: 0x10,
            // };
            // data[2] = RGB8 {
            //     r: 0,
            //     g: 0,
            //     b: 0x10,
            // };
            // ws.write(data.iter().cloned()).unwrap();
            // delay.delay_ms(1000 as u16);
            // ws.write(empty.iter().cloned()).unwrap();
            // delay.delay_ms(1000 as u16);


            // rainbow

            for j in 0..(256 * 5) {
                for i in 0..NUM_LEDS {
                    data[i] = rainbow((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
                }
                ws.write(brightness(data.iter().cloned(), 32)).unwrap();
                delay.delay_ms(5u8);
            }

        }
    }
    loop {
        continue;
    }
}



fn rainbow(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }

    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }

    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}
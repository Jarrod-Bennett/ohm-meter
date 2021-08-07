#![no_main]
#![no_std]

use ohm_meter as _; // global logger + panicking-behavior + memory layout

use stm32l4xx_hal::{self as hal, pac, prelude::*};
use hal::{adc::ADC, delay::Delay};

#[cortex_m_rt::entry]
fn main() -> ! {

    defmt::info!("Starting up!");

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut led = gpioa.pa11.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let mut adc1_pin = gpioa.pa0.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);
    let mut adc1 = ADC::new(
        dp.ADC1,
        dp.ADC_COMMON,
        &mut rcc.ahb2,
        &mut rcc.ccipr,
        &mut delay
    );

    defmt::info!("Startup completed successfully!");

    loop {

        delay.delay_ms(1000_u32);
        led.set_high().ok();

        let adc_val = adc1.read(&mut adc1_pin).unwrap();
        let resistance = adc_to_ohms(adc_val);
        defmt::info!("Read {} Ohms", resistance);

        delay.delay_ms(1000_u32);
        led.set_low().ok();

        let adc_val = adc1.read(&mut adc1_pin).unwrap();
        let resistance = adc_to_ohms(adc_val);
        defmt::info!("Read {} Ohms", resistance);
    }
}

fn adc_to_ohms(adc: u16) -> u32 {
    const ADC_MAX_VAL: f32 = 4096.0;
    const ADC_TOP_VOLTS: f32 = 5.0;
    const REF_RESISTOR_VAL: f32 = 1_000.0;
    let adc_float = adc as f32;
    let adc_volts = (adc_float / ADC_MAX_VAL) * ADC_TOP_VOLTS;
    ((ADC_TOP_VOLTS - adc_volts) * REF_RESISTOR_VAL / adc_volts) as u32
}

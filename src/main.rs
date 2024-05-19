#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_persist as _};

#[cortex_m_rt::pre_init]
unsafe fn before_main() {
    // Soft-reset doesn't clear spinlocks. Clear the one used by critical-section
    // before we hit main to avoid deadlocks when using a debugger
    embassy_rp::pac::SIO.spinlock(31).write_value(1);
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Program start");
    if let Some(msg) = panic_persist::get_panic_message_utf8() {
        info!("{}", msg);
    }
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);

    let mut loop_count = 0;
    loop {
        if loop_count > 5 {
            panic!("panic at 10s to test");
        } else {
            loop_count += 1;
        }
        info!("led on!");
        led.set_high();
        Timer::after(Duration::from_secs(1)).await;

        info!("led off!");
        led.set_low();
        Timer::after(Duration::from_secs(1)).await;
    }
}

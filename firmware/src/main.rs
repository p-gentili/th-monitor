#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl, delay::Delay, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*,
    timer::TimerGroup, Rtc,
};
use sensor_temp_humidity_sht40::{I2CAddr, Precision, SHT40Driver, TempUnit};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Create a new peripheral object with the described wiring
    // and standard I2C clock speed
    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio21,
        io.pins.gpio22,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );
    // SHT40::read_serial(&mut i2c);
    let delay = Delay::new(&clocks);
    let mut sht40 = SHT40Driver::new(i2c, I2CAddr::SHT4x_A, delay);

    if let Ok(measurement) = sht40.get_temp_and_rh(Precision::High, TempUnit::MilliDegreesCelsius) {
        println!(
            "Temp: {temp} C, Relative Humidity: {rh} %",
            temp = measurement.temp / 1000,
            rh = measurement.rel_hum_pcm / 1000
        );
    }

    loop {}
}

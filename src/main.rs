// ===========
// Req 1, 2, 3
// ===========
#![no_std]
#![no_main]

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Alias for our HAL crate
use rp2040_hal as hal;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

// ===========
// Req 4
// ===========
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin; // or digital::{InputPin, OutputPin} for both

// ===========
// Req 5
// ===========
use rp2040_boot2;
#[link_section = ".boot2"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

// ===========
// Req 6
// ===========
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

// ===========
// Req 7
// ===========
#[rp2040_hal::entry]
fn main() -> ! {
    // Main doesn't return anything.
    // ===========
    // Req 8
    // ===========
    let mut pac = pac::Peripherals::take().unwrap(); // Handle errors if take() fails
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();
    // ===========
    // Req 9
    // ===========
    // Requires pac, clocks and watchdogs before init
    let mut timer = rp2040_hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // ===========
    // Req 10
    // ===========
    // Requires pac singleton before init
    let sio = hal::Sio::new(pac.SIO);

    // ===========
    // Req 11
    // ===========
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.gpio25.into_push_pull_output();
    // This actually goes last, after everything has been initialized
    loop {
        // Loopy things go here
        led_pin.set_high().unwrap();
        timer.delay_ms(500);
        led_pin.set_low().unwrap();
        timer.delay_ms(500);
    }
}
#![no_std]
#![no_main]

use panic_halt as _;

use rp2040_hal as hal;
use hal::pac;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

use rp2040_boot2;
#[link_section = ".boot2"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;
const UNIT_DELAY: u32 =  31_770_000;      // in nanoseconds
                                          // every 525 unit delays, vsync also updates
const HSYNC_TO_VSYNC_RATIO: u32 =  50;    // in nanoseconds

#[rp2040_hal::entry]
fn main() -> ! {
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
    let mut timer = rp2040_hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // // let mut vsync_pin = pins.gpio14.into_push_pull_output();
    // // VGA implementation
    // // Hsync needs to refresh every 31.77 microseconds and
    // // Vsync every 16.67 miliseconds.
    let mut hsync_pin = pins.gpio9.into_push_pull_output();
    let mut vsync_pin = pins.gpio10.into_push_pull_output();
    let mut hsync_state: bool = false;  // 0 state
    let mut vsync_state: bool = false;  // 0 state
    let mut counter: u32 = 0;
    loop {
        //Update hsync
        if hsync_state {
            hsync_pin.set_low().unwrap();
            hsync_state ^= true
        }
        else {
            hsync_pin.set_high().unwrap();
            hsync_state ^= true
        }

        if counter % UNIT_DELAY == HSYNC_TO_VSYNC_RATIO {
            // Update vsync
            if vsync_state {
                vsync_pin.set_low().unwrap();
                vsync_state ^= true
            }
            else {
                vsync_pin.set_high().unwrap();
                vsync_state ^= true
            }
        }

        counter += UNIT_DELAY;
        timer.delay_ns(UNIT_DELAY);
    }
}

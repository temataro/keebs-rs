# Keebs 'R Us

Informal notes on me learning how to actually make this.
If you read this and you're not me, pretend you didn't.

The project template for the `rp2040-hal` is good.

To flash the code onto the uC, `probe-rs` is used.
  We'll just copy it over the old fashioned way since I don't have a debug probe.

Probes can translate from the USB/Ethernet interface on your computer to the
target interface.
(SWD for ARM, JTAG for ?..., etc)

===============================================================================
We'll be using `elf2uf2-rs` which will convert the `.elf` file we generate to
binary targets for the cortex-m0+ uC. Loading a `.uf2` over USB.

Hold down the `BOOT` button on the uC while connecting to the PC and upload.
The pico should automatically get mounted on Linux.
It's found in `/run/media/tem`

> [!IMPORTANT]
> Go to `.cargo/config` and change the default runner 

#### <callout> `.cargo/config.toml`
```toml
[target.`cfg(all(target-arch = "arm", target_os = "none"))`]
runner = "elf2uf2-rs -d"
```

===============================================================================

(Alternate): Look into `picotool`. The `elf` produced by Rust should be compatible
with the output that it takes. Just append a `.elf` to the binary.


### Requirements:

* rustup
* thumbv6m-none-eabi toolchain
* elf2uf2-rs

### Bare minimum file structure in your project

```
.
|-> Cargo.toml
|-> .cargo/
    |-> config.toml
|
|-> src/
    |-> main.rs
|
| memory.x
```


```shell
rustup target install thumbv6m-none-eabi
cargo install --locked elf2uf2-rs
```

## Building and Running:
`cargo build --release --target thumbv6m-none-eabi`

After connecting the rp2040 board with the boot sel button pressed,
`cargo run` - debug build
or
`cargo run --release --target thumbv6m-none-eabi` - release build (Our runner (`elf2uf2-rs`) will build and
copy it to the RP2040.)


## memory.x


## Bootloader

So apparently there's a bootloader, and to write 256 bytes to it, we need to initialize
it. Use this snippet to do so.

#### <callout> `src/main.rs`
```rust
use rp2040_boot2;
#[link_section = ".boot2"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
```


## Cargo.toml

#### <callout> `Cargo.toml`
```toml
[package]
edition = "2021"
# We're using 2021 because `cargo build` errors with 2024. No stable version exists.
name = "Keebs-rs"
version = "0.1.0"
license = "MIT OR Apache-2.0"
author = "Tem soon but for now this is all rp-hal code..."

[dependencies]
cortex-m = "0.7"
cortex-m-rt = "0.7"
embedded-hal = { version = "1.0.0" }
rp2040-hal = { version="0.10", features=["rt", "critical-section-impl"] }
rp2040-boot2 = "0.3.0"
panic-halt = "0.2.0"

defmt = "0.3"
defmt-rtt = "0.4"
panic-probe = { version = "0.3", features = ["print-defmt"] }

rp-pico = "0.9"

# cargo build/run
[profile.dev]
codegen-units = 1
debug = 2
debug-assertions = true
incremental = false
opt-level = 3
overflow-checks = true

# cargo build/run --release
[profile.release]
codegen-units = 1
debug = 2
debug-assertions = false
incremental = false
lto = 'fat'
opt-level = 3
overflow-checks = false

# do not optimize proc-macro crates = faster builds from scratch
[profile.dev.build-override]
codegen-units = 8
debug = false
debug-assertions = false
opt-level = 0
overflow-checks = false

[profile.release.build-override]
codegen-units = 8
debug = false
debug-assertions = false
opt-level = 0
overflow-checks = false

# cargo test
[profile.test]
codegen-units = 1
debug = 2
debug-assertions = true
incremental = false
opt-level = 3
overflow-checks = true

# cargo test --release
[profile.bench]
codegen-units = 1
debug = 2
debug-assertions = false
incremental = false
lto = 'fat'
opt-level = 3
```


## The Bare Minimum to Include

Goal: GPIO25 is the onboard green LED.
Make blinky.

Req. 1) #![no_std]
  - Can only use the core features of Rust, no fancy std features in embedded.

Req. 2) #![no_main]
  - Define your entry point like a man. No main as default entry for your code.

Req. 3) Crates and aliases
  - Introduce crates for `panic_halt`, `rp2040_hal`
  - We need `rp2040_hal::pac` (Peripheral Access Crate)
  

#### <callout> `src/main.rs`

```rust
// File starts here
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
```

Req. 4) Embedded Traits
  - Need these to 
    - do delay,
    - define input and output pins,
  
```rust
// Cont'd
// Apparently not needed for blinky, we use other pins???
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin; // or digital::{InputPin, OutputPin} for both
```

Req. 5) The bootloader
  - We've talked about this

```rust
// Cont'd

use rp2040_boot2;
#[link_section = ".boot2"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
```

Req. 6) Clock stuff
  - Define the crystal's frequency
      - RP2040 by default takes 12MHz, we'll probably buy a 12MHz crystal also...

```rust
// Cont'd
const XTAL_FREQ_HZ: u32 = 12_000_000u32;
```

  - We'll also need to configure clocks later with `hal::clocks::init_clocks_and_plls()`
  - Watchdog timer (to make sure the uC doesn't go in an infinite loop?)

Req. 7) Entry

```rust
// Cont'd
#[rp2040_hal::entry]
fn main() -> ! {
 // Main doesn't return anything.


  // This actually goes last, after everything has been initialized
  loop {
    // Loopy things go here
    led_pin.set_high().unwrap();
    timer.delay_ms(500);
    led_pin.set_low().unwrap();
    timer.delay_ms(500);
  }
}
```

Req. 8) Initialize PAC Peripherals, watchdog, and clocks
```rust
// Inside main function
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
```

Req. 9) Timer
```rust
// Inside main function
// Requires pac, clocks and watchdogs before init
let mut timer = rp2040_hal::Timer::new(pac.Timer, &mut pac.RESETS, &clocks);
```

Req. 10) SIO: Single-Cycle I/O
```rust
// Inside main function
// Requires pac singleton before init
let sio = hal::Sio::new(pac.SIO);
```

Req. 11) Pins
```rust
// Inside main function
let pins = hal::gpio::Pins::new(
  pac.IO_BANK0,
  pac.PADS_BANK0,
  sio.gpio_bank0,
  &mut pac.RESETS,
);
```

// TODO: Actually paste the function prototypes for all these functions
// Timer::new(), Pins::new(), clocks::init_clocks_and_plls(), Sio::new()
// Peripherals::take(), Watchdog::new(), gpio::Pins::new()


#### Finally,
Now that we have a clock (from our timer, PAC, and watchdog), and GPIO pins
(from SIO and PAC), we can configure a specific GPIO pin as output and blink'er.


// TODO: Make a list of all the functions in the pins module
```rust
// After all the inits
let mut led_pin = pins.gpio25.into_push_pull_output();
```

### Notes

Defining the dependencies on `rp2040_hal`, `rp2040_boot2`, and `panic_halt` NEEDS a `-` between the words
but it has to be a `_` in the main function!!!!
WHATTTT????

### Other notes

YOU NEED A `memory.x` file for the rp2040!!!
Without one, simply uploading a file to ``/run/media/<username>/RP-PI2` will result in elf2uf2 saying
`Error: "entry point is not in mapped part of file"`

In order to use the memory.x file, `.cargo/config.toml` must be called with the right linker flags.
The linker script (whatever that is... .cargo/config.toml?) defines where different sections of code are placed
in memory. (.text, .data, etc)


### Implementing a VGA

All I know is that there is an hsync and vsync signal that need to be sent out.
Idea, divide delay into the smallest unit and test a counter conditional to see
whether each pin would update.

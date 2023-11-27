#![no_std]
#![no_main]

mod panic;

use riscv_rt::entry;

use gd32vf103xx_hal::{prelude::*, pac::*};

use embedded_hal::digital::v2::InputPin;

use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::pixelcolor::raw::LittleEndian;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::draw_target::DrawTarget;

use longan_nano::{lcd, lcd_pins};

const HAPPY: &[u8] = include_bytes!("../assets/build/happy.raw");

enum State {
    Up,
    Down,
}

impl State {

    pub fn state(value: bool) -> Self {
        match value {
            true => Self::Up,
            false => Self::Down,
        }
    }
    
    pub fn update(self, lcd: &mut lcd::Lcd) {
        match self {
            Self::Up => self.draw(lcd),
            Self::Down => self.clear(lcd),
        }
    }

    fn draw(&self, lcd: &mut lcd::Lcd) {
        let raw_image: ImageRaw<Rgb565, LittleEndian> = ImageRaw::new(&HAPPY, 40);
    
        let (width, height) = (lcd.size().width as u32, lcd.size().height as u32);
    
        Image::new(&raw_image, Point::new(width as i32/2 -20, height as i32/2 -20))
            .draw(lcd)
            .unwrap();
    }

    fn clear(&self, lcd: &mut lcd::Lcd) where lcd::Lcd: DrawTarget {
        lcd.clear(<lcd::Lcd as DrawTarget>::Color::BLACK).unwrap();
    }
}

#[entry]
fn main() -> ! {
    
    let dp = Peripherals::take().unwrap();

    let mut rcu = dp.RCU.configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    let mut afio = dp.AFIO.constrain(&mut rcu);
    
    let gpioa = dp.GPIOA.split(&mut rcu);

    let gpiob = dp.GPIOB.split(&mut rcu);

    let lcd_pins = lcd_pins!(gpioa, gpiob);

    let mut display = lcd::configure(dp.SPI0, lcd_pins, &mut afio, &mut rcu);

    let button = gpioa.pa11.into_pull_down_input();
    
    loop {

            State::state(button.is_high().unwrap_or(false)).update(&mut display);
        
    }
}


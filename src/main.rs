#![no_std]
#![no_main]

mod panic;
mod error;

use riscv_rt::entry;

use gd32vf103xx_hal::{prelude::*, pac::*};
use gd32vf103xx_hal::pwm::{Pins, PwmTimer, Channel, NoRemap};
use gd32vf103xx_hal::delay::McycleDelay;

use embedded_hal::digital::v2::InputPin;

use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::pixelcolor::raw::LittleEndian;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::draw_target::DrawTarget;

use longan_nano::{lcd, lcd_pins};

const HAPPY: &[u8] = include_bytes!("../assets/build/happy.raw");

//education is life
//https://youtu.be/w2Hw4gZW8lg?si=934sth7pvmnwWfsa
const FQ: &[u32] = &[660, 660, 660, 510, 660, 770, 380, 510, 380, 320, 440, 480, 450, 430, 380, 660, 760, 860, 700, 760, 660, 520, 580, 480];

const DR: &[u32] = &[150, 300, 300, 100, 300, 550, 575, 450, 400, 500, 300, 330, 150, 300, 200, 200, 150, 300, 150, 350, 300, 150, 150, 500];

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
    
    pub fn update(self, lcd: &mut lcd::Lcd, pwm: &mut PwmTimer::<TIMER0, NoRemap>, delay: &mut McycleDelay) {
        match self {
            Self::Up => {
                self.draw(lcd);
                self.play(pwm, delay);
            },
            Self::Down => {
                self.clear(lcd);
                self.stop(pwm);
            },
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

    fn play(self, pwm: &mut PwmTimer::<TIMER0, NoRemap>, delay: &mut McycleDelay) {
        let max = pwm.get_max_duty();
        pwm.set_duty(Channel::CH0, max / 2);
        pwm.enable(Channel::CH0);
        
        for index in 0..(FQ.len()-1) {
            pwm.set_period(FQ[index].hz());
            delay.delay_ms(DR[index]);
        }
    }

    fn stop(self, pwm: &mut PwmTimer::<TIMER0, NoRemap>) {
        pwm.disable(Channel::CH0);
    }
}

const MUX: u32 = u32::MAX/u8::MAX as u32;

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

    let sound = gpioa.pa8.into_alternate_push_pull();

    let mut pwm = PwmTimer::<TIMER0, NoRemap>::new(
        dp.TIMER0, (Some(&sound), None, None, None), &mut rcu, &mut afio);

    //let mut index: usize = 0;

    let mut delay = McycleDelay::new(&rcu.clocks);
    
    loop {

        State::state(button.is_high().unwrap_or(false)).update(&mut display, &mut pwm, &mut delay);

        /*
        let max = pwm.get_max_duty();
        pwm.set_period(FQ[index].hz());
        pwm.set_duty(Channel::CH0, max / 2); // 25% duty cycle
        pwm.enable(Channel::CH0);


        delay.delay_ms(DR[index]);
        
        if index == (FQ.len()-1) {
            index = 0;
        } else {
            index += 1;
        }
        */
    }
}


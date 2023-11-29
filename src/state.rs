
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::geometry::OriginDimensions;
use embedded_graphics::pixelcolor::raw::ByteOrder;

use crate::error::Error;

const HAPPY: &[u8] = include_bytes!("../assets/build/happy.raw");

#[derive(Debug)]
enum State {
    Up,
    Down,
}

impl <D, Bo> State 
where
    D: DrawTarget + OriginDimensions,
    Bo: ByteOrder,
{
    type Response = ();
    
    pub fn take(value: bool) -> Self {
        match value {
            true => Self::Up,
            false => Self::Down,
        }
    }
    
    pub fn update(self, frame: &mut D) -> Result<Self::Response, Error>{
        match self {
            Self::Up => self.draw(frame),
            Self::Down => self.clear(frame),
        }
    }

    fn draw(&self, frame: &mut D) -> Result<Self::Response, Error> {
        let (width, height) = (frame.size().width as i32, frame.size().height as i32);
        
        let raw_image: ImageRaw<<D as DrawTarget>::Color, Bo> = ImageRaw::new(&HAPPY, 40);
    
        let image = Image::new(&raw_image, Point::new(width/2 -20, height/2 -20));
        
        image.draw(frame).ok_or(Error::Draw)
    }

    fn clear(&self, frame: &mut D) -> Result<Self::Response, Error> {
        frame.clear(D::Color::BLACK).ok_or(Error::Draw)
    }
}

/*
#[cfg(test)]
mod state {
    use super::State;
    use crate::error::Error;
    
    use embedded_graphics::mock_display::MockDisplay;
    use embedded_graphics_core::draw_target::DrawTarget;
    use embedded_graphics_core::geometry::OriginDimensions;
    use embedded_graphics::pixelcolor::raw::ByteOrder;
    use embedded_graphics::pixelcolor::Rgb565;

    #[test]
    fn draw() {
        let mut screen: MockDisplay<Rgb888> = MockDisplay::new();
        
        assert_eq!(State::take(true).update(&mut screen), Err(Error::Draw));
    }
}
*/


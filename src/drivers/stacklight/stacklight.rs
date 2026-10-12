
use stm32h7xx_hal::hal::*;


// this only sets a 3v3 rail on for anything but like this iz just the logic you need we need to feed 14.8v somewhere anyways.
// No error types because it's just not as finnicky as UART or I2C.
pub enum LightStatus {
    Red,
    Yellow,
    Green,
    Speaker 
}

pub struct StackLight<R, Y, G, S> {
    red: R,
    yellow: Y,
    green: G,
    speaker: S,
}
impl<R, Y, G, S> StackLight<R,Y,G,S>
where
    R: digital::v2::OutputPin<Error = E>,
    Y: digital::v2::OutputPin<Error = E>,
    G: digital::v2::OutputPin<Error = E>,
    S: digital::v2::OutputPin<Error = E>,
{
    pub fn new(r: R, y: Y, g: G, s: S) -> Self {
        Self {
            red: R,
            yellow: Y,
            green: G,
            speaker: S,
        }
    }
    pub fn update(&mut self, status: LightStatus) {
        self.flush(&mut self);
        match &status {
            LightStatus::Red => {
                self.red.set_high();
            }
            LightStatus::Yellow => {
                self.yellow.set_high();
            }
            LightStatus::Green => {
                self.green.set_high();
            }
            // I don't know what to do with the speaker yet
            _ => {
            }
        }
    }

    pub fn speaker(&mut self, opt: bool) {
        match &opt {
            true => {
                self.speaker.set_high(); 
            }
            false => {
                self.speaker.set_low();
            }
        }
    } 
    // There's no way setting high or low fails, I think it's fine tbh.
    pub fn flush(&mut self) {
        self.red.set_low();    
        self.yellow.set_low();    
        self.green.set_low();    
        self.speaker.set_low();
    }
        
    }
}

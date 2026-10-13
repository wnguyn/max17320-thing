use embedded_hal::digital::OutputPin;
// don't tell ryan this....
//
pub enum LightStatus {
    Red,
    Yellow,
    Green,
    Speaker,
}

pub struct StackLight<R, Y, G, S> {
    red: R,
    yellow: Y,
    green: G,
    speaker: S,
}
// there's literally no way sending high or low fails, so we're just not gonna return anything lol....
// but i mean this is just later on if needed
// it's sending 3v3 rail so we'd have to do something to check or whatever...
impl<R, Y, G, S> StackLight<R, Y, G, S>
where
    R: OutputPin,
    Y: OutputPin,
    G: OutputPin,
    S: OutputPin,
{
    pub fn new(r: R, y: Y, g: G, s: S) -> Self {
        Self {
            red: r,
            yellow: y,
            green: g,
            speaker: s,
        }
    }
    pub fn update(&mut self, status: LightStatus) {
        self.flush();
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
            LightStatus::Speaker => {}
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
    pub fn flush(&mut self) {
        self.red.set_low();
        self.yellow.set_low();
        self.green.set_low();
        self.speaker.set_low();
    }
}

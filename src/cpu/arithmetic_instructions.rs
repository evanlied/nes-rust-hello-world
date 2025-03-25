use super::CPU;

impl CPU {
    pub fn increment_x(&mut self) {
        self.register_x += 1;
        self.set_status_flag(self.register_x);
    }
}
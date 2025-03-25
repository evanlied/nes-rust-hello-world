use super::CPU;

impl CPU {
    pub fn load_register_a(&mut self, program: &Vec<u8>) {
        self.program_counter += 1;
        let param = &program[self.program_counter as usize];
        self.register_a = *param;

        self.set_status_flag(*param);
    }

    pub fn transfer_a_to_x(&mut self) {
        self.register_x = self.register_a;
        self.set_status_flag(self.register_x);
    }
}

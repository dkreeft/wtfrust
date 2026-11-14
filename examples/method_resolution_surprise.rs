struct Device;

trait Run {
    fn run(&self);
}

impl Device {
    fn run(&mut self) {
        println!("inherent");
    }
}

impl Run for Device {
    fn run(&self) {
        println!("trait");
    }
}

fn main() {
    let mut device = Device;
    device.run();
    Device::run(&mut device);
}
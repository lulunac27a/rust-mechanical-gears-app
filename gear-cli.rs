struct GearTrain {
    driver_teeth: u32,
    driven_teeth: u32,
    module: f64, // mm per tooth
    input_rpm: f64,
}

impl GearTrain {
    fn new(driver_teeth: u32, driven_teeth: u32, module: f64, input_rpm: f64) -> Self {
        Self {
            driver_teeth,
            driven_teeth,
            module,
            input_rpm,
        }
    }

    // Calculate gear ratio (driven / driver)
    fn gear_ratio(&self) -> f64 {
        self.driven_teeth as f64 / self.driver_teeth as f64
    }

    // Calculate pitch diameter in mm (Module * Teeth)
    fn pitch_diameter(&self, teeth: u32) -> f64 {
        self.module * teeth as f64
    }

    // Calculate output speed in RPM
    fn output_rpm(&self) -> f64 {
        self.input_rpm / self.gear_ratio()
    }
}

fn main() {
    // Example: Driver gear has 20 teeth, driven gear has 60 teeth, module is 2.0 mm, input is 1200 RPM
    let train = GearTrain::new(20, 60, 2.0, 1200.0);

    let ratio = train.gear_ratio();
    let driver_pd = train.pitch_diameter(train.driver_teeth);
    let driven_pd = train.pitch_diameter(train.driven_teeth);
    let out_rpm = train.output_rpm();

    println!("Driver Teeth: {}", train.driver_teeth);
    println!("Driven Teeth: {}", train.driven_teeth);
    println!("Gear Ratio: {:.2}:1", ratio);
    println!("Driver Pitch Diameter: {:.2} mm", driver_pd);
    println!("Driven Pitch Diameter: {:.2} mm", driven_pd);
    println!("Output Speed: {:.2} RPM", out_rpm);
}

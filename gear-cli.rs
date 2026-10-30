struct GearTrain {
    //gear train struct to hold gear parameters
    driver_teeth: u32, //number of teeth on the driver gear
    driven_teeth: u32, //number of teeth on the driven gear
    module: f64,       // mm per tooth
    input_rpm: f64,    //input speed in RPM
}

impl GearTrain {
    //implementation of methods for GearTrain
    fn new(
        driver_teeth: u32,
        driven_teeth: u32,
        module: f64,
        input_rpm: f64,
    ) -> Self {
        //constructor for GearTrain
        Self {
            driver_teeth,
            driven_teeth,
            module,
            input_rpm,
        }
    }

    // Calculate gear ratio (driven / driver)
    fn gear_ratio(&self) -> f64 {
        //calculate gear ratio
        self.driven_teeth as f64 / self.driver_teeth as f64 //returns the gear ratio as a float
    }

    // Calculate pitch diameter in mm (Module * Teeth)
    fn pitch_diameter(&self, teeth: u32) -> f64 {
        //calculate pitch diameter
        self.module * teeth as f64 //returns the pitch diameter in mm
    }

    // Calculate output speed in RPM
    fn output_rpm(&self) -> f64 {
        //calculate output speed in RPM
        self.input_rpm / self.gear_ratio() //returns the output speed in RPM
    }
}

fn main() {
    //main function to demonstrate usage of GearTrain
    // Example: Driver gear has 20 teeth, driven gear has 60 teeth, module is 2.0 mm, input is 1200 RPM
    let train = GearTrain::new(20, 60, 2.0, 1200.0); //create a new GearTrain instance with specified parameters

    let ratio = train.gear_ratio(); //calculate the gear ratio
    let driver_pd = train.pitch_diameter(train.driver_teeth); //calculate the pitch diameter of the driver gear
    let driven_pd = train.pitch_diameter(train.driven_teeth); //calculate the pitch diameter of the driven gear
    let out_rpm = train.output_rpm(); //calculate the output speed in RPM

    println!("Driver Teeth: {}", train.driver_teeth); //print the number of teeth on the driver gear
    println!("Driven Teeth: {}", train.driven_teeth);//print the number of teeth on the driven gear
    println!("Gear Ratio: {:.2}:1", ratio);//print the gear ratio
    println!("Driver Pitch Diameter: {:.2} mm", driver_pd);//print the pitch diameter of the driver gear
    println!("Driven Pitch Diameter: {:.2} mm", driven_pd);//print the pitch diameter of the driven gear
    println!("Output Speed: {:.2} RPM", out_rpm);//print the output speed in RPM
}

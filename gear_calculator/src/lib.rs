use serde::Serialize;
use wasm_bindgen::prelude::*;//import the serde crate for serialization

#[derive(Serialize)]//derive the Serialize trait for the GearResult struct
pub struct GearResult {//define a struct to hold the results of the gear calculation
    pub gear_ratio: f64,//gear ratio of the gear train
    pub driven_rpm: f64,//output speed in RPM of the driven gear
    pub center_distance: f64,//center distance between the gears in mm
}

#[wasm_bindgen]//indicate that the following struct and its methods will be exposed to WebAssembly
pub struct GearTrain {//define a struct to hold the parameters of the gear train
    driver_teeth: u32,//number of teeth on the driver gear
    driven_teeth: u32,//number of teeth on the driven gear
    module: f64, // Size of gear teeth (pitch diameter / number of teeth)
}

#[wasm_bindgen]
impl GearTrain {//implement methods for the GearTrain struct
    #[wasm_bindgen(constructor)]//indicate that this is the constructor for the GearTrain struct
    pub fn new(driver_teeth: u32, driven_teeth: u32, module: f64) -> GearTrain {//constructor for GearTrain
        GearTrain {
            driver_teeth,
            driven_teeth,
            module,
        }
    }

    /// Calculate gear ratio and output RPM given input RPM
    pub fn calculate(&self, driver_rpm: f64) -> String {//calculate the gear ratio and output RPM based on the input RPM
        let ratio = self.driven_teeth as f64 / self.driver_teeth as f64;//calculate the gear ratio (driven teeth / driver teeth)
        let driven_rpm = driver_rpm / ratio;//calculate the output speed in RPM of the driven gear based on the input RPM and gear ratio

        // Pitch diameter = module * teeth
        let d_driver = self.module * (self.driver_teeth as f64);//calculate the pitch diameter of the driver gear
        let d_driven = self.module * (self.driven_teeth as f64);//calculate the pitch diameter of the driven gear
        let center_distance = (d_driver + d_driven) / 2.0;//calculate the center distance between the gears in mm

        let result = GearResult {//create a new GearResult struct to hold the results of the calculation
            gear_ratio: ratio,
            driven_rpm,
            center_distance,
        };

        serde_json::to_string(&result).unwrap()//serialize the GearResult struct to a JSON string and return it
    }
}

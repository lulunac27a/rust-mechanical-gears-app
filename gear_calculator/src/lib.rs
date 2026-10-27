use wasm_bindgen::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct GearResult {
    pub gear_ratio: f64,
    pub driven_rpm: f64,
    pub center_distance: f64,
}

#[wasm_bindgen]
pub struct GearTrain {
    driver_teeth: u32,
    driven_teeth: u32,
    module: f64, // Size of gear teeth (pitch diameter / number of teeth)
}

#[wasm_bindgen]
impl GearTrain {
    #[wasm_bindgen(constructor)]
    pub fn new(driver_teeth: u32, driven_teeth: u32, module: f64) -> GearTrain {
        GearTrain {
            driver_teeth,
            driven_teeth,
            module,
        }
    }

    /// Calculate gear ratio and output RPM given input RPM
    pub fn calculate(&self, driver_rpm: f64) -> String {
        let ratio = self.driven_teeth as f64 / self.driver_teeth as f64;
        let driven_rpm = driver_rpm / ratio;
        
        // Pitch diameter = module * teeth
        let d_driver = self.module * (self.driver_teeth as f64);
        let d_driven = self.module * (self.driven_teeth as f64);
        let center_distance = (d_driver + d_driven) / 2.0;

        let result = GearResult {
            gear_ratio: ratio,
            driven_rpm,
            center_distance,
        };

        serde_json::to_string(&result).unwrap()
    }
}

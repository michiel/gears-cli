use gears::structure::common::ModelLoadError;
use gears::structure::gxmodel::GxModel;

pub trait ModelStore {
    fn list(&self) -> Result<Vec<GxModel>, ModelLoadError>;
    fn get(&self, id: &str) -> Result<GxModel, ModelLoadError>;
    fn new(&self) -> Result<GxModel, ModelLoadError>;
    fn create(&self, json: &str) -> Result<GxModel, ModelLoadError>;
    fn update(&self, json: &str) -> Result<GxModel, ModelLoadError>;
    fn delete(&self, json: &str) -> Result<(), ModelLoadError>;
}

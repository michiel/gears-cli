use actix::prelude::*;
use futures::Future;
use gears;
use gears::structure::common::ModelLoadError;
use gears::structure::gxmodel::GxModel;

use super::model_executor::ModelStore;

#[derive(Clone)]
pub struct SQLliteModelStore {
    root: String,
}

impl SQLliteModelStore {
    pub fn new(path: &str) -> Result<Self, ModelLoadError> {
        unimplemented!()
    }
}

impl ModelStore for SQLliteModelStore {
    fn list(&self) -> Result<Vec<GxModel>, ModelLoadError> {
        unimplemented!()
    }

    fn get(&self, _id: &str) -> Result<GxModel, ModelLoadError> {
        unimplemented!()
    }

    fn new(&self) -> Result<GxModel, ModelLoadError> {
        unimplemented!()
    }

    fn create(&self, json: &str) -> Result<GxModel, ModelLoadError> {
        unimplemented!()
    }

    fn update(&self, json: &str) -> Result<GxModel, ModelLoadError> {
        unimplemented!()
    }

    fn delete(&self, json: &str) -> Result<(), ModelLoadError> {
        unimplemented!()
    }
}

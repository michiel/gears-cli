use actix::prelude::*;
use futures::Future;
use gears;
use gears::structure::common::ModelLoadError;
use gears::structure::model::ModelDocument;

use super::model_executor::{InputError, ModelStore};

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
    fn list(&self) -> Result<Vec<ModelDocument>, InputError> {
        unimplemented!()
    }

    fn get(&self, _id: &str) -> Result<ModelDocument, InputError> {
        unimplemented!()
    }

    fn new(&self) -> Result<ModelDocument, InputError> {
        unimplemented!()
    }

    fn create(&self, json: &str) -> Result<ModelDocument, InputError> {
        unimplemented!()
    }

    fn update(&self, json: &str) -> Result<ModelDocument, InputError> {
        unimplemented!()
    }

    fn delete(&self, json: &str) -> Result<(), InputError> {
        unimplemented!()
    }
}

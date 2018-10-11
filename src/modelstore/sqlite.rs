use gears::structure::model::ModelDocument;
use gears::structure::common::ModelLoadError;
use futures::Future;
use actix::prelude::*;
use gears;

use diesel::Connection;
use diesel::prelude::*;

use super::model_executor::{ModelStore, InputError};
use super::sqlite_schema;

#[derive(Clone)]
pub struct SQLliteModelStore {
    root: String,
    connection: SqliteConnection
}

impl SQLliteModelStore {
    pub fn fnord(path: &str) -> Result<Self, ModelLoadError> {

        match SqliteConnection::establish(path) {
            Ok(conn) => {
                Ok(SQLliteModelStore {
                    root: path.to_owned(),
                    connection: conn,
                })
            },
            Err(_) => {
                panic!("Error connecting to {}", path);
            }
        }
    }
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


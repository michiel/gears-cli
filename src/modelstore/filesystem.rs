use actix::prelude::*;
use futures::Future;
use gears;
use gears::structure::common::ModelLoadError;
use gears::structure::model::ModelDocument;

use super::model_executor::{InputError, ModelStore};

fn load_model(path: &str) -> Result<ModelDocument, InputError> {
    match gears::util::fs::model_from_fs(path) {
        Ok(model) => Ok(model),
        Err(_) => Err(InputError::IOError),
    }
}

#[derive(Clone)]
pub struct FileSystemModelStore {
    root: String,
}

impl FileSystemModelStore {
    pub fn new(path: &str) -> Result<Self, ModelLoadError> {
        match load_model(&path) {
            Ok(_) => Ok(FileSystemModelStore {
                root: path.to_owned(),
            }),
            Err(err) => Err(ModelLoadError::BadStructure("Unable to init".to_owned())),
        }
    }
}

impl ModelStore for FileSystemModelStore {
    fn list(&self) -> Result<Vec<ModelDocument>, InputError> {
        match load_model(&self.root) {
            Ok(res) => Ok(vec![res]),
            Err(_) => Err(InputError::IOError),
        }
    }

    fn get(&self, _id: &str) -> Result<ModelDocument, InputError> {
        load_model(&self.root)
    }

    fn new(&self) -> Result<ModelDocument, InputError> {
        info!("init: in directory {}", self.root);
        match gears::util::fs::init_new_model_dir(&self.root) {
            Ok(_) => load_model(&self.root),
            Err(err) => {
                let msg = format!("{:?}", err);
                Err(InputError::BadFormat(msg))
            }
        }
    }

    fn create(&self, json: &str) -> Result<ModelDocument, InputError> {
        info!("create: in directory {}", self.root);
        match gears::structure::model::ModelDocument::from_json(&json) {
            Ok(model) => match gears::util::fs::model_to_fs(&model, &self.root) {
                Ok(_) => load_model(&self.root),
                Err(_) => Err(InputError::IOError),
            },
            Err(err) => Err(InputError::BadFormat(format!("{:?}", err))),
        }
    }

    fn update(&self, json: &str) -> Result<ModelDocument, InputError> {
        info!("update: in directory {}", self.root);
        match gears::structure::model::ModelDocument::from_json(&json) {
            Ok(model) => match gears::util::fs::model_to_fs(&model, &self.root) {
                Ok(_) => load_model(&self.root),
                Err(_) => Err(InputError::IOError),
            },
            Err(err) => Err(InputError::BadFormat(format!("{:?}", err))),
        }
    }

    fn delete(&self, json: &str) -> Result<(), InputError> {
        unimplemented!()
    }
}

impl Actor for FileSystemModelStore {
    type Context = SyncContext<Self>;

    fn started(&mut self, ctx: &mut SyncContext<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut SyncContext<Self>) {
        println!("Actor is stopped");
    }
}

struct ModelStoreList;

impl Message for ModelStoreList {
    type Result = Result<Vec<ModelDocument>, InputError>;
}

impl Handler<ModelStoreList> for FileSystemModelStore {
    type Result = Result<Vec<ModelDocument>, InputError>;

    fn handle(&mut self, msg: ModelStoreList, ctx: &mut SyncContext<Self>) -> Self::Result {
        self.list()
    }
}

struct ModelStoreGet<'a> {
    id: &'a str,
}

impl<'a> Message for ModelStoreGet<'a> {
    type Result = Result<ModelDocument, InputError>;
}

impl<'a> Handler<ModelStoreGet<'a>> for FileSystemModelStore {
    type Result = Result<ModelDocument, InputError>;

    fn handle(&mut self, msg: ModelStoreGet, ctx: &mut SyncContext<Self>) -> Self::Result {
        self.get(&msg.id)
    }
}

struct ModelStoreNew;

impl Message for ModelStoreNew {
    type Result = Result<ModelDocument, InputError>;
}

impl Handler<ModelStoreNew> for FileSystemModelStore {
    type Result = Result<ModelDocument, InputError>;

    fn handle(&mut self, msg: ModelStoreNew, ctx: &mut SyncContext<Self>) -> Self::Result {
        self.new()
    }
}

struct ModelStoreCreate<'a> {
    json: &'a str,
}

impl<'a> Message for ModelStoreCreate<'a> {
    type Result = Result<ModelDocument, InputError>;
}

impl<'a> Handler<ModelStoreCreate<'a>> for FileSystemModelStore {
    type Result = Result<ModelDocument, InputError>;

    fn handle(&mut self, msg: ModelStoreCreate, ctx: &mut SyncContext<Self>) -> Self::Result {
        self.create(msg.json)
    }
}

struct ModelStoreDelete<'a> {
    id: &'a str,
}

impl<'a> Message for ModelStoreDelete<'a> {
    type Result = Result<(), InputError>;
}

impl<'a> Handler<ModelStoreDelete<'a>> for FileSystemModelStore {
    type Result = Result<(), InputError>;

    fn handle(&mut self, msg: ModelStoreDelete, ctx: &mut SyncContext<Self>) -> Self::Result {
        self.delete(msg.id)
    }
}

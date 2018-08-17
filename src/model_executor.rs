use gears::structure::model::ModelDocument;
use gears;

use actix::prelude::*;
use futures::Future;

fn load_model(path: &str) -> Result<ModelDocument, InputError> {
    match gears::util::fs::model_from_fs(path) {
        Ok(model) => Ok(model),
        Err(_) => Err(InputError::IOError)
    }
}

pub enum InputError {
    IOError,
    BadFormat,
}

pub trait ModelStore {
    fn list(&self) -> Result<Vec<ModelDocument>, InputError>;
    fn get(&self, id: &str) -> Result<ModelDocument, InputError>;
    fn new(&self) -> Result<ModelDocument, InputError>;
    fn create(&self, json: &str) -> Result<ModelDocument, InputError>;
    fn update(&self, json: &str) -> Result<ModelDocument, InputError>;
    fn delete(&self, json: &str) -> Result<(), InputError>;
}

#[derive(Clone)]
pub struct FileSystemModelStore {
    root: String,
}

impl FileSystemModelStore {
    pub fn new(path: &str) -> Self {
        FileSystemModelStore {
            root: path.to_owned()
        }
    }
}

impl ModelStore for FileSystemModelStore {
    fn list(&self) -> Result<Vec<ModelDocument>, InputError> {
        match load_model(&self.root) {
            Ok(res) => Ok(vec![res]),
            Err(_) => Err(InputError::IOError)
        }
    }

    fn get(&self, _id: &str) -> Result<ModelDocument, InputError> {
        load_model(&self.root)
    }

    fn new(&self) -> Result<ModelDocument, InputError> {
        info!("init: in directory {}", self.root);
        match gears::util::fs::init_new_model_dir(&self.root) {
            Ok(_) => load_model(&self.root),
            Err(_) => Err(InputError::BadFormat),
        }
    }

    fn create(&self, json: &str) -> Result<ModelDocument, InputError> {
        info!("create: in directory {}", self.root);
        let model = gears::structure::model::ModelDocument::from_json(&json);
        match gears::util::fs::model_to_fs(&model, &self.root) {
            Ok(_) => load_model(&self.root),
            Err(_) => Err(InputError::IOError),
        }
    }

    fn update(&self, json: &str) -> Result<ModelDocument, InputError> {
        unimplemented!()
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
    id: &'a str
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
    json: &'a str
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
    id: &'a str
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


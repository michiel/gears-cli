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

enum InputError {
    IOError,
    BadFormat,
}

trait ModelStore {

    fn list(&self) -> Vec<ModelDocument>;

    fn get(&self, id: &str) -> Result<ModelDocument, InputError>;

    fn new(&self) -> Result<ModelDocument, InputError>;

    fn create(&self, json: &str) -> Result<ModelDocument, InputError>;

    fn update(&self, json: &str) -> Result<ModelDocument, InputError>;

    fn delete(&self, json: &str) -> Result<ModelDocument, InputError>;
}

struct FileSystemModelStore {
    root: String,
}

impl FileSystemModelStore {
    fn new(path: &str) -> Self {
        FileSystemModelStore {
            root: path.to_owned()
        }
    }
}

impl ModelStore for FileSystemModelStore {
    fn list(&self) -> Vec<ModelDocument> {
        unimplemented!()
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

    fn delete(&self, json: &str) -> Result<ModelDocument, InputError> {
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
    type Result = Result<bool, InputError>;
}

impl Handler<ModelStoreList> for FileSystemModelStore {
    type Result = Result<bool, InputError>;
    fn handle(&mut self, msg: ModelStoreList, ctx: &mut SyncContext<Self>) -> Self::Result {
        Ok(true)
    }
}


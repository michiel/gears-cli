use actix::prelude::*;
use futures::Future;
use gears;
use gears::structure::common::{ModelLoadError, DocumentNature, DocumentFileSystemLoadable};
use gears::structure::gxmodel::GxModel;


use super::model_executor::{ModelStore};

#[derive(Clone)]
pub struct FileSystemModelStore {
    root: String,
}

impl FileSystemModelStore {
    pub fn new(path: &str) -> Result<Self, ModelLoadError> {
        match GxModel::load_from_filesystem(&path) {
            Ok(_) => Ok(FileSystemModelStore {
                root: path.to_owned(),
            }),
            Err(err) => Err(ModelLoadError::BadStructure("Unable to init".to_owned())),
        }
    }
}

impl ModelStore for FileSystemModelStore {
    fn list(&self) -> Result<Vec<GxModel>, ModelLoadError> {
        match GxModel::load_from_filesystem(&self.root) {
            Ok(res) => Ok(vec![res]),
            Err(err) => Err(err)
        }
    }

    fn get(&self, _id: &str) -> Result<GxModel, ModelLoadError> {
        GxModel::load_from_filesystem(&self.root)
    }

    fn new(&self) -> Result<GxModel, ModelLoadError> {
        info!("init: in directory {}", self.root);
        match gears::util::fs::init_new_model_dir(&self.root) {
            Ok(_) => GxModel::load_from_filesystem(&self.root),
            Err(err) => {
                let msg = format!("{:?}", err);
                Err(ModelLoadError::InputError(msg))
            }
        }
    }

    fn create(&self, json: &str) -> Result<GxModel, ModelLoadError> {
        info!("create: in directory {}", self.root);
        match GxModel::from_json(&json) {
            Ok(model) => {
                match &model.write_to_filesystem(&self.root) {
                    Ok(_) => GxModel::load_from_filesystem(&self.root),
                    Err(msg) => {
                        let msg = format!("{:?}", msg);
                        Err(ModelLoadError::InputError(msg))
                    }
                }
            },
            Err(err) =>{
                let msg = format!("{:?}", err);
                Err(ModelLoadError::InputError(msg))
            }
        }
    }

    fn update(&self, json: &str) -> Result<GxModel, ModelLoadError> {
        info!("update: in directory {}", self.root);
        match GxModel::from_json(&json) {
            Ok(model) => {
                match &model.write_to_filesystem(&self.root) {
                    Ok(_) => GxModel::load_from_filesystem(&self.root),
                    Err(msg) => {
                        let msg = format!("{:?}", msg);
                        Err(ModelLoadError::InputError(msg))
                    }
                }
            },
            Err(err) =>{
                let msg = format!("{:?}", err);
                Err(ModelLoadError::InputError(msg))
            }
        }
    }

    fn delete(&self, json: &str) -> Result<(), ModelLoadError> {
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
    type Result = Result<Vec<GxModel>, ModelLoadError>;
}

impl Handler<ModelStoreList> for FileSystemModelStore {
    type Result = Result<Vec<GxModel>, ModelLoadError>;

    fn handle(&mut self, msg: ModelStoreList, ctx: &mut SyncContext<Self>) -> Self::Result {
        self.list()
    }
}

struct ModelStoreGet<'a> {
    id: &'a str,
}

impl<'a> Message for ModelStoreGet<'a> {
    type Result = Result<GxModel, ModelLoadError>;
}

impl<'a> Handler<ModelStoreGet<'a>> for FileSystemModelStore {
    type Result = Result<GxModel, ModelLoadError>;

    fn handle(&mut self, msg: ModelStoreGet, ctx: &mut SyncContext<Self>) -> Self::Result {
        self.get(&msg.id)
    }
}

struct ModelStoreNew;

impl Message for ModelStoreNew {
    type Result = Result<GxModel, ModelLoadError>;
}

impl Handler<ModelStoreNew> for FileSystemModelStore {
    type Result = Result<GxModel, ModelLoadError>;

    fn handle(&mut self, msg: ModelStoreNew, ctx: &mut SyncContext<Self>) -> Self::Result {
        self.new()
    }
}

struct ModelStoreCreate<'a> {
    json: &'a str,
}

impl<'a> Message for ModelStoreCreate<'a> {
    type Result = Result<GxModel, ModelLoadError>;
}

impl<'a> Handler<ModelStoreCreate<'a>> for FileSystemModelStore {
    type Result = Result<GxModel, ModelLoadError>;

    fn handle(&mut self, msg: ModelStoreCreate, ctx: &mut SyncContext<Self>) -> Self::Result {
        self.create(msg.json)
    }
}

struct ModelStoreDelete<'a> {
    id: &'a str,
}

impl<'a> Message for ModelStoreDelete<'a> {
    type Result = Result<(), ModelLoadError>;
}

impl<'a> Handler<ModelStoreDelete<'a>> for FileSystemModelStore {
    type Result = Result<(), ModelLoadError>;

    fn handle(&mut self, msg: ModelStoreDelete, ctx: &mut SyncContext<Self>) -> Self::Result {
        self.delete(msg.id)
    }
}

use std::io::{self, Read};
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use gears::structure::model::ModelDocument;
use gears;

fn load_model(path: &str) -> Result<ModelDocument, InputError> {
    match gears::util::fs::model_from_fs(path) {
        Ok(model) => Ok(model),
        Err(_) => Err(InputError::IOError)
    }
}

fn write_file(filename: &str, data: &str) -> () {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => {
            error!("couldn't create {}: {}", display, why.description());
            panic!("couldn't create {}: {}", display, why.description());
        }
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
        Err(why) => {
            error!("couldn't write to {}: {}", display, why.description());
            panic!("couldn't write to {}: {}", display, why.description());
        }
        Ok(_) => debug!("successfully wrote to {}", display),
    }
}

enum InputError {
    IOError,
    BadFormat,
    InvalidIdentifier,
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

    fn get(&self, id: &str) -> Result<ModelDocument, InputError> {
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


use gears::structure::model::ModelDocument;

#[derive(Debug)]
pub enum InputError {
    IOError,
    BadFormat(String),
}

pub trait ModelStore {
    fn list(&self) -> Result<Vec<ModelDocument>, InputError>;
    fn get(&self, id: &str) -> Result<ModelDocument, InputError>;
    fn new(&self) -> Result<ModelDocument, InputError>;
    fn create(&self, json: &str) -> Result<ModelDocument, InputError>;
    fn update(&self, json: &str) -> Result<ModelDocument, InputError>;
    fn delete(&self, json: &str) -> Result<(), InputError>;
}


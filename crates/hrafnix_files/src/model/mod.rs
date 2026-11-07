use camino::Utf8Path;
use hrafnix_message::message::Message;
use crate::common::read_file_to_string;
use crate::model::data::ModelFile;

pub mod data;
pub(crate) mod writer;
pub(crate) mod reader;

pub fn read_model_file(path: &Utf8Path) -> Result<ModelFile, Message> {
    let string = read_file_to_string(path)?;


}
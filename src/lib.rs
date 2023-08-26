use dicom::core::{value::ConvertValueError, Tag};
use dicom::dictionary_std::tags;
use dicom::object::{open_file, AccessError, ReadError};
use std::convert::AsRef;
use std::path::{Path, PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DicomReadError(#[from] ReadError),
    #[error(transparent)]
    DicomAccessError(#[from] AccessError),
    #[error(transparent)]
    DicomConvertValueError(#[from] ConvertValueError),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DicomFile {
    pub path: PathBuf,
    pub sop_class_uid: String,
    pub sop_instance_uid: String,
    pub modality: String,
    pub patient_id: String,
    pub patient_name: String,
    pub frame_of_reference_uid: String,
}

impl DicomFile {
    pub fn read_path<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let obj = open_file(path.as_ref())?;
        let sop_class_uid = obj.element(tags::SOP_CLASS_UID)?.to_str()?.to_string();
        let sop_instance_uid = obj.element(Tag(0x0008, 0x0018))?.to_str()?.to_string();
        let modality = obj.element(Tag(0x0008, 0x0060))?.to_str()?.to_string();
        let patient_id = obj.element(Tag(0x0010, 0x0020))?.to_str()?.to_string();
        let patient_name = obj.element(Tag(0x0010, 0x0010))?.to_str()?.to_string();
        let frame_of_reference_uid = obj.element(Tag(0x0020, 0x0052))?.to_str()?.to_string();

        Ok(DicomFile {
            path: path.as_ref().into(),
            sop_class_uid,
            sop_instance_uid,
            modality,
            patient_id,
            patient_name,
            frame_of_reference_uid,
        })
    }
}

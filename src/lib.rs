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

/// Stores the path of the DICOM file and a limited set of DICOM attributes to identify the patient
/// and the type of DICOM file.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DicomFile {
    /// Path of the DICOM file
    pub path: PathBuf,
    /// Uniquely identifies the SOP class.
    pub sop_class_uid: String,
    /// Uniquely identifies the SOP instance.
    pub sop_instance_uid: String,
    /// Type of equipment on which the data was acquired.
    pub modality: String,
    /// Uniquely identifies the patient.
    pub patient_id: String,
    /// Name of the patient
    pub patient_name: String,
    /// Uniquely identifies the Frame of Reference for a series.
    pub frame_of_reference_uid: String,
}

/// Read a DicomFile from a path.
///
/// # Attributes
///
/// * `path` - file path to a DICOM file
///
/// # Returns
///
/// A Result is returned with either a DicomFile, or an Error.
pub fn read_dicom_file<P>(path: P) -> Result<DicomFile, Error>
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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OdelError {
    #[error("{0}")]
    GeneralError(String),
    #[error("File processing failed in TRIRIGA for '{0}'. Status: '{1}'. For more details, see \
    {2}")]
    UploadFailed(String, String, String),
    #[error("File status did not change after the maximum wait time had elapsed.
 Verify that the 'Data Upload' agent is running on the server you are connecting to.
 The most recent status was '{0}'.")]
    Timeout(String),
}

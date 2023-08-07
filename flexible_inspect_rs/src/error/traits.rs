use std::fmt::Debug;
pub trait ValidationError: Send + Sync + Debug {
    fn get_code(&self) -> i32;
    fn get_message(&self) -> &str;
}

#[cfg(feature = "export_to_other_languages")]
pub trait ValidationErrorExportLang {
    type Callback;
    type ItemError;
    type OkStatus;
    type ErrorStatus;
    fn next(&mut self) -> Option<Self::ItemError>;
    fn for_each_with_reserved_parameters(
        &self,
        callback: Self::Callback,
    ) -> Result<Vec<Self::OkStatus>, Self::ErrorStatus>;
    fn if_error_with_reserved_parameters(
        &self,
        callback: Self::Callback,
    ) -> Result<Vec<Self::OkStatus>, Self::ErrorStatus>;
    fn for_each(&self, callback: Self::Callback) -> Result<Vec<Self::OkStatus>, Self::ErrorStatus>;
    fn if_error(&self, callback: Self::Callback) -> Result<Vec<Self::OkStatus>, Self::ErrorStatus>;
    fn if_ok(&self, callback: Self::Callback) -> Result<Self::OkStatus, Self::ErrorStatus>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
}

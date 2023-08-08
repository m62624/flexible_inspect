// =======================================================
mod validate_bytes;
mod validate_str;
use crate::cartridges::traits::CartridgeBase;
use crate::error::iterator::ValidationErrorIterator;
// =======================================================
#[cfg(feature = "serde")]
use super::{Deserialize, Serialize};
#[cfg(feature = "export_to_other_languages")]
use std::sync::Arc;
use crate::rules::next::NextStep;
use async_trait::async_trait;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

/// Use trait for `overloading` methods of `&str` and `&[u8]` types
#[async_trait]
pub trait ValidatorBase<C, IC, D>
where
    C: CartridgeBase<D> + Sync + Debug,
    IC: IntoIterator<Item = C> + Sync + Send,
    D: PartialEq + Eq + Hash + Debug,
{
    fn new(cartridges: IC) -> Self;
    fn validate(&self, data: D) -> Result<(), ValidationErrorIterator>;
    async fn async_validate(&self, data: D) -> Result<(), ValidationErrorIterator>;
}

/// The structure for creating unique validators, load different `cartridges` to validate data.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateValidator<IC, D>
where
    D: PartialEq + Eq + Hash + Debug,
{
    pub(crate) cartridges: IC,
    _phantom: PhantomData<D>,
}

use super::*;
use crate::error::ValidationError;
use crate::message::filling_message;
use futures::StreamExt;

#[async_trait]
impl<'a, C, IC> ValidatorBase<C, IC, &'a [u8]> for TemplateValidator<IC, &'a [u8]>
where
    C: CartridgeBase<&'a [u8]> + Debug + Sync,
    IC: IntoIterator<Item = C> + AsRef<[C]> + Sync + Send,
{
    fn new(cartridges: IC) -> Self {
        Self {
            cartridges,
            _phantom: PhantomData,
        }
    }

    fn validate(&self, data: &'a [u8]) -> Result<(), Vec<ValidationError>> {
        let mut error = Vec::new();
        for cartridge in self.cartridges.as_ref().iter() {
            if let NextStep::Error(extra_with_value) = cartridge.run(data) {
                error.push(ValidationError::new(
                    cartridge.get_id(),
                    filling_message(cartridge.get_message(), extra_with_value),
                ))
            }
        }
        if error.is_empty() {
            Ok(())
        } else {
            Err(error)
        }
    }

    #[allow(clippy::into_iter_on_ref)]
    async fn async_validate(&self, data: &'a [u8]) -> Result<(), Vec<ValidationError>> {
        let error = futures::stream::iter(self.cartridges.as_ref().into_iter())
            .filter_map(|cartridge| async move {
                if let NextStep::Error(extra_with_value) = cartridge.run(data) {
                    Some(ValidationError::new(
                        cartridge.get_id(),
                        filling_message(cartridge.get_message(), extra_with_value),
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .await;

        if error.is_empty() {
            Ok(())
        } else {
            Err(error)
        }
    }
}

#[cfg(feature = "export_to_other_languages")]
#[async_trait]
impl<C, IC> ValidatorBase<C, IC, Arc<[u8]>> for TemplateValidator<IC, Arc<[u8]>>
where
    C: CartridgeBase<Arc<[u8]>> + Debug + Sync,
    IC: IntoIterator<Item = C> + AsRef<[C]> + Sync + Send,
{
    fn new(cartridges: IC) -> Self {
        Self {
            cartridges,
            _phantom: PhantomData,
        }
    }

    fn validate(&self, data: Arc<[u8]>) -> Result<(), Vec<ValidationError>> {
        let mut error = Vec::new();
        for cartridge in self.cartridges.as_ref().iter() {
            if let NextStep::Error(extra_with_value) = cartridge.run(Arc::clone(&data)) {
                error.push(ValidationError::new(
                    cartridge.get_id(),
                    filling_message(cartridge.get_message(), extra_with_value),
                ))
            }
        }
        if error.is_empty() {
            Ok(())
        } else {
            Err(error)
        }
    }

    #[allow(clippy::into_iter_on_ref)]
    async fn async_validate(&self, data: Arc<[u8]>) -> Result<(), Vec<ValidationError>> {
        let error = futures::stream::iter(self.cartridges.as_ref().into_iter())
            .filter_map(|cartridge| {
                let data_clone = Arc::clone(&data);
                async move {
                    if let NextStep::Error(extra_with_value) = cartridge.run(data_clone) {
                        Some(ValidationError::new(
                            cartridge.get_id(),
                            filling_message(cartridge.get_message(), extra_with_value),
                        ))
                    } else {
                        None
                    }
                }
            })
            .collect::<Vec<_>>()
            .await;

        if error.is_empty() {
            Ok(())
        } else {
            Err(error)
        }
    }
}

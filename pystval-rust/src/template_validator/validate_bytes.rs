use super::*;
use crate::error::{traits::ValidationError, BaseValidationError};
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

    fn validate(&self, data: &'a [u8]) -> Result<(), ValidationErrorIterator> {
        let mut error = Vec::new();
        for cartridge in self.cartridges.as_ref().iter() {
            if let NextStep::Error(extra_with_value) = cartridge.run(data) {
                error.push(Box::new(BaseValidationError::new(
                    cartridge.get_id(),
                    filling_message(cartridge.get_message(), extra_with_value),
                )) as Box<dyn ValidationError + Send>);
            }
        }
        if error.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrorIterator::new(error))
        }
    }

    async fn async_validate(&self, data: &'a [u8]) -> Result<(), ValidationErrorIterator> {
        let error = futures::stream::iter(self.cartridges.as_ref().into_iter())
            .filter_map(|cartridge| async move {
                if let NextStep::Error(extra_with_value) = cartridge.run(&data) {
                    Some(Box::new(BaseValidationError::new(
                        cartridge.get_id(),
                        filling_message(cartridge.get_message(), extra_with_value),
                    )) as Box<dyn ValidationError + Send>)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .await;

        if error.is_empty() {
            Ok(())
        } else {
            Err(ValidationErrorIterator::new(error))
        }
    }
}

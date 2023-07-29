use super::*;

impl<'a, C, IC> ValidatorBase<Rule, C, IC, &'a str> for TemplateValidator<IC>
where
    C: CartridgeBase<Rule, &'a str> + Debug,
    IC: IntoIterator<Item = C> + AsRef<[C]>,
{
    fn new(cartridges: IC) -> Self {
        Self { cartridges }
    }

    fn validate(&self, data: &'a str) -> Result<(), Vec<PystvalError>> {
        trace!("{:#?}", &self.cartridges.as_ref().iter());
        let mut error = Vec::new();
        for cartridge in self.cartridges.as_ref().iter() {
            if let NextStep::Error(extra_with_value) = cartridge.run(data) {
                error.push(PystvalError::new(
                    cartridge.get_id(),
                    filling_message(cartridge.get_message(), extra_with_value),
                ));
            }
        }
        if error.is_empty() {
            Ok(())
        } else {
            Err(error)
        }
    }
}

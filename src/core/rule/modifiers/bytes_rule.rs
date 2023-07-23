use super::*;

impl Rule {
    /// The method to change regular expression behavior to byte checks `&[u8]`
    pub fn for_bytes(&mut self) -> Self {
        self.content_mut_unchecked().str_with_type = RegexRaw::RegexBytes(
            self.content_unchecked()
                .str_with_type
                .as_ref()
                .to_string()
                .into_boxed_str(),
        );
        std::mem::take(self)
    }
}

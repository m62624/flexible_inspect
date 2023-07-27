// use super::cartridges::CartridgeBase;
// use crate::core::rules::CaptureData;
// use std::hash::Hash;

// pub trait PystvalValidator<C>
// where
//     C: PartialEq + Eq + Hash,
// {
//     type cartridges: CartridgeBase;
//     type RuleCollection: CartridgeBase<RuleCollection = Self::RuleCollection>;
//     type RuleType: CartridgeBase<RuleType = Self::RuleType>;
//     fn template_validator<T: IntoIterator<Item = Self::cartridges>>(cartridges: T) -> Self;
//     fn validate(
//         &self,
//         text: C,
//     ) -> Box<dyn CartridgeBase<RuleCollection = Self::RuleCollection, RuleType = Self::RuleType>>;
// }

// pub struct TempplateValidator<T: IntoIterator<Item = T>> {
//     cartridges: T,
// }

// impl<T, C> PystvalValidator<C> for TempplateValidator<T>
// where
//     T: CartridgeBase + IntoIterator<Item = T> + FromIterator<T>,
//     C: PartialEq + Eq + Hash,
// {
//     type cartridges = T;
//     type RuleCollection = T;
//     type RuleType = T;

//     fn template_validator<R: IntoIterator<Item = Self::cartridges>>(cartridges: R) -> Self {
//         Self {
//             cartridges: cartridges.into_iter().collect(),
//         }
//     }

//     fn validate(&self, text: C) {
//         &self.cartridges;
//     }
// }

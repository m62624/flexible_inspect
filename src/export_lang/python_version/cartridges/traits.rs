pub trait PyCartridgeBase {
    type CartridgeType;
    fn to_rust(&mut self) -> Self::CartridgeType;
}

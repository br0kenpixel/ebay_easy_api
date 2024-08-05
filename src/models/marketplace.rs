use derive_more::Display;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub enum Marketplace {
    #[default]
    #[display(fmt = "EBAY_US")]
    UnitedStates,
    #[display(fmt = "EBAY_DE")]
    Germany,
    #[display(fmt = "EBAY_IT")]
    Italy,
    #[display(fmt = "EBAY_IR")]
    Ireland,
    #[display(fmt = "EBAY_SG")]
    Singapore,
    #[display(fmt = "EBAY_UK")]
    UnitedKingdom,
    #[display(fmt = "EBAY_FR")]
    France,
}

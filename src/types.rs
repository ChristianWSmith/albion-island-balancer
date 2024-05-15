use clap::ValueEnum;

#[derive(PartialEq, ValueEnum, Clone, Debug)]
pub enum PremiumStatus {
    Premium,
    Free,
}

#[derive(PartialEq, Copy, ValueEnum, Clone, Debug)]
pub enum Product {
    MinorEnergyPotion,
    MinorHealingPotion,
    MinorGigantifyPotion,
    MinorResistancePotion,
    MinorStickyPotion,
    MinorPoisonPotion,
    EnergyPotion,
    HealingPotion,
    GigantifyPotion,
    ResistancePotion,
    StickyPotion,
    PoisonPotion,
    MajorEnergyPotion,
    MajorHealingPotion,
    MajorGigantifyPotion,
    MajorResistancePotion,
    MajorStickyPotion,
    MajorPoisonPotion,
    InvisibilityPotion,
    ChickenOmelette,
    GooseOmelette,
    PorkOmelette,
    BeanSalad,
    TurnipSalad,
    PotatoSalad,
    GoatSandwich,
    MuttonSandwich,
    BeefSandwich,
    CarrotSoup,
    WheatSoup,
    CabbageSoup,
    GoatStew,
    MuttonStew,
    BeefStew,
    RoastChicken,
    RoastGoose,
    RoastPork,
}

pub struct ModelContext {
    pub brecilien_plots: f64,
    pub bridgewatch_plots: f64,
    pub caerleon_plots: f64,
    pub fort_sterling_plots: f64,
    pub lymhurst_plots: f64,
    pub martlock_plots: f64,
    pub thetford_plots: f64,
    pub premium_factor: f64,
    pub target: Product
}
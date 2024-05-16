use clap::ValueEnum;
use std::fmt;
use regex::Regex;

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
    pub target: Product,
}

#[derive(Debug)]
pub struct PlotPlan {
    pub output: f64,

    pub herb_gardens_brecilien: f64,
    pub farms_brecilien: f64,
    pub pastures_brecilien: f64,
    pub herb_gardens_bridgewatch: f64,
    pub farms_bridgewatch: f64,
    pub pastures_bridgewatch: f64,
    pub herb_gardens_caerleon: f64,
    pub farms_caerleon: f64,
    pub pastures_caerleon: f64,
    pub herb_gardens_fort_sterling: f64,
    pub farms_fort_sterling: f64,
    pub pastures_fort_sterling: f64,
    pub herb_gardens_lymhurst: f64,
    pub farms_lymhurst: f64,
    pub pastures_lymhurst: f64,
    pub herb_gardens_martlock: f64,
    pub farms_martlock: f64,
    pub pastures_martlock: f64,
    pub herb_gardens_thetford: f64,
    pub farms_thetford: f64,
    pub pastures_thetford: f64,
    
    pub agaric_tiles_brecilien: f64,
    pub comfrey_tiles_brecilien: f64,
    pub burdock_tiles_brecilien: f64,
    pub teasel_tiles_brecilien: f64,
    pub foxglove_tiles_brecilien: f64,
    pub muellin_tiles_brecilien: f64,
    pub yarrow_tiles_brecilien: f64,
    pub carrot_tiles_brecilien: f64,
    pub bean_tiles_brecilien: f64,
    pub wheat_tiles_brecilien: f64,
    pub turnip_tiles_brecilien: f64,
    pub cabbage_tiles_brecilien: f64,
    pub potato_tiles_brecilien: f64,
    pub corn_tiles_brecilien: f64,
    pub pumpkin_tiles_brecilien: f64,
    pub baby_chicken_tiles_brecilien: f64,
    pub kid_tiles_brecilien: f64,
    pub gosling_tiles_brecilien: f64,
    pub lamb_tiles_brecilien: f64,
    pub piglet_tiles_brecilien: f64,
    pub calf_tiles_brecilien: f64,
    pub chicken_tiles_brecilien: f64,
    pub goat_tiles_brecilien: f64,
    pub goose_tiles_brecilien: f64,
    pub sheep_tiles_brecilien: f64,
    pub pig_tiles_brecilien: f64,
    pub cow_tiles_brecilien: f64,
    pub agaric_tiles_bridgewatch: f64,
    pub comfrey_tiles_bridgewatch: f64,
    pub burdock_tiles_bridgewatch: f64,
    pub teasel_tiles_bridgewatch: f64,
    pub foxglove_tiles_bridgewatch: f64,
    pub muellin_tiles_bridgewatch: f64,
    pub yarrow_tiles_bridgewatch: f64,
    pub carrot_tiles_bridgewatch: f64,
    pub bean_tiles_bridgewatch: f64,
    pub wheat_tiles_bridgewatch: f64,
    pub turnip_tiles_bridgewatch: f64,
    pub cabbage_tiles_bridgewatch: f64,
    pub potato_tiles_bridgewatch: f64,
    pub corn_tiles_bridgewatch: f64,
    pub pumpkin_tiles_bridgewatch: f64,
    pub baby_chicken_tiles_bridgewatch: f64,
    pub kid_tiles_bridgewatch: f64,
    pub gosling_tiles_bridgewatch: f64,
    pub lamb_tiles_bridgewatch: f64,
    pub piglet_tiles_bridgewatch: f64,
    pub calf_tiles_bridgewatch: f64,
    pub chicken_tiles_bridgewatch: f64,
    pub goat_tiles_bridgewatch: f64,
    pub goose_tiles_bridgewatch: f64,
    pub sheep_tiles_bridgewatch: f64,
    pub pig_tiles_bridgewatch: f64,
    pub cow_tiles_bridgewatch: f64,
    pub agaric_tiles_caerleon: f64,
    pub comfrey_tiles_caerleon: f64,
    pub burdock_tiles_caerleon: f64,
    pub teasel_tiles_caerleon: f64,
    pub foxglove_tiles_caerleon: f64,
    pub muellin_tiles_caerleon: f64,
    pub yarrow_tiles_caerleon: f64,
    pub carrot_tiles_caerleon: f64,
    pub bean_tiles_caerleon: f64,
    pub wheat_tiles_caerleon: f64,
    pub turnip_tiles_caerleon: f64,
    pub cabbage_tiles_caerleon: f64,
    pub potato_tiles_caerleon: f64,
    pub corn_tiles_caerleon: f64,
    pub pumpkin_tiles_caerleon: f64,
    pub baby_chicken_tiles_caerleon: f64,
    pub kid_tiles_caerleon: f64,
    pub gosling_tiles_caerleon: f64,
    pub lamb_tiles_caerleon: f64,
    pub piglet_tiles_caerleon: f64,
    pub calf_tiles_caerleon: f64,
    pub chicken_tiles_caerleon: f64,
    pub goat_tiles_caerleon: f64,
    pub goose_tiles_caerleon: f64,
    pub sheep_tiles_caerleon: f64,
    pub pig_tiles_caerleon: f64,
    pub cow_tiles_caerleon: f64,
    pub agaric_tiles_fort_sterling: f64,
    pub comfrey_tiles_fort_sterling: f64,
    pub burdock_tiles_fort_sterling: f64,
    pub teasel_tiles_fort_sterling: f64,
    pub foxglove_tiles_fort_sterling: f64,
    pub muellin_tiles_fort_sterling: f64,
    pub yarrow_tiles_fort_sterling: f64,
    pub carrot_tiles_fort_sterling: f64,
    pub bean_tiles_fort_sterling: f64,
    pub wheat_tiles_fort_sterling: f64,
    pub turnip_tiles_fort_sterling: f64,
    pub cabbage_tiles_fort_sterling: f64,
    pub potato_tiles_fort_sterling: f64,
    pub corn_tiles_fort_sterling: f64,
    pub pumpkin_tiles_fort_sterling: f64,
    pub baby_chicken_tiles_fort_sterling: f64,
    pub kid_tiles_fort_sterling: f64,
    pub gosling_tiles_fort_sterling: f64,
    pub lamb_tiles_fort_sterling: f64,
    pub piglet_tiles_fort_sterling: f64,
    pub calf_tiles_fort_sterling: f64,
    pub chicken_tiles_fort_sterling: f64,
    pub goat_tiles_fort_sterling: f64,
    pub goose_tiles_fort_sterling: f64,
    pub sheep_tiles_fort_sterling: f64,
    pub pig_tiles_fort_sterling: f64,
    pub cow_tiles_fort_sterling: f64,
    pub agaric_tiles_lymhurst: f64,
    pub comfrey_tiles_lymhurst: f64,
    pub burdock_tiles_lymhurst: f64,
    pub teasel_tiles_lymhurst: f64,
    pub foxglove_tiles_lymhurst: f64,
    pub muellin_tiles_lymhurst: f64,
    pub yarrow_tiles_lymhurst: f64,
    pub carrot_tiles_lymhurst: f64,
    pub bean_tiles_lymhurst: f64,
    pub wheat_tiles_lymhurst: f64,
    pub turnip_tiles_lymhurst: f64,
    pub cabbage_tiles_lymhurst: f64,
    pub potato_tiles_lymhurst: f64,
    pub corn_tiles_lymhurst: f64,
    pub pumpkin_tiles_lymhurst: f64,
    pub baby_chicken_tiles_lymhurst: f64,
    pub kid_tiles_lymhurst: f64,
    pub gosling_tiles_lymhurst: f64,
    pub lamb_tiles_lymhurst: f64,
    pub piglet_tiles_lymhurst: f64,
    pub calf_tiles_lymhurst: f64,
    pub chicken_tiles_lymhurst: f64,
    pub goat_tiles_lymhurst: f64,
    pub goose_tiles_lymhurst: f64,
    pub sheep_tiles_lymhurst: f64,
    pub pig_tiles_lymhurst: f64,
    pub cow_tiles_lymhurst: f64,
    pub agaric_tiles_martlock: f64,
    pub comfrey_tiles_martlock: f64,
    pub burdock_tiles_martlock: f64,
    pub teasel_tiles_martlock: f64,
    pub foxglove_tiles_martlock: f64,
    pub muellin_tiles_martlock: f64,
    pub yarrow_tiles_martlock: f64,
    pub carrot_tiles_martlock: f64,
    pub bean_tiles_martlock: f64,
    pub wheat_tiles_martlock: f64,
    pub turnip_tiles_martlock: f64,
    pub cabbage_tiles_martlock: f64,
    pub potato_tiles_martlock: f64,
    pub corn_tiles_martlock: f64,
    pub pumpkin_tiles_martlock: f64,
    pub baby_chicken_tiles_martlock: f64,
    pub kid_tiles_martlock: f64,
    pub gosling_tiles_martlock: f64,
    pub lamb_tiles_martlock: f64,
    pub piglet_tiles_martlock: f64,
    pub calf_tiles_martlock: f64,
    pub chicken_tiles_martlock: f64,
    pub goat_tiles_martlock: f64,
    pub goose_tiles_martlock: f64,
    pub sheep_tiles_martlock: f64,
    pub pig_tiles_martlock: f64,
    pub cow_tiles_martlock: f64,
    pub agaric_tiles_thetford: f64,
    pub comfrey_tiles_thetford: f64,
    pub burdock_tiles_thetford: f64,
    pub teasel_tiles_thetford: f64,
    pub foxglove_tiles_thetford: f64,
    pub muellin_tiles_thetford: f64,
    pub yarrow_tiles_thetford: f64,
    pub carrot_tiles_thetford: f64,
    pub bean_tiles_thetford: f64,
    pub wheat_tiles_thetford: f64,
    pub turnip_tiles_thetford: f64,
    pub cabbage_tiles_thetford: f64,
    pub potato_tiles_thetford: f64,
    pub corn_tiles_thetford: f64,
    pub pumpkin_tiles_thetford: f64,
    pub baby_chicken_tiles_thetford: f64,
    pub kid_tiles_thetford: f64,
    pub gosling_tiles_thetford: f64,
    pub lamb_tiles_thetford: f64,
    pub piglet_tiles_thetford: f64,
    pub calf_tiles_thetford: f64,
    pub chicken_tiles_thetford: f64,
    pub goat_tiles_thetford: f64,
    pub goose_tiles_thetford: f64,
    pub sheep_tiles_thetford: f64,
    pub pig_tiles_thetford: f64,
    pub cow_tiles_thetford: f64,
}

impl PlotPlan {
    pub fn display(&self) -> String {
        // Use the default Debug implementation to format the struct
        let formatted = format!("{:?}", self);

        // Define a regex to match fields with value 0.0
        let re = Regex::new(r",\s*\w+: -?0\.0").unwrap();

        // Remove fields with value 0.0
        let result = re.replace_all(&formatted, "");

        let re = Regex::new(r"\{").unwrap();
        let result = re.replace_all(&result, "{\n    ");

        let re = Regex::new(r",").unwrap();
        let result = re.replace_all(&result, ",\n    ");

        let re = Regex::new(r"\}").unwrap();
        let result = re.replace_all(&result, "\n}");


        // Return the cleaned string
        result.to_string()
    }
}
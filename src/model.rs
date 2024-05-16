use linprog::{
    Model,
    Objective,
    Summand,
    Operator,
    Var
};

use crate::{constants::*, types::Product, ModelContext};


pub fn optimize_plots(context: ModelContext) -> Model {
    let mut model = Model::new("Plots", Objective::Max);

    let plots_lymhurst = model.reg_var(0.0);
    let herb_gardens_lymhurst = model.reg_var(0.0);
    let farms_lymhurst = model.reg_var(0.0);
    let pastures_lymhurst = model.reg_var(0.0);
    let herb_garden_tiles_lymhurst = model.reg_var(0.0);
    let farm_tiles_lymhurst = model.reg_var(0.0);
    let pasture_tiles_lymhurst = model.reg_var(0.0);
    let teasel_tiles_lymhurst = model.reg_var(0.0);
    let muellin_tiles_lymhurst = model.reg_var(0.0);
    let yarrow_tiles_lymhurst = model.reg_var(0.0);
    let pumpkin_tiles_lymhurst = model.reg_var(0.0);
    let cow_tiles_lymhurst = model.reg_var(0.0);
    let teasel_herbs = model.reg_var(0.0);
    let muellin_herbs = model.reg_var(0.0);
    let yarrow_herbs = model.reg_var(0.0);
    let pumpkin_crops = model.reg_var(0.0);
    let cows_milk = model.reg_var(0.0);
    let major_poison_potion = model.reg_var(if context.target == Product::MajorPoisonPotion { 1.0 } else { 0.0 });

    model.reg_constr(vec![Summand(1.0, &plots_lymhurst)], Operator::E, context.lymhurst_plots);
    model.reg_constr(vec![Summand(1.0, &herb_gardens_lymhurst), Summand(1.0, &farms_lymhurst), Summand(1.0, &pastures_lymhurst), Summand(-1.0, &plots_lymhurst)], Operator::E, 0.0);
    model.reg_constr(vec![Summand(TILES_PER_PLOT, &herb_gardens_lymhurst), Summand(-1.0, &herb_garden_tiles_lymhurst)], Operator::E, 0.0);
    model.reg_constr(vec![Summand(TILES_PER_PLOT, &farms_lymhurst), Summand(-1.0, &farm_tiles_lymhurst)], Operator::E, 0.0);
    model.reg_constr(vec![Summand(TILES_PER_PLOT, &pastures_lymhurst), Summand(-1.0, &pasture_tiles_lymhurst)], Operator::E, 0.0);
    model.reg_constr(vec![Summand(1.0, &yarrow_tiles_lymhurst), Summand(1.0, &muellin_tiles_lymhurst), Summand(1.0, &teasel_tiles_lymhurst)], Operator::Le, 0.0);
    model.reg_constr(vec![Summand(1.0, &cow_tiles_lymhurst), Summand(-1.0, &pasture_tiles_lymhurst)], Operator::Le, 0.0);
    model.reg_constr(vec![Summand(1.0, &pumpkin_tiles_lymhurst), Summand(-1.0, &farm_tiles_lymhurst)], Operator::Le, 0.0);
    model.reg_constr(vec![Summand(1.0, &teasel_herbs), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &teasel_tiles_lymhurst)], Operator::E, 0.0);
    model.reg_constr(vec![Summand(1.0, &muellin_herbs), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &muellin_tiles_lymhurst)], Operator::E, 0.0);
    model.reg_constr(vec![Summand(1.0, &yarrow_herbs), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &yarrow_tiles_lymhurst)], Operator::E, 0.0);
    model.reg_constr(vec![Summand(1.0, &pumpkin_crops), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &pumpkin_tiles_lymhurst), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &cow_tiles_lymhurst)], Operator::E, 0.0);
    model.reg_constr(vec![Summand(1.0, &cows_milk), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &cow_tiles_lymhurst)], Operator::E, 0.0);
    model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &major_poison_potion), Summand(-1.0 * L11_INGREDIENT, &yarrow_herbs), Summand(-1.0 * L09_INGREDIENT, &muellin_herbs), Summand(-1.0 * L09_INGREDIENT, &teasel_herbs), Summand(-1.0 * L07_INGREDIENT, &cows_milk), Summand(-1.0 * L07_INGREDIENT, &pumpkin_crops)], Operator::E, 0.0);
    
    model.optimize();
    
    model
}

// pub fn optimize_plots(context: ModelContext) -> Model {
//     let mut model = Model::new("Plots", Objective::Max);
    
//     // variables
//     //// plots
//     let plots_brecilien = model.reg_var(0.0);
//     let plots_bridgewatch = model.reg_var(0.0);
//     let plots_caerleon = model.reg_var(0.0);
//     let plots_fort_sterling = model.reg_var(0.0);
//     let plots_lymhurst = model.reg_var(0.0);
//     let plots_martlock = model.reg_var(0.0);
//     let plots_thetford = model.reg_var(0.0);
    
//     //// used plots
//     let herb_gardens_brecilien = model.reg_var(0.0);
//     let farms_brecilien = model.reg_var(0.0);
//     let pastures_brecilien = model.reg_var(0.0);
//     let herb_gardens_bridgewatch = model.reg_var(0.0);
//     let farms_bridgewatch = model.reg_var(0.0);
//     let pastures_bridgewatch = model.reg_var(0.0);
//     let herb_gardens_caerleon = model.reg_var(0.0);
//     let farms_caerleon = model.reg_var(0.0);
//     let pastures_caerleon = model.reg_var(0.0);
//     let herb_gardens_fort_sterling = model.reg_var(0.0);
//     let farms_fort_sterling = model.reg_var(0.0);
//     let pastures_fort_sterling = model.reg_var(0.0);
//     let herb_gardens_lymhurst = model.reg_var(0.0);
//     let farms_lymhurst = model.reg_var(0.0);
//     let pastures_lymhurst = model.reg_var(0.0);
//     let herb_gardens_martlock = model.reg_var(0.0);
//     let farms_martlock = model.reg_var(0.0);
//     let pastures_martlock = model.reg_var(0.0);
//     let herb_gardens_thetford = model.reg_var(0.0);
//     let farms_thetford = model.reg_var(0.0);
//     let pastures_thetford = model.reg_var(0.0);

//     //// tiles
//     let herb_garden_tiles_brecilien = model.reg_var(0.0);
//     let farm_tiles_brecilien = model.reg_var(0.0);
//     let pasture_tiles_brecilien = model.reg_var(0.0);
//     let herb_garden_tiles_bridgewatch = model.reg_var(0.0);
//     let farm_tiles_bridgewatch = model.reg_var(0.0);
//     let pasture_tiles_bridgewatch = model.reg_var(0.0);
//     let herb_garden_tiles_caerleon = model.reg_var(0.0);
//     let farm_tiles_caerleon = model.reg_var(0.0);
//     let pasture_tiles_caerleon = model.reg_var(0.0);
//     let herb_garden_tiles_fort_sterling = model.reg_var(0.0);
//     let farm_tiles_fort_sterling = model.reg_var(0.0);
//     let pasture_tiles_fort_sterling = model.reg_var(0.0);
//     let herb_garden_tiles_lymhurst = model.reg_var(0.0);
//     let farm_tiles_lymhurst = model.reg_var(0.0);
//     let pasture_tiles_lymhurst = model.reg_var(0.0);
//     let herb_garden_tiles_martlock = model.reg_var(0.0);
//     let farm_tiles_martlock = model.reg_var(0.0);
//     let pasture_tiles_martlock = model.reg_var(0.0);
//     let herb_garden_tiles_thetford = model.reg_var(0.0);
//     let farm_tiles_thetford = model.reg_var(0.0);
//     let pasture_tiles_thetford = model.reg_var(0.0);
//     let agaric_tiles_brecilien = model.reg_var(0.0);
//     let comfrey_tiles_brecilien = model.reg_var(0.0);
//     let burdock_tiles_brecilien = model.reg_var(0.0);
//     let teasel_tiles_brecilien = model.reg_var(0.0);
//     let foxglove_tiles_brecilien = model.reg_var(0.0);
//     let muellin_tiles_brecilien = model.reg_var(0.0);
//     let yarrow_tiles_brecilien = model.reg_var(0.0);
//     let carrot_tiles_brecilien = model.reg_var(0.0);
//     let bean_tiles_brecilien = model.reg_var(0.0);
//     let wheat_tiles_brecilien = model.reg_var(0.0);
//     let turnip_tiles_brecilien = model.reg_var(0.0);
//     let cabbage_tiles_brecilien = model.reg_var(0.0);
//     let potato_tiles_brecilien = model.reg_var(0.0);
//     let corn_tiles_brecilien = model.reg_var(0.0);
//     let pumpkin_tiles_brecilien = model.reg_var(0.0);
//     let baby_chicken_tiles_brecilien = model.reg_var(0.0);
//     let kid_tiles_brecilien = model.reg_var(0.0);
//     let gosling_tiles_brecilien = model.reg_var(0.0);
//     let lamb_tiles_brecilien = model.reg_var(0.0);
//     let piglet_tiles_brecilien = model.reg_var(0.0);
//     let calf_tiles_brecilien = model.reg_var(0.0);
//     let chicken_tiles_brecilien = model.reg_var(0.0);
//     let goat_tiles_brecilien = model.reg_var(0.0);
//     let goose_tiles_brecilien = model.reg_var(0.0);
//     let sheep_tiles_brecilien = model.reg_var(0.0);
//     let pig_tiles_brecilien = model.reg_var(0.0);
//     let cow_tiles_brecilien = model.reg_var(0.0);
//     let agaric_tiles_bridgewatch = model.reg_var(0.0);
//     let comfrey_tiles_bridgewatch = model.reg_var(0.0);
//     let burdock_tiles_bridgewatch = model.reg_var(0.0);
//     let teasel_tiles_bridgewatch = model.reg_var(0.0);
//     let foxglove_tiles_bridgewatch = model.reg_var(0.0);
//     let muellin_tiles_bridgewatch = model.reg_var(0.0);
//     let yarrow_tiles_bridgewatch = model.reg_var(0.0);
//     let carrot_tiles_bridgewatch = model.reg_var(0.0);
//     let bean_tiles_bridgewatch = model.reg_var(0.0);
//     let wheat_tiles_bridgewatch = model.reg_var(0.0);
//     let turnip_tiles_bridgewatch = model.reg_var(0.0);
//     let cabbage_tiles_bridgewatch = model.reg_var(0.0);
//     let potato_tiles_bridgewatch = model.reg_var(0.0);
//     let corn_tiles_bridgewatch = model.reg_var(0.0);
//     let pumpkin_tiles_bridgewatch = model.reg_var(0.0);
//     let baby_chicken_tiles_bridgewatch = model.reg_var(0.0);
//     let kid_tiles_bridgewatch = model.reg_var(0.0);
//     let gosling_tiles_bridgewatch = model.reg_var(0.0);
//     let lamb_tiles_bridgewatch = model.reg_var(0.0);
//     let piglet_tiles_bridgewatch = model.reg_var(0.0);
//     let calf_tiles_bridgewatch = model.reg_var(0.0);
//     let chicken_tiles_bridgewatch = model.reg_var(0.0);
//     let goat_tiles_bridgewatch = model.reg_var(0.0);
//     let goose_tiles_bridgewatch = model.reg_var(0.0);
//     let sheep_tiles_bridgewatch = model.reg_var(0.0);
//     let pig_tiles_bridgewatch = model.reg_var(0.0);
//     let cow_tiles_bridgewatch = model.reg_var(0.0);
//     let agaric_tiles_caerleon = model.reg_var(0.0);
//     let comfrey_tiles_caerleon = model.reg_var(0.0);
//     let burdock_tiles_caerleon = model.reg_var(0.0);
//     let teasel_tiles_caerleon = model.reg_var(0.0);
//     let foxglove_tiles_caerleon = model.reg_var(0.0);
//     let muellin_tiles_caerleon = model.reg_var(0.0);
//     let yarrow_tiles_caerleon = model.reg_var(0.0);
//     let carrot_tiles_caerleon = model.reg_var(0.0);
//     let bean_tiles_caerleon = model.reg_var(0.0);
//     let wheat_tiles_caerleon = model.reg_var(0.0);
//     let turnip_tiles_caerleon = model.reg_var(0.0);
//     let cabbage_tiles_caerleon = model.reg_var(0.0);
//     let potato_tiles_caerleon = model.reg_var(0.0);
//     let corn_tiles_caerleon = model.reg_var(0.0);
//     let pumpkin_tiles_caerleon = model.reg_var(0.0);
//     let baby_chicken_tiles_caerleon = model.reg_var(0.0);
//     let kid_tiles_caerleon = model.reg_var(0.0);
//     let gosling_tiles_caerleon = model.reg_var(0.0);
//     let lamb_tiles_caerleon = model.reg_var(0.0);
//     let piglet_tiles_caerleon = model.reg_var(0.0);
//     let calf_tiles_caerleon = model.reg_var(0.0);
//     let chicken_tiles_caerleon = model.reg_var(0.0);
//     let goat_tiles_caerleon = model.reg_var(0.0);
//     let goose_tiles_caerleon = model.reg_var(0.0);
//     let sheep_tiles_caerleon = model.reg_var(0.0);
//     let pig_tiles_caerleon = model.reg_var(0.0);
//     let cow_tiles_caerleon = model.reg_var(0.0);
//     let agaric_tiles_fort_sterling = model.reg_var(0.0);
//     let comfrey_tiles_fort_sterling = model.reg_var(0.0);
//     let burdock_tiles_fort_sterling = model.reg_var(0.0);
//     let teasel_tiles_fort_sterling = model.reg_var(0.0);
//     let foxglove_tiles_fort_sterling = model.reg_var(0.0);
//     let muellin_tiles_fort_sterling = model.reg_var(0.0);
//     let yarrow_tiles_fort_sterling = model.reg_var(0.0);
//     let carrot_tiles_fort_sterling = model.reg_var(0.0);
//     let bean_tiles_fort_sterling = model.reg_var(0.0);
//     let wheat_tiles_fort_sterling = model.reg_var(0.0);
//     let turnip_tiles_fort_sterling = model.reg_var(0.0);
//     let cabbage_tiles_fort_sterling = model.reg_var(0.0);
//     let potato_tiles_fort_sterling = model.reg_var(0.0);
//     let corn_tiles_fort_sterling = model.reg_var(0.0);
//     let pumpkin_tiles_fort_sterling = model.reg_var(0.0);
//     let baby_chicken_tiles_fort_sterling = model.reg_var(0.0);
//     let kid_tiles_fort_sterling = model.reg_var(0.0);
//     let gosling_tiles_fort_sterling = model.reg_var(0.0);
//     let lamb_tiles_fort_sterling = model.reg_var(0.0);
//     let piglet_tiles_fort_sterling = model.reg_var(0.0);
//     let calf_tiles_fort_sterling = model.reg_var(0.0);
//     let chicken_tiles_fort_sterling = model.reg_var(0.0);
//     let goat_tiles_fort_sterling = model.reg_var(0.0);
//     let goose_tiles_fort_sterling = model.reg_var(0.0);
//     let sheep_tiles_fort_sterling = model.reg_var(0.0);
//     let pig_tiles_fort_sterling = model.reg_var(0.0);
//     let cow_tiles_fort_sterling = model.reg_var(0.0);
//     let agaric_tiles_lymhurst = model.reg_var(0.0);
//     let comfrey_tiles_lymhurst = model.reg_var(0.0);
//     let burdock_tiles_lymhurst = model.reg_var(0.0);
//     let teasel_tiles_lymhurst = model.reg_var(0.0);
//     let foxglove_tiles_lymhurst = model.reg_var(0.0);
//     let muellin_tiles_lymhurst = model.reg_var(0.0);
//     let yarrow_tiles_lymhurst = model.reg_var(0.0);
//     let carrot_tiles_lymhurst = model.reg_var(0.0);
//     let bean_tiles_lymhurst = model.reg_var(0.0);
//     let wheat_tiles_lymhurst = model.reg_var(0.0);
//     let turnip_tiles_lymhurst = model.reg_var(0.0);
//     let cabbage_tiles_lymhurst = model.reg_var(0.0);
//     let potato_tiles_lymhurst = model.reg_var(0.0);
//     let corn_tiles_lymhurst = model.reg_var(0.0);
//     let pumpkin_tiles_lymhurst = model.reg_var(0.0);
//     let baby_chicken_tiles_lymhurst = model.reg_var(0.0);
//     let kid_tiles_lymhurst = model.reg_var(0.0);
//     let gosling_tiles_lymhurst = model.reg_var(0.0);
//     let lamb_tiles_lymhurst = model.reg_var(0.0);
//     let piglet_tiles_lymhurst = model.reg_var(0.0);
//     let calf_tiles_lymhurst = model.reg_var(0.0);
//     let chicken_tiles_lymhurst = model.reg_var(0.0);
//     let goat_tiles_lymhurst = model.reg_var(0.0);
//     let goose_tiles_lymhurst = model.reg_var(0.0);
//     let sheep_tiles_lymhurst = model.reg_var(0.0);
//     let pig_tiles_lymhurst = model.reg_var(0.0);
//     let cow_tiles_lymhurst = model.reg_var(0.0);
//     let agaric_tiles_martlock = model.reg_var(0.0);
//     let comfrey_tiles_martlock = model.reg_var(0.0);
//     let burdock_tiles_martlock = model.reg_var(0.0);
//     let teasel_tiles_martlock = model.reg_var(0.0);
//     let foxglove_tiles_martlock = model.reg_var(0.0);
//     let muellin_tiles_martlock = model.reg_var(0.0);
//     let yarrow_tiles_martlock = model.reg_var(0.0);
//     let carrot_tiles_martlock = model.reg_var(0.0);
//     let bean_tiles_martlock = model.reg_var(0.0);
//     let wheat_tiles_martlock = model.reg_var(0.0);
//     let turnip_tiles_martlock = model.reg_var(0.0);
//     let cabbage_tiles_martlock = model.reg_var(0.0);
//     let potato_tiles_martlock = model.reg_var(0.0);
//     let corn_tiles_martlock = model.reg_var(0.0);
//     let pumpkin_tiles_martlock = model.reg_var(0.0);
//     let baby_chicken_tiles_martlock = model.reg_var(0.0);
//     let kid_tiles_martlock = model.reg_var(0.0);
//     let gosling_tiles_martlock = model.reg_var(0.0);
//     let lamb_tiles_martlock = model.reg_var(0.0);
//     let piglet_tiles_martlock = model.reg_var(0.0);
//     let calf_tiles_martlock = model.reg_var(0.0);
//     let chicken_tiles_martlock = model.reg_var(0.0);
//     let goat_tiles_martlock = model.reg_var(0.0);
//     let goose_tiles_martlock = model.reg_var(0.0);
//     let sheep_tiles_martlock = model.reg_var(0.0);
//     let pig_tiles_martlock = model.reg_var(0.0);
//     let cow_tiles_martlock = model.reg_var(0.0);
//     let agaric_tiles_thetford = model.reg_var(0.0);
//     let comfrey_tiles_thetford = model.reg_var(0.0);
//     let burdock_tiles_thetford = model.reg_var(0.0);
//     let teasel_tiles_thetford = model.reg_var(0.0);
//     let foxglove_tiles_thetford = model.reg_var(0.0);
//     let muellin_tiles_thetford = model.reg_var(0.0);
//     let yarrow_tiles_thetford = model.reg_var(0.0);
//     let carrot_tiles_thetford = model.reg_var(0.0);
//     let bean_tiles_thetford = model.reg_var(0.0);
//     let wheat_tiles_thetford = model.reg_var(0.0);
//     let turnip_tiles_thetford = model.reg_var(0.0);
//     let cabbage_tiles_thetford = model.reg_var(0.0);
//     let potato_tiles_thetford = model.reg_var(0.0);
//     let corn_tiles_thetford = model.reg_var(0.0);
//     let pumpkin_tiles_thetford = model.reg_var(0.0);
//     let baby_chicken_tiles_thetford = model.reg_var(0.0);
//     let kid_tiles_thetford = model.reg_var(0.0);
//     let gosling_tiles_thetford = model.reg_var(0.0);
//     let lamb_tiles_thetford = model.reg_var(0.0);
//     let piglet_tiles_thetford = model.reg_var(0.0);
//     let calf_tiles_thetford = model.reg_var(0.0);
//     let chicken_tiles_thetford = model.reg_var(0.0);
//     let goat_tiles_thetford = model.reg_var(0.0);
//     let goose_tiles_thetford = model.reg_var(0.0);
//     let sheep_tiles_thetford = model.reg_var(0.0);
//     let pig_tiles_thetford = model.reg_var(0.0);
//     let cow_tiles_thetford = model.reg_var(0.0);

//     //// yields
//     let agaric_herbs = model.reg_var(0.0);
//     let comfrey_herbs = model.reg_var(0.0);
//     let burdock_herbs = model.reg_var(0.0);
//     let teasel_herbs = model.reg_var(0.0);
//     let foxglove_herbs = model.reg_var(0.0);
//     let muellin_herbs = model.reg_var(0.0);
//     let yarrow_herbs = model.reg_var(0.0);
//     let carrot_crops = model.reg_var(0.0);
//     let bean_crops = model.reg_var(0.0);
//     let wheat_crops = model.reg_var(0.0);
//     let turnip_crops = model.reg_var(0.0);
//     let cabbage_crops = model.reg_var(0.0);
//     let potato_crops = model.reg_var(0.0);
//     let corn_crops = model.reg_var(0.0);
//     let pumpkin_crops = model.reg_var(0.0);
//     let hen_eggs = model.reg_var(0.0);
//     let goats_milk = model.reg_var(0.0);
//     let goose_eggs = model.reg_var(0.0);
//     let sheeps_milk = model.reg_var(0.0);
//     let cows_milk = model.reg_var(0.0);
//     let raw_chicken = model.reg_var(0.0);
//     let raw_goat = model.reg_var(0.0);
//     let raw_goose = model.reg_var(0.0);
//     let raw_mutton = model.reg_var(0.0);
//     let raw_pork = model.reg_var(0.0);
//     let raw_beef = model.reg_var(0.0);

//     //// products
//     let minor_energy_potion = model.reg_var(if context.target == Product::MinorEnergyPotion { 1.0 } else { if context.target == Product::MinorHealingPotion { 1.0 } else { 0.0 } });
//     let minor_healing_potion = model.reg_var(if context.target == Product::MinorHealingPotion { 1.0 } else { 0.0 });
//     let minor_gigantify_potion = model.reg_var(if context.target == Product::MinorGigantifyPotion { 1.0 } else { 0.0 });
//     let minor_resistance_potion = model.reg_var(if context.target == Product::MinorResistancePotion { 1.0 } else { 0.0 });
//     let minor_sticky_potion = model.reg_var(if context.target == Product::MinorStickyPotion { 1.0 } else { 0.0 });
//     let minor_poison_potion = model.reg_var(if context.target == Product::MinorPoisonPotion { 1.0 } else { 0.0 });
//     let energy_potion = model.reg_var(if context.target == Product::EnergyPotion { 1.0 } else { 0.0 });
//     let healing_potion = model.reg_var(if context.target == Product::HealingPotion { 1.0 } else { 0.0 });
//     let gigantify_potion = model.reg_var(if context.target == Product::GigantifyPotion { 1.0 } else { 0.0 });
//     let resistance_potion = model.reg_var(if context.target == Product::ResistancePotion { 1.0 } else { 0.0 });
//     let sticky_potion = model.reg_var(if context.target == Product::StickyPotion { 1.0 } else { 0.0 });
//     let poison_potion = model.reg_var(if context.target == Product::PoisonPotion { 1.0 } else { 0.0 });
//     let major_energy_potion = model.reg_var(if context.target == Product::MajorEnergyPotion { 1.0 } else { 0.0 });
//     let major_healing_potion = model.reg_var(if context.target == Product::MajorHealingPotion { 1.0 } else { 0.0 });
//     let major_gigantify_potion = model.reg_var(if context.target == Product::MajorGigantifyPotion { 1.0 } else { 0.0 });
//     let major_resistance_potion = model.reg_var(if context.target == Product::MajorResistancePotion { 1.0 } else { 0.0 });
//     let major_sticky_potion = model.reg_var(if context.target == Product::MajorStickyPotion { 1.0 } else { 0.0 });
//     let major_poison_potion = model.reg_var(if context.target == Product::MajorPoisonPotion { 1.0 } else { 0.0 });
//     let invisibility_potion = model.reg_var(if context.target == Product::InvisibilityPotion { 1.0 } else { 0.0 });
//     let chicken_omelette = model.reg_var(if context.target == Product::ChickenOmelette { 1.0 } else { 0.0 });
//     let goose_omelette = model.reg_var(if context.target == Product::GooseOmelette { 1.0 } else { 0.0 });
//     let pork_omelette = model.reg_var(if context.target == Product::PorkOmelette { 1.0 } else { 0.0 });
//     let bean_salad = model.reg_var(if context.target == Product::BeanSalad { 1.0 } else { 0.0 });
//     let turnip_salad = model.reg_var(if context.target == Product::TurnipSalad { 1.0 } else { 0.0 });
//     let potato_salad = model.reg_var(if context.target == Product::PotatoSalad { 1.0 } else { 0.0 });
//     let goat_sandwich = model.reg_var(if context.target == Product::GoatSandwich { 1.0 } else { 0.0 });
//     let mutton_sandwich = model.reg_var(if context.target == Product::MuttonSandwich { 1.0 } else { 0.0 });
//     let beef_sandwich = model.reg_var(if context.target == Product::BeefSandwich { 1.0 } else { 0.0 });
//     let carrot_soup = model.reg_var(if context.target == Product::CarrotSoup { 1.0 } else { 0.0 });
//     let wheat_soup = model.reg_var(if context.target == Product::WheatSoup { 1.0 } else { 0.0 });
//     let cabbage_soup = model.reg_var(if context.target == Product::CabbageSoup { 1.0 } else { 0.0 });
//     let goat_stew = model.reg_var(if context.target == Product::GoatStew { 1.0 } else { 0.0 });
//     let mutton_stew = model.reg_var(if context.target == Product::MuttonStew { 1.0 } else { 0.0 });
//     let beef_stew = model.reg_var(if context.target == Product::BeefStew { 1.0 } else { 0.0 });
//     let roast_chicken = model.reg_var(if context.target == Product::RoastChicken { 1.0 } else { 0.0 });
//     let roast_goose = model.reg_var(if context.target == Product::RoastGoose { 1.0 } else { 0.0 });
//     let roast_pork = model.reg_var(if context.target == Product::RoastPork { 1.0 } else { 0.0 });

    
//     // constraints
//     //// argument constraints
//     model.reg_constr(vec![Summand(1.0, &plots_brecilien)], Operator::E, context.brecilien_plots);
//     model.reg_constr(vec![Summand(1.0, &plots_bridgewatch)], Operator::E, context.bridgewatch_plots);
//     model.reg_constr(vec![Summand(1.0, &plots_caerleon)], Operator::E, context.caerleon_plots);
//     model.reg_constr(vec![Summand(1.0, &plots_fort_sterling)], Operator::E, context.fort_sterling_plots);
//     model.reg_constr(vec![Summand(1.0, &plots_lymhurst)], Operator::E, context.lymhurst_plots);
//     model.reg_constr(vec![Summand(1.0, &plots_martlock)], Operator::E, context.martlock_plots);
//     model.reg_constr(vec![Summand(1.0, &plots_thetford)], Operator::E, context.thetford_plots);

//     //// plot count constraints
//     model.reg_constr(vec![Summand(1.0, &herb_gardens_brecilien), Summand(1.0, &farms_brecilien), Summand(1.0, &pastures_brecilien), Summand(-1.0, &plots_brecilien)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &herb_gardens_bridgewatch), Summand(1.0, &farms_bridgewatch), Summand(1.0, &pastures_bridgewatch), Summand(-1.0, &plots_bridgewatch)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &herb_gardens_caerleon), Summand(1.0, &farms_caerleon), Summand(1.0, &pastures_caerleon), Summand(-1.0, &plots_caerleon)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &herb_gardens_fort_sterling), Summand(1.0, &farms_fort_sterling), Summand(1.0, &pastures_fort_sterling), Summand(-1.0, &plots_fort_sterling)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &herb_gardens_lymhurst), Summand(1.0, &farms_lymhurst), Summand(1.0, &pastures_lymhurst), Summand(-1.0, &plots_lymhurst)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &herb_gardens_martlock), Summand(1.0, &farms_martlock), Summand(1.0, &pastures_martlock), Summand(-1.0, &plots_martlock)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &herb_gardens_thetford), Summand(1.0, &farms_thetford), Summand(1.0, &pastures_thetford), Summand(-1.0, &plots_thetford)], Operator::E, 0.0);

//     //// plots to tiles constraints
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &herb_gardens_brecilien), Summand(-1.0, &herb_garden_tiles_brecilien)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &farms_brecilien), Summand(-1.0, &farm_tiles_brecilien)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &pastures_brecilien), Summand(-1.0, &pasture_tiles_brecilien)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &herb_gardens_bridgewatch), Summand(-1.0, &herb_garden_tiles_bridgewatch)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &farms_bridgewatch), Summand(-1.0, &farm_tiles_bridgewatch)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &pastures_bridgewatch), Summand(-1.0, &pasture_tiles_bridgewatch)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &herb_gardens_caerleon), Summand(-1.0, &herb_garden_tiles_caerleon)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &farms_caerleon), Summand(-1.0, &farm_tiles_caerleon)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &pastures_caerleon), Summand(-1.0, &pasture_tiles_caerleon)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &herb_gardens_fort_sterling), Summand(-1.0, &herb_garden_tiles_fort_sterling)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &farms_fort_sterling), Summand(-1.0, &farm_tiles_fort_sterling)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &pastures_fort_sterling), Summand(-1.0, &pasture_tiles_fort_sterling)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &herb_gardens_lymhurst), Summand(-1.0, &herb_garden_tiles_lymhurst)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &farms_lymhurst), Summand(-1.0, &farm_tiles_lymhurst)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &pastures_lymhurst), Summand(-1.0, &pasture_tiles_lymhurst)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &herb_gardens_martlock), Summand(-1.0, &herb_garden_tiles_martlock)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &farms_martlock), Summand(-1.0, &farm_tiles_martlock)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &pastures_martlock), Summand(-1.0, &pasture_tiles_martlock)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &herb_gardens_thetford), Summand(-1.0, &herb_garden_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &farms_thetford), Summand(-1.0, &farm_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(TILES_PER_PLOT, &pastures_thetford), Summand(-1.0, &pasture_tiles_thetford)], Operator::E, 0.0);

//     //// tiles to filled tiles constraints
//     model.reg_constr(vec![Summand(1.0, &yarrow_tiles_brecilien), Summand(1.0, &muellin_tiles_brecilien), Summand(1.0, &foxglove_tiles_brecilien), Summand(1.0, &teasel_tiles_brecilien), Summand(1.0, &burdock_tiles_brecilien), Summand(1.0, &comfrey_tiles_brecilien), Summand(1.0, &agaric_tiles_brecilien), Summand(-1.0, &herb_garden_tiles_brecilien)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &cow_tiles_brecilien), Summand(1.0, &pig_tiles_brecilien), Summand(1.0, &sheep_tiles_brecilien), Summand(1.0, &goose_tiles_brecilien), Summand(1.0, &goat_tiles_brecilien), Summand(1.0, &chicken_tiles_brecilien), Summand(1.0, &calf_tiles_brecilien), Summand(1.0, &piglet_tiles_brecilien), Summand(1.0, &lamb_tiles_brecilien), Summand(1.0, &gosling_tiles_brecilien), Summand(1.0, &kid_tiles_brecilien), Summand(1.0, &baby_chicken_tiles_brecilien), Summand(-1.0, &pasture_tiles_brecilien)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &pumpkin_tiles_brecilien), Summand(1.0, &corn_tiles_brecilien), Summand(1.0, &potato_tiles_brecilien), Summand(1.0, &cabbage_tiles_brecilien), Summand(1.0, &turnip_tiles_brecilien), Summand(1.0, &wheat_tiles_brecilien), Summand(1.0, &bean_tiles_brecilien), Summand(1.0, &carrot_tiles_brecilien), Summand(-1.0, &farm_tiles_brecilien)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &yarrow_tiles_bridgewatch), Summand(1.0, &muellin_tiles_bridgewatch), Summand(1.0, &foxglove_tiles_bridgewatch), Summand(1.0, &teasel_tiles_bridgewatch), Summand(1.0, &burdock_tiles_bridgewatch), Summand(1.0, &comfrey_tiles_bridgewatch), Summand(1.0, &agaric_tiles_bridgewatch), Summand(-1.0, &herb_garden_tiles_bridgewatch)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &cow_tiles_bridgewatch), Summand(1.0, &pig_tiles_bridgewatch), Summand(1.0, &sheep_tiles_bridgewatch), Summand(1.0, &goose_tiles_bridgewatch), Summand(1.0, &goat_tiles_bridgewatch), Summand(1.0, &chicken_tiles_bridgewatch), Summand(1.0, &calf_tiles_bridgewatch), Summand(1.0, &piglet_tiles_bridgewatch), Summand(1.0, &lamb_tiles_bridgewatch), Summand(1.0, &gosling_tiles_bridgewatch), Summand(1.0, &kid_tiles_bridgewatch), Summand(1.0, &baby_chicken_tiles_bridgewatch), Summand(-1.0, &pasture_tiles_bridgewatch)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &pumpkin_tiles_bridgewatch), Summand(1.0, &corn_tiles_bridgewatch), Summand(1.0, &potato_tiles_bridgewatch), Summand(1.0, &cabbage_tiles_bridgewatch), Summand(1.0, &turnip_tiles_bridgewatch), Summand(1.0, &wheat_tiles_bridgewatch), Summand(1.0, &bean_tiles_bridgewatch), Summand(1.0, &carrot_tiles_bridgewatch), Summand(-1.0, &farm_tiles_bridgewatch)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &yarrow_tiles_caerleon), Summand(1.0, &muellin_tiles_caerleon), Summand(1.0, &foxglove_tiles_caerleon), Summand(1.0, &teasel_tiles_caerleon), Summand(1.0, &burdock_tiles_caerleon), Summand(1.0, &comfrey_tiles_caerleon), Summand(1.0, &agaric_tiles_caerleon), Summand(-1.0, &herb_garden_tiles_caerleon)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &cow_tiles_caerleon), Summand(1.0, &pig_tiles_caerleon), Summand(1.0, &sheep_tiles_caerleon), Summand(1.0, &goose_tiles_caerleon), Summand(1.0, &goat_tiles_caerleon), Summand(1.0, &chicken_tiles_caerleon), Summand(1.0, &calf_tiles_caerleon), Summand(1.0, &piglet_tiles_caerleon), Summand(1.0, &lamb_tiles_caerleon), Summand(1.0, &gosling_tiles_caerleon), Summand(1.0, &kid_tiles_caerleon), Summand(1.0, &baby_chicken_tiles_caerleon), Summand(-1.0, &pasture_tiles_caerleon)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &pumpkin_tiles_caerleon), Summand(1.0, &corn_tiles_caerleon), Summand(1.0, &potato_tiles_caerleon), Summand(1.0, &cabbage_tiles_caerleon), Summand(1.0, &turnip_tiles_caerleon), Summand(1.0, &wheat_tiles_caerleon), Summand(1.0, &bean_tiles_caerleon), Summand(1.0, &carrot_tiles_caerleon), Summand(-1.0, &farm_tiles_caerleon)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &yarrow_tiles_fort_sterling), Summand(1.0, &muellin_tiles_fort_sterling), Summand(1.0, &foxglove_tiles_fort_sterling), Summand(1.0, &teasel_tiles_fort_sterling), Summand(1.0, &burdock_tiles_fort_sterling), Summand(1.0, &comfrey_tiles_fort_sterling), Summand(1.0, &agaric_tiles_fort_sterling), Summand(-1.0, &herb_garden_tiles_fort_sterling)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &cow_tiles_fort_sterling), Summand(1.0, &pig_tiles_fort_sterling), Summand(1.0, &sheep_tiles_fort_sterling), Summand(1.0, &goose_tiles_fort_sterling), Summand(1.0, &goat_tiles_fort_sterling), Summand(1.0, &chicken_tiles_fort_sterling), Summand(1.0, &calf_tiles_fort_sterling), Summand(1.0, &piglet_tiles_fort_sterling), Summand(1.0, &lamb_tiles_fort_sterling), Summand(1.0, &gosling_tiles_fort_sterling), Summand(1.0, &kid_tiles_fort_sterling), Summand(1.0, &baby_chicken_tiles_fort_sterling), Summand(-1.0, &pasture_tiles_fort_sterling)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &pumpkin_tiles_fort_sterling), Summand(1.0, &corn_tiles_fort_sterling), Summand(1.0, &potato_tiles_fort_sterling), Summand(1.0, &cabbage_tiles_fort_sterling), Summand(1.0, &turnip_tiles_fort_sterling), Summand(1.0, &wheat_tiles_fort_sterling), Summand(1.0, &bean_tiles_fort_sterling), Summand(1.0, &carrot_tiles_fort_sterling), Summand(-1.0, &farm_tiles_fort_sterling)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &yarrow_tiles_lymhurst), Summand(1.0, &muellin_tiles_lymhurst), Summand(1.0, &foxglove_tiles_lymhurst), Summand(1.0, &teasel_tiles_lymhurst), Summand(1.0, &burdock_tiles_lymhurst), Summand(1.0, &comfrey_tiles_lymhurst), Summand(1.0, &agaric_tiles_lymhurst), Summand(-1.0, &herb_garden_tiles_lymhurst)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &cow_tiles_lymhurst), Summand(1.0, &pig_tiles_lymhurst), Summand(1.0, &sheep_tiles_lymhurst), Summand(1.0, &goose_tiles_lymhurst), Summand(1.0, &goat_tiles_lymhurst), Summand(1.0, &chicken_tiles_lymhurst), Summand(1.0, &calf_tiles_lymhurst), Summand(1.0, &piglet_tiles_lymhurst), Summand(1.0, &lamb_tiles_lymhurst), Summand(1.0, &gosling_tiles_lymhurst), Summand(1.0, &kid_tiles_lymhurst), Summand(1.0, &baby_chicken_tiles_lymhurst), Summand(-1.0, &pasture_tiles_lymhurst)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &pumpkin_tiles_lymhurst), Summand(1.0, &corn_tiles_lymhurst), Summand(1.0, &potato_tiles_lymhurst), Summand(1.0, &cabbage_tiles_lymhurst), Summand(1.0, &turnip_tiles_lymhurst), Summand(1.0, &wheat_tiles_lymhurst), Summand(1.0, &bean_tiles_lymhurst), Summand(1.0, &carrot_tiles_lymhurst), Summand(-1.0, &farm_tiles_lymhurst)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &yarrow_tiles_martlock), Summand(1.0, &muellin_tiles_martlock), Summand(1.0, &foxglove_tiles_martlock), Summand(1.0, &teasel_tiles_martlock), Summand(1.0, &burdock_tiles_martlock), Summand(1.0, &comfrey_tiles_martlock), Summand(1.0, &agaric_tiles_martlock), Summand(-1.0, &herb_garden_tiles_martlock)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &cow_tiles_martlock), Summand(1.0, &pig_tiles_martlock), Summand(1.0, &sheep_tiles_martlock), Summand(1.0, &goose_tiles_martlock), Summand(1.0, &goat_tiles_martlock), Summand(1.0, &chicken_tiles_martlock), Summand(1.0, &calf_tiles_martlock), Summand(1.0, &piglet_tiles_martlock), Summand(1.0, &lamb_tiles_martlock), Summand(1.0, &gosling_tiles_martlock), Summand(1.0, &kid_tiles_martlock), Summand(1.0, &baby_chicken_tiles_martlock), Summand(-1.0, &pasture_tiles_martlock)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &pumpkin_tiles_martlock), Summand(1.0, &corn_tiles_martlock), Summand(1.0, &potato_tiles_martlock), Summand(1.0, &cabbage_tiles_martlock), Summand(1.0, &turnip_tiles_martlock), Summand(1.0, &wheat_tiles_martlock), Summand(1.0, &bean_tiles_martlock), Summand(1.0, &carrot_tiles_martlock), Summand(-1.0, &farm_tiles_martlock)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &yarrow_tiles_thetford), Summand(1.0, &muellin_tiles_thetford), Summand(1.0, &foxglove_tiles_thetford), Summand(1.0, &teasel_tiles_thetford), Summand(1.0, &burdock_tiles_thetford), Summand(1.0, &comfrey_tiles_thetford), Summand(1.0, &agaric_tiles_thetford), Summand(-1.0, &herb_garden_tiles_thetford)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &cow_tiles_thetford), Summand(1.0, &pig_tiles_thetford), Summand(1.0, &sheep_tiles_thetford), Summand(1.0, &goose_tiles_thetford), Summand(1.0, &goat_tiles_thetford), Summand(1.0, &chicken_tiles_thetford), Summand(1.0, &calf_tiles_thetford), Summand(1.0, &piglet_tiles_thetford), Summand(1.0, &lamb_tiles_thetford), Summand(1.0, &gosling_tiles_thetford), Summand(1.0, &kid_tiles_thetford), Summand(1.0, &baby_chicken_tiles_thetford), Summand(-1.0, &pasture_tiles_thetford)], Operator::Le, 0.0);
//     model.reg_constr(vec![Summand(1.0, &pumpkin_tiles_thetford), Summand(1.0, &corn_tiles_thetford), Summand(1.0, &potato_tiles_thetford), Summand(1.0, &cabbage_tiles_thetford), Summand(1.0, &turnip_tiles_thetford), Summand(1.0, &wheat_tiles_thetford), Summand(1.0, &bean_tiles_thetford), Summand(1.0, &carrot_tiles_thetford), Summand(-1.0, &farm_tiles_thetford)], Operator::Le, 0.0);

//     //// filled tiles to products constraints
//     ////// herb constraints
//     model.reg_constr(vec![Summand(1.0, &agaric_herbs), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &agaric_tiles_brecilien), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &agaric_tiles_bridgewatch), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &agaric_tiles_caerleon), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &agaric_tiles_fort_sterling), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &agaric_tiles_lymhurst), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &agaric_tiles_martlock), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &agaric_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &comfrey_herbs), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &comfrey_tiles_brecilien), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &comfrey_tiles_bridgewatch), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &comfrey_tiles_caerleon), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &comfrey_tiles_fort_sterling), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &comfrey_tiles_lymhurst), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &comfrey_tiles_martlock), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &comfrey_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &burdock_herbs), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &burdock_tiles_brecilien), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &burdock_tiles_bridgewatch), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &burdock_tiles_caerleon), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &burdock_tiles_fort_sterling), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &burdock_tiles_lymhurst), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &burdock_tiles_martlock), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &burdock_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &teasel_herbs), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &teasel_tiles_brecilien), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &teasel_tiles_bridgewatch), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &teasel_tiles_caerleon), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &teasel_tiles_fort_sterling), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &teasel_tiles_lymhurst), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &teasel_tiles_martlock), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &teasel_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &foxglove_herbs), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &foxglove_tiles_brecilien), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &foxglove_tiles_bridgewatch), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &foxglove_tiles_caerleon), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &foxglove_tiles_fort_sterling), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &foxglove_tiles_lymhurst), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &foxglove_tiles_martlock), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &foxglove_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &muellin_herbs), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &muellin_tiles_brecilien), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &muellin_tiles_bridgewatch), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &muellin_tiles_caerleon), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &muellin_tiles_fort_sterling), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &muellin_tiles_lymhurst), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &muellin_tiles_martlock), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &muellin_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &yarrow_herbs), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &yarrow_tiles_brecilien), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &yarrow_tiles_bridgewatch), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &yarrow_tiles_caerleon), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &yarrow_tiles_fort_sterling), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &yarrow_tiles_lymhurst), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &yarrow_tiles_martlock), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &yarrow_tiles_thetford)], Operator::E, 0.0);

//     ////// crop constraints
//     model.reg_constr(vec![Summand(1.0, &carrot_crops), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &carrot_tiles_brecilien), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &carrot_tiles_bridgewatch), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &carrot_tiles_caerleon), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &carrot_tiles_fort_sterling), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &carrot_tiles_lymhurst), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &carrot_tiles_martlock), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &carrot_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &bean_crops), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &bean_tiles_brecilien), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &bean_tiles_bridgewatch), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &bean_tiles_caerleon), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &bean_tiles_fort_sterling), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &bean_tiles_lymhurst), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &bean_tiles_martlock), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &bean_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &wheat_crops), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &wheat_tiles_brecilien), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &baby_chicken_tiles_brecilien), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &chicken_tiles_brecilien), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &wheat_tiles_bridgewatch), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &baby_chicken_tiles_bridgewatch), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &chicken_tiles_bridgewatch), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &wheat_tiles_caerleon), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &baby_chicken_tiles_caerleon), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &chicken_tiles_caerleon), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &wheat_tiles_fort_sterling), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &baby_chicken_tiles_fort_sterling), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &chicken_tiles_fort_sterling), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &wheat_tiles_lymhurst), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &baby_chicken_tiles_lymhurst), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &chicken_tiles_lymhurst), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &wheat_tiles_martlock), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &baby_chicken_tiles_martlock), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &chicken_tiles_martlock), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &wheat_tiles_thetford), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &baby_chicken_tiles_thetford), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &chicken_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &turnip_crops), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &turnip_tiles_brecilien), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &kid_tiles_brecilien), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &goat_tiles_brecilien), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &turnip_tiles_bridgewatch), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &kid_tiles_bridgewatch), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &goat_tiles_bridgewatch), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &turnip_tiles_caerleon), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &kid_tiles_caerleon), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &goat_tiles_caerleon), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &turnip_tiles_fort_sterling), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &kid_tiles_fort_sterling), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &goat_tiles_fort_sterling), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &turnip_tiles_lymhurst), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &kid_tiles_lymhurst), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &goat_tiles_lymhurst), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &turnip_tiles_martlock), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &kid_tiles_martlock), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &goat_tiles_martlock), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &turnip_tiles_thetford), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &kid_tiles_thetford), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &goat_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &cabbage_crops), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &cabbage_tiles_brecilien), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &gosling_tiles_brecilien), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &goose_tiles_brecilien), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &cabbage_tiles_bridgewatch), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &gosling_tiles_bridgewatch), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &goose_tiles_bridgewatch), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &cabbage_tiles_caerleon), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &gosling_tiles_caerleon), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &goose_tiles_caerleon), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &cabbage_tiles_fort_sterling), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &gosling_tiles_fort_sterling), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &goose_tiles_fort_sterling), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &cabbage_tiles_lymhurst), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &gosling_tiles_lymhurst), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &goose_tiles_lymhurst), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &cabbage_tiles_martlock), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &gosling_tiles_martlock), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &goose_tiles_martlock), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &cabbage_tiles_thetford), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &gosling_tiles_thetford), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &goose_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &potato_crops), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &potato_tiles_brecilien), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &lamb_tiles_brecilien), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &sheep_tiles_brecilien), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &potato_tiles_bridgewatch), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &lamb_tiles_bridgewatch), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &sheep_tiles_bridgewatch), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &potato_tiles_caerleon), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &lamb_tiles_caerleon), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &sheep_tiles_caerleon), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &potato_tiles_fort_sterling), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &lamb_tiles_fort_sterling), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &sheep_tiles_fort_sterling), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &potato_tiles_lymhurst), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &lamb_tiles_lymhurst), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &sheep_tiles_lymhurst), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &potato_tiles_martlock), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &lamb_tiles_martlock), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &sheep_tiles_martlock), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &potato_tiles_thetford), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &lamb_tiles_thetford), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &sheep_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &corn_crops), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &corn_tiles_brecilien), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &piglet_tiles_brecilien), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &pig_tiles_brecilien), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &corn_tiles_bridgewatch), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &piglet_tiles_bridgewatch), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &pig_tiles_bridgewatch), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &corn_tiles_caerleon), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &piglet_tiles_caerleon), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &pig_tiles_caerleon), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &corn_tiles_fort_sterling), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &piglet_tiles_fort_sterling), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &pig_tiles_fort_sterling), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &corn_tiles_lymhurst), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &piglet_tiles_lymhurst), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &pig_tiles_lymhurst), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &corn_tiles_martlock), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &piglet_tiles_martlock), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &pig_tiles_martlock), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &corn_tiles_thetford), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &piglet_tiles_thetford), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &pig_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &pumpkin_crops), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &pumpkin_tiles_brecilien), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &calf_tiles_brecilien), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &cow_tiles_brecilien), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &pumpkin_tiles_bridgewatch), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &calf_tiles_bridgewatch), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &cow_tiles_bridgewatch), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &pumpkin_tiles_caerleon), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &calf_tiles_caerleon), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &cow_tiles_caerleon), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &pumpkin_tiles_fort_sterling), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &calf_tiles_fort_sterling), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &cow_tiles_fort_sterling), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR, &pumpkin_tiles_lymhurst), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &calf_tiles_lymhurst), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &cow_tiles_lymhurst), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &pumpkin_tiles_martlock), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &calf_tiles_martlock), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &cow_tiles_martlock), Summand(-1.0 * CROP_AND_HERB_FACTOR * context.premium_factor, &pumpkin_tiles_thetford), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &calf_tiles_thetford), Summand(ANIMAL_FAVORITE_FOOD_CONSUMPTION, &cow_tiles_thetford)], Operator::E, 0.0);

//     ////// animal product constraints
//     model.reg_constr(vec![Summand(1.0, &hen_eggs), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &chicken_tiles_brecilien), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &chicken_tiles_bridgewatch), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &chicken_tiles_caerleon), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor * BONUS_FACTOR, &chicken_tiles_fort_sterling), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &chicken_tiles_lymhurst), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &chicken_tiles_martlock), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &chicken_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &goats_milk), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &goat_tiles_brecilien), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor * BONUS_FACTOR, &goat_tiles_bridgewatch), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &goat_tiles_caerleon), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &goat_tiles_fort_sterling), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &goat_tiles_lymhurst), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &goat_tiles_martlock), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &goat_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &goose_eggs), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &goose_tiles_brecilien), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &goose_tiles_bridgewatch), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &goose_tiles_caerleon), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &goose_tiles_fort_sterling), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor * BONUS_FACTOR, &goose_tiles_lymhurst), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &goose_tiles_martlock), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &goose_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &sheeps_milk), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &sheep_tiles_brecilien), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &sheep_tiles_bridgewatch), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &sheep_tiles_caerleon), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor * BONUS_FACTOR, &sheep_tiles_fort_sterling), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &sheep_tiles_lymhurst), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &sheep_tiles_martlock), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &sheep_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &cows_milk), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &cow_tiles_brecilien), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &cow_tiles_bridgewatch), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &cow_tiles_caerleon), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &cow_tiles_fort_sterling), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &cow_tiles_lymhurst), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor * BONUS_FACTOR, &cow_tiles_martlock), Summand(-1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor, &cow_tiles_thetford)], Operator::E, 0.0);

//     ////// meat constraints
//     model.reg_constr(vec![Summand(1.0, &raw_chicken), Summand(-1.0 * MEAT_FACTOR, &baby_chicken_tiles_brecilien), Summand(-1.0 * MEAT_FACTOR, &baby_chicken_tiles_bridgewatch), Summand(-1.0 * MEAT_FACTOR, &baby_chicken_tiles_caerleon), Summand(-1.0 * MEAT_FACTOR * BONUS_FACTOR, &baby_chicken_tiles_fort_sterling), Summand(-1.0 * MEAT_FACTOR, &baby_chicken_tiles_lymhurst), Summand(-1.0 * MEAT_FACTOR, &baby_chicken_tiles_martlock), Summand(-1.0 * MEAT_FACTOR, &baby_chicken_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &raw_goat), Summand(-1.0 * MEAT_FACTOR, &kid_tiles_brecilien), Summand(-1.0 * MEAT_FACTOR * BONUS_FACTOR, &kid_tiles_bridgewatch), Summand(-1.0 * MEAT_FACTOR, &kid_tiles_caerleon), Summand(-1.0 * MEAT_FACTOR, &kid_tiles_fort_sterling), Summand(-1.0 * MEAT_FACTOR, &kid_tiles_lymhurst), Summand(-1.0 * MEAT_FACTOR, &kid_tiles_martlock), Summand(-1.0 * MEAT_FACTOR, &kid_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &raw_goose), Summand(-1.0 * MEAT_FACTOR, &gosling_tiles_brecilien), Summand(-1.0 * MEAT_FACTOR, &gosling_tiles_bridgewatch), Summand(-1.0 * MEAT_FACTOR, &gosling_tiles_caerleon), Summand(-1.0 * MEAT_FACTOR, &gosling_tiles_fort_sterling), Summand(-1.0 * MEAT_FACTOR * BONUS_FACTOR, &gosling_tiles_lymhurst), Summand(-1.0 * MEAT_FACTOR, &gosling_tiles_martlock), Summand(-1.0 * MEAT_FACTOR, &gosling_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &raw_mutton), Summand(-1.0 * MEAT_FACTOR, &lamb_tiles_brecilien), Summand(-1.0 * MEAT_FACTOR, &lamb_tiles_bridgewatch), Summand(-1.0 * MEAT_FACTOR, &lamb_tiles_caerleon), Summand(-1.0 * MEAT_FACTOR * BONUS_FACTOR, &lamb_tiles_fort_sterling), Summand(-1.0 * MEAT_FACTOR, &lamb_tiles_lymhurst), Summand(-1.0 * MEAT_FACTOR, &lamb_tiles_martlock), Summand(-1.0 * MEAT_FACTOR, &lamb_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &raw_pork), Summand(-1.0 * MEAT_FACTOR, &piglet_tiles_brecilien), Summand(-1.0 * MEAT_FACTOR, &piglet_tiles_bridgewatch), Summand(-1.0 * MEAT_FACTOR, &piglet_tiles_caerleon), Summand(-1.0 * MEAT_FACTOR, &piglet_tiles_fort_sterling), Summand(-1.0 * MEAT_FACTOR, &piglet_tiles_lymhurst), Summand(-1.0 * MEAT_FACTOR, &piglet_tiles_martlock), Summand(-1.0 * MEAT_FACTOR * BONUS_FACTOR, &piglet_tiles_thetford)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(1.0, &raw_beef), Summand(-1.0 * MEAT_FACTOR, &calf_tiles_brecilien), Summand(-1.0 * MEAT_FACTOR, &calf_tiles_bridgewatch), Summand(-1.0 * MEAT_FACTOR, &calf_tiles_caerleon), Summand(-1.0 * MEAT_FACTOR, &calf_tiles_fort_sterling), Summand(-1.0 * MEAT_FACTOR, &calf_tiles_lymhurst), Summand(-1.0 * MEAT_FACTOR * BONUS_FACTOR, &calf_tiles_martlock), Summand(-1.0 * MEAT_FACTOR, &calf_tiles_thetford)], Operator::E, 0.0);

//     ////// recipe constraints
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &minor_energy_potion), Summand(-1.0 * L04_INGREDIENT, &agaric_herbs)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &minor_healing_potion), Summand(-1.0 * L04_INGREDIENT, &agaric_herbs)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &minor_gigantify_potion), Summand(-1.0 * L04_INGREDIENT, &comfrey_herbs)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &minor_resistance_potion), Summand(-1.0 * L04_INGREDIENT, &comfrey_herbs)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &minor_sticky_potion), Summand(-1.0 * L04_INGREDIENT, &comfrey_herbs)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &minor_poison_potion), Summand(-1.0 * L04_INGREDIENT, &burdock_herbs), Summand(-1.0 * L02_INGREDIENT, &comfrey_herbs)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &energy_potion), Summand(-1.0 * L08_INGREDIENT, &burdock_herbs ), Summand(-1.0 * L03_INGREDIENT, &goats_milk)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &healing_potion), Summand(-1.0 * L08_INGREDIENT, &burdock_herbs), Summand(-1.0 * L03_INGREDIENT, &hen_eggs)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &gigantify_potion), Summand(-1.0 * L08_INGREDIENT, &teasel_herbs), Summand(-1.0 * L05_INGREDIENT, &burdock_herbs), Summand(-1.0 * L03_INGREDIENT, &goose_eggs)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &resistance_potion), Summand(-1.0 * L08_INGREDIENT, &teasel_herbs), Summand(-1.0 * L05_INGREDIENT, &burdock_herbs), Summand(-1.0 * L03_INGREDIENT, &goats_milk)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &sticky_potion), Summand(-1.0 * L08_INGREDIENT, &teasel_herbs), Summand(-1.0 * L05_INGREDIENT, &burdock_herbs), Summand(-1.0 * L03_INGREDIENT, &goose_eggs)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &poison_potion), Summand(-1.0 * L08_INGREDIENT, &foxglove_herbs), Summand(-1.0 * L05_INGREDIENT, &teasel_herbs), Summand(-1.0 * L05_INGREDIENT, &comfrey_herbs), Summand(-1.0 * L03_INGREDIENT, &sheeps_milk)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &major_energy_potion), Summand(-1.0 * L11_INGREDIENT, &foxglove_herbs), Summand(-1.0 * L07_INGREDIENT, &sheeps_milk), Summand(-1.0 * L07_INGREDIENT, &potato_crops)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &major_healing_potion), Summand(-1.0 * L11_INGREDIENT, &foxglove_herbs), Summand(-1.0 * L07_INGREDIENT, &goose_eggs), Summand(-1.0 * L07_INGREDIENT, &potato_crops)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &major_gigantify_potion), Summand(-1.0 * L11_INGREDIENT, &muellin_herbs), Summand(-1.0 * L09_INGREDIENT, &foxglove_herbs), Summand(-1.0 * L07_INGREDIENT, &goose_eggs), Summand(-1.0 * L07_INGREDIENT, &corn_crops)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &major_resistance_potion), Summand(-1.0 * L11_INGREDIENT, &muellin_herbs), Summand(-1.0 * L09_INGREDIENT, &foxglove_herbs), Summand(-1.0 * L09_INGREDIENT, &burdock_herbs), Summand(-1.0 * L07_INGREDIENT, &sheeps_milk), Summand(-1.0 * L07_INGREDIENT, &corn_crops)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &major_sticky_potion), Summand(-1.0 * L11_INGREDIENT, &muellin_herbs), Summand(-1.0 * L09_INGREDIENT, &foxglove_herbs), Summand(-1.0 * L09_INGREDIENT, &burdock_herbs), Summand(-1.0 * L07_INGREDIENT, &goose_eggs), Summand(-1.0 * L07_INGREDIENT, &corn_crops)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &major_poison_potion), Summand(-1.0 * L11_INGREDIENT, &yarrow_herbs), Summand(-1.0 * L09_INGREDIENT, &muellin_herbs), Summand(-1.0 * L09_INGREDIENT, &teasel_herbs), Summand(-1.0 * L07_INGREDIENT, &cows_milk), Summand(-1.0 * L07_INGREDIENT, &pumpkin_crops)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(POTIONS_PER_CRAFT, &invisibility_potion), Summand(-1.0 * L11_INGREDIENT, &yarrow_herbs), Summand(-1.0 * L09_INGREDIENT, &muellin_herbs), Summand(-1.0 * L09_INGREDIENT, &teasel_herbs), Summand(-1.0 * L07_INGREDIENT, &cows_milk), Summand(-1.0 * L07_INGREDIENT, &pumpkin_crops)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &chicken_omelette), Summand(-1.0 * L02_INGREDIENT, &wheat_crops), Summand(-1.0 * L04_INGREDIENT, &raw_chicken), Summand(-1.0 * L01_INGREDIENT, &hen_eggs)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &goose_omelette), Summand(-1.0 * L05_INGREDIENT, &cabbage_crops), Summand(-1.0 * L08_INGREDIENT, &raw_goose), Summand(-1.0 * L03_INGREDIENT, &goose_eggs)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &pork_omelette), Summand(-1.0 * L09_INGREDIENT, &corn_crops), Summand(-1.0 * L11_INGREDIENT, &raw_pork), Summand(-1.0 * L07_INGREDIENT, &goose_eggs)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &bean_salad), Summand(-1.0 * L04_INGREDIENT, &bean_crops), Summand(-1.0 * L04_INGREDIENT, &carrot_crops)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &turnip_salad), Summand(-1.0 * L08_INGREDIENT, &turnip_crops), Summand(-1.0 * L08_INGREDIENT, &wheat_crops)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &potato_salad), Summand(-1.0 * L11_INGREDIENT, &potato_crops), Summand(-1.0 * L11_INGREDIENT, &cabbage_crops)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &goat_sandwich), Summand(-1.0 * L02_INGREDIENT, &wheat_crops), Summand(-1.0 * L04_INGREDIENT, &raw_goat), Summand(-1.0 * L01_INGREDIENT, &goats_milk)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &mutton_sandwich), Summand(-1.0 * L05_INGREDIENT, &wheat_crops), Summand(-1.0 * L08_INGREDIENT, &raw_mutton), Summand(-1.0 * L03_INGREDIENT, &sheeps_milk)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &beef_sandwich), Summand(-1.0 * L09_INGREDIENT, &wheat_crops), Summand(-1.0 * L11_INGREDIENT, &raw_beef), Summand(-1.0 * L07_INGREDIENT, &cows_milk)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &carrot_soup), Summand(-1.0 * L06_INGREDIENT, &carrot_crops)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &wheat_soup), Summand(-1.0 * L10_INGREDIENT, &wheat_crops)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &cabbage_soup), Summand(-1.0 * L12_INGREDIENT, &cabbage_crops)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &goat_stew), Summand(-1.0 * L02_INGREDIENT, &wheat_crops), Summand(-1.0 * L04_INGREDIENT, &raw_goat), Summand(-1.0 * L02_INGREDIENT, &turnip_crops)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &mutton_stew), Summand(-1.0 * L05_INGREDIENT, &wheat_crops), Summand(-1.0 * L08_INGREDIENT, &raw_mutton), Summand(-1.0 * L05_INGREDIENT, &potato_crops)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &beef_stew), Summand(-1.0 * L09_INGREDIENT, &pumpkin_crops), Summand(-1.0 * L11_INGREDIENT, &raw_beef), Summand(-1.0 * L09_INGREDIENT, &wheat_crops)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &roast_chicken), Summand(-1.0 * L04_INGREDIENT, &raw_chicken), Summand(-1.0 * L02_INGREDIENT, &bean_crops), Summand(-1.0 * L02_INGREDIENT, &goats_milk)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &roast_goose), Summand(-1.0 * L08_INGREDIENT, &raw_goose), Summand(-1.0 * L05_INGREDIENT, &cabbage_crops), Summand(-1.0 * L05_INGREDIENT, &sheeps_milk)], Operator::E, 0.0);
//     model.reg_constr(vec![Summand(FOOD_PER_CRAFT, &roast_pork), Summand(-1.0 * L11_INGREDIENT, &raw_pork), Summand(-1.0 * L09_INGREDIENT, &corn_crops), Summand(-1.0 * L09_INGREDIENT, &cows_milk)], Operator::E, 0.0);

//     model.optimize();
//     model
// }
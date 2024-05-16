use minilp::{Problem, OptimizationDirection, ComparisonOp, Solution};


use crate::{constants::*, types::Product, ModelContext};

 
// pub fn optimize_plots(context: ModelContext) -> Solution {
//     let mut problem = Problem::new(OptimizationDirection::Maximize);

//     let plots_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let herb_gardens_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let farms_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let pastures_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let herb_garden_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let farm_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let pasture_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let teasel_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let muellin_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let yarrow_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let pumpkin_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let cow_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let teasel_herbs = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let muellin_herbs = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let yarrow_herbs = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let pumpkin_crops = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let cows_milk = problem.add_var(0.0, (0.0, f64::INFINITY));
//     let major_poison_potion = problem.add_var(if context.target == Product::MajorPoisonPotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));

//     problem.add_constraint(&[(plots_lymhurst, 1.0)], ComparisonOp::Eq, context.lymhurst_plots);
//     problem.add_constraint(&[(herb_gardens_lymhurst, 1.0), (farms_lymhurst, 1.0), (pastures_lymhurst, 1.0), (plots_lymhurst, -1.0)], ComparisonOp::Eq, 0.0);
//     problem.add_constraint(&[(herb_gardens_lymhurst, TILES_PER_PLOT), (herb_garden_tiles_lymhurst, -1.0)], ComparisonOp::Eq, 0.0);
//     problem.add_constraint(&[(farms_lymhurst, TILES_PER_PLOT), (farm_tiles_lymhurst, -1.0)], ComparisonOp::Eq, 0.0);
//     problem.add_constraint(&[(pastures_lymhurst, TILES_PER_PLOT), (pasture_tiles_lymhurst, -1.0)], ComparisonOp::Eq, 0.0);
//     problem.add_constraint(&[(yarrow_tiles_lymhurst, 1.0), (muellin_tiles_lymhurst, 1.0), (teasel_tiles_lymhurst, 1.0), (herb_garden_tiles_lymhurst, -1.0)], ComparisonOp::Le, 0.0);
//     problem.add_constraint(&[(cow_tiles_lymhurst, 1.0), (pasture_tiles_lymhurst, -1.0)], ComparisonOp::Le, 0.0);
//     problem.add_constraint(&[(pumpkin_tiles_lymhurst, 1.0), (farm_tiles_lymhurst, -1.0)], ComparisonOp::Le, 0.0);
//     problem.add_constraint(&[(teasel_herbs, 1.0), (teasel_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor)], ComparisonOp::Eq, 0.0);
//     problem.add_constraint(&[(muellin_herbs, 1.0), (muellin_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor)], ComparisonOp::Eq, 0.0);
//     problem.add_constraint(&[(yarrow_herbs, 1.0), (yarrow_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor)], ComparisonOp::Eq, 0.0);
//     problem.add_constraint(&[(pumpkin_crops, 1.0), (pumpkin_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (cow_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION)], ComparisonOp::Eq, 0.0);
//     problem.add_constraint(&[(cows_milk, 1.0), (cow_tiles_lymhurst, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor)], ComparisonOp::Eq, 0.0);
    
//     problem.add_constraint(&[(cows_milk, 1.0), (pumpkin_crops, -1.0)], ComparisonOp::Eq, 0.0);
//     problem.add_constraint(&[(teasel_herbs, 1.0), (cows_milk, -2.0)], ComparisonOp::Eq, 0.0);
//     problem.add_constraint(&[(muellin_herbs, 1.0), (teasel_herbs, -1.0)], ComparisonOp::Eq, 0.0);
//     problem.add_constraint(&[(yarrow_herbs, 1.0), (muellin_herbs, -2.0)], ComparisonOp::Eq, 0.0);
//     problem.add_constraint(&[(major_poison_potion, L11_INGREDIENT), (yarrow_herbs, -1.0 * POTIONS_PER_CRAFT)], ComparisonOp::Eq, 0.0);
        
//     let solution = problem.solve().unwrap();
    
//     solution
// }

pub fn optimize_plots(context: ModelContext) -> Solution {
    let mut problem = Problem::new(OptimizationDirection::Maximize);

    // variables
    //// plots
    let plots_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let plots_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let plots_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let plots_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let plots_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let plots_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let plots_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    
    //// used plots
    let herb_gardens_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let farms_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pastures_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let herb_gardens_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let farms_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pastures_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let herb_gardens_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let farms_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pastures_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let herb_gardens_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let farms_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pastures_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let herb_gardens_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let farms_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pastures_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let herb_gardens_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let farms_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pastures_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let herb_gardens_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let farms_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pastures_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));

    //// tiles
    let herb_garden_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let farm_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pasture_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let herb_garden_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let farm_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pasture_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let herb_garden_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let farm_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pasture_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let herb_garden_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let farm_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pasture_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let herb_garden_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let farm_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pasture_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let herb_garden_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let farm_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pasture_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let herb_garden_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let farm_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pasture_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let agaric_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let comfrey_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let burdock_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let teasel_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let foxglove_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let muellin_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let yarrow_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let carrot_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let bean_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let wheat_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let turnip_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let cabbage_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let potato_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let corn_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pumpkin_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let baby_chicken_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let kid_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let gosling_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let lamb_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let piglet_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let calf_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let chicken_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let goat_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let goose_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let sheep_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pig_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let cow_tiles_brecilien = problem.add_var(0.0, (0.0, f64::INFINITY));
    let agaric_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let comfrey_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let burdock_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let teasel_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let foxglove_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let muellin_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let yarrow_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let carrot_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let bean_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let wheat_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let turnip_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let cabbage_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let potato_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let corn_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pumpkin_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let baby_chicken_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let kid_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let gosling_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let lamb_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let piglet_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let calf_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let chicken_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let goat_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let goose_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let sheep_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pig_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let cow_tiles_bridgewatch = problem.add_var(0.0, (0.0, f64::INFINITY));
    let agaric_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let comfrey_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let burdock_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let teasel_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let foxglove_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let muellin_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let yarrow_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let carrot_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let bean_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let wheat_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let turnip_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let cabbage_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let potato_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let corn_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pumpkin_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let baby_chicken_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let kid_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let gosling_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let lamb_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let piglet_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let calf_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let chicken_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let goat_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let goose_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let sheep_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pig_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let cow_tiles_caerleon = problem.add_var(0.0, (0.0, f64::INFINITY));
    let agaric_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let comfrey_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let burdock_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let teasel_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let foxglove_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let muellin_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let yarrow_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let carrot_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let bean_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let wheat_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let turnip_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let cabbage_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let potato_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let corn_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pumpkin_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let baby_chicken_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let kid_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let gosling_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let lamb_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let piglet_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let calf_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let chicken_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let goat_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let goose_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let sheep_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pig_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let cow_tiles_fort_sterling = problem.add_var(0.0, (0.0, f64::INFINITY));
    let agaric_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let comfrey_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let burdock_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let teasel_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let foxglove_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let muellin_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let yarrow_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let carrot_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let bean_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let wheat_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let turnip_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let cabbage_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let potato_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let corn_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pumpkin_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let baby_chicken_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let kid_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let gosling_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let lamb_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let piglet_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let calf_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let chicken_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let goat_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let goose_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let sheep_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pig_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let cow_tiles_lymhurst = problem.add_var(0.0, (0.0, f64::INFINITY));
    let agaric_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let comfrey_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let burdock_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let teasel_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let foxglove_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let muellin_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let yarrow_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let carrot_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let bean_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let wheat_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let turnip_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let cabbage_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let potato_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let corn_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pumpkin_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let baby_chicken_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let kid_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let gosling_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let lamb_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let piglet_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let calf_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let chicken_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let goat_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let goose_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let sheep_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pig_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let cow_tiles_martlock = problem.add_var(0.0, (0.0, f64::INFINITY));
    let agaric_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let comfrey_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let burdock_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let teasel_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let foxglove_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let muellin_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let yarrow_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let carrot_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let bean_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let wheat_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let turnip_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let cabbage_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let potato_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let corn_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pumpkin_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let baby_chicken_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let kid_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let gosling_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let lamb_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let piglet_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let calf_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let chicken_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let goat_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let goose_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let sheep_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pig_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));
    let cow_tiles_thetford = problem.add_var(0.0, (0.0, f64::INFINITY));

    //// yields
    let agaric_herbs = problem.add_var(0.0, (0.0, f64::INFINITY));
    let comfrey_herbs = problem.add_var(0.0, (0.0, f64::INFINITY));
    let burdock_herbs = problem.add_var(0.0, (0.0, f64::INFINITY));
    let teasel_herbs = problem.add_var(0.0, (0.0, f64::INFINITY));
    let foxglove_herbs = problem.add_var(0.0, (0.0, f64::INFINITY));
    let muellin_herbs = problem.add_var(0.0, (0.0, f64::INFINITY));
    let yarrow_herbs = problem.add_var(0.0, (0.0, f64::INFINITY));
    let carrot_crops = problem.add_var(0.0, (0.0, f64::INFINITY));
    let bean_crops = problem.add_var(0.0, (0.0, f64::INFINITY));
    let wheat_crops = problem.add_var(0.0, (0.0, f64::INFINITY));
    let turnip_crops = problem.add_var(0.0, (0.0, f64::INFINITY));
    let cabbage_crops = problem.add_var(0.0, (0.0, f64::INFINITY));
    let potato_crops = problem.add_var(0.0, (0.0, f64::INFINITY));
    let corn_crops = problem.add_var(0.0, (0.0, f64::INFINITY));
    let pumpkin_crops = problem.add_var(0.0, (0.0, f64::INFINITY));
    let hen_eggs = problem.add_var(0.0, (0.0, f64::INFINITY));
    let goats_milk = problem.add_var(0.0, (0.0, f64::INFINITY));
    let goose_eggs = problem.add_var(0.0, (0.0, f64::INFINITY));
    let sheeps_milk = problem.add_var(0.0, (0.0, f64::INFINITY));
    let cows_milk = problem.add_var(0.0, (0.0, f64::INFINITY));
    let raw_chicken = problem.add_var(0.0, (0.0, f64::INFINITY));
    let raw_goat = problem.add_var(0.0, (0.0, f64::INFINITY));
    let raw_goose = problem.add_var(0.0, (0.0, f64::INFINITY));
    let raw_mutton = problem.add_var(0.0, (0.0, f64::INFINITY));
    let raw_pork = problem.add_var(0.0, (0.0, f64::INFINITY));
    let raw_beef = problem.add_var(0.0, (0.0, f64::INFINITY));

    //// products
    // TODO: do this better
    let minor_energy_potion = problem.add_var(if context.target == Product::MinorEnergyPotion { 1.0 } else { if context.target == Product::MinorHealingPotion { 1.0 } else { 0.0 } }, (0.0, f64::INFINITY));
    let minor_healing_potion = problem.add_var(if context.target == Product::MinorHealingPotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let minor_gigantify_potion = problem.add_var(if context.target == Product::MinorGigantifyPotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let minor_resistance_potion = problem.add_var(if context.target == Product::MinorResistancePotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let minor_sticky_potion = problem.add_var(if context.target == Product::MinorStickyPotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let minor_poison_potion = problem.add_var(if context.target == Product::MinorPoisonPotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let energy_potion = problem.add_var(if context.target == Product::EnergyPotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let healing_potion = problem.add_var(if context.target == Product::HealingPotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let gigantify_potion = problem.add_var(if context.target == Product::GigantifyPotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let resistance_potion = problem.add_var(if context.target == Product::ResistancePotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let sticky_potion = problem.add_var(if context.target == Product::StickyPotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let poison_potion = problem.add_var(if context.target == Product::PoisonPotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let major_energy_potion = problem.add_var(if context.target == Product::MajorEnergyPotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let major_healing_potion = problem.add_var(if context.target == Product::MajorHealingPotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let major_gigantify_potion = problem.add_var(if context.target == Product::MajorGigantifyPotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let major_resistance_potion = problem.add_var(if context.target == Product::MajorResistancePotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let major_sticky_potion = problem.add_var(if context.target == Product::MajorStickyPotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let major_poison_potion = problem.add_var(if context.target == Product::MajorPoisonPotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let invisibility_potion = problem.add_var(if context.target == Product::InvisibilityPotion { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let chicken_omelette = problem.add_var(if context.target == Product::ChickenOmelette { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let goose_omelette = problem.add_var(if context.target == Product::GooseOmelette { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let pork_omelette = problem.add_var(if context.target == Product::PorkOmelette { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let bean_salad = problem.add_var(if context.target == Product::BeanSalad { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let turnip_salad = problem.add_var(if context.target == Product::TurnipSalad { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let potato_salad = problem.add_var(if context.target == Product::PotatoSalad { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let goat_sandwich = problem.add_var(if context.target == Product::GoatSandwich { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let mutton_sandwich = problem.add_var(if context.target == Product::MuttonSandwich { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let beef_sandwich = problem.add_var(if context.target == Product::BeefSandwich { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let carrot_soup = problem.add_var(if context.target == Product::CarrotSoup { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let wheat_soup = problem.add_var(if context.target == Product::WheatSoup { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let cabbage_soup = problem.add_var(if context.target == Product::CabbageSoup { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let goat_stew = problem.add_var(if context.target == Product::GoatStew { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let mutton_stew = problem.add_var(if context.target == Product::MuttonStew { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let beef_stew = problem.add_var(if context.target == Product::BeefStew { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let roast_chicken = problem.add_var(if context.target == Product::RoastChicken { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let roast_goose = problem.add_var(if context.target == Product::RoastGoose { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));
    let roast_pork = problem.add_var(if context.target == Product::RoastPork { 1.0 } else { 0.0 }, (0.0, f64::INFINITY));

    
    // constraints
    //// argument constraints
    problem.add_constraint(&[(plots_brecilien, 1.0)], ComparisonOp::Eq, context.brecilien_plots);
    problem.add_constraint(&[(plots_bridgewatch, 1.0)], ComparisonOp::Eq, context.bridgewatch_plots);
    problem.add_constraint(&[(plots_caerleon, 1.0)], ComparisonOp::Eq, context.caerleon_plots);
    problem.add_constraint(&[(plots_fort_sterling, 1.0)], ComparisonOp::Eq, context.fort_sterling_plots);
    problem.add_constraint(&[(plots_lymhurst, 1.0)], ComparisonOp::Eq, context.lymhurst_plots);
    problem.add_constraint(&[(plots_martlock, 1.0)], ComparisonOp::Eq, context.martlock_plots);
    problem.add_constraint(&[(plots_thetford, 1.0)], ComparisonOp::Eq, context.thetford_plots);

    //// plot count constraints
    problem.add_constraint(&[(herb_gardens_brecilien, 1.0), (farms_brecilien, 1.0), (pastures_brecilien, 1.0), (plots_brecilien, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(herb_gardens_bridgewatch, 1.0), (farms_bridgewatch, 1.0), (pastures_bridgewatch, 1.0), (plots_bridgewatch, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(herb_gardens_caerleon, 1.0), (farms_caerleon, 1.0), (pastures_caerleon, 1.0), (plots_caerleon, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(herb_gardens_fort_sterling, 1.0), (farms_fort_sterling, 1.0), (pastures_fort_sterling, 1.0), (plots_fort_sterling, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(herb_gardens_lymhurst, 1.0), (farms_lymhurst, 1.0), (pastures_lymhurst, 1.0), (plots_lymhurst, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(herb_gardens_martlock, 1.0), (farms_martlock, 1.0), (pastures_martlock, 1.0), (plots_martlock, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(herb_gardens_thetford, 1.0), (farms_thetford, 1.0), (pastures_thetford, 1.0), (plots_thetford, -1.0)], ComparisonOp::Eq, 0.0);

    //// plots to tiles constraints
    problem.add_constraint(&[(herb_gardens_brecilien, TILES_PER_PLOT), (herb_garden_tiles_brecilien, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(farms_brecilien, TILES_PER_PLOT), (farm_tiles_brecilien, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(pastures_brecilien, TILES_PER_PLOT), (pasture_tiles_brecilien, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(herb_gardens_bridgewatch, TILES_PER_PLOT), (herb_garden_tiles_bridgewatch, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(farms_bridgewatch, TILES_PER_PLOT), (farm_tiles_bridgewatch, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(pastures_bridgewatch, TILES_PER_PLOT), (pasture_tiles_bridgewatch, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(herb_gardens_caerleon, TILES_PER_PLOT), (herb_garden_tiles_caerleon, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(farms_caerleon, TILES_PER_PLOT), (farm_tiles_caerleon, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(pastures_caerleon, TILES_PER_PLOT), (pasture_tiles_caerleon, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(herb_gardens_fort_sterling, TILES_PER_PLOT), (herb_garden_tiles_fort_sterling, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(farms_fort_sterling, TILES_PER_PLOT), (farm_tiles_fort_sterling, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(pastures_fort_sterling, TILES_PER_PLOT), (pasture_tiles_fort_sterling, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(herb_gardens_lymhurst, TILES_PER_PLOT), (herb_garden_tiles_lymhurst, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(farms_lymhurst, TILES_PER_PLOT), (farm_tiles_lymhurst, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(pastures_lymhurst, TILES_PER_PLOT), (pasture_tiles_lymhurst, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(herb_gardens_martlock, TILES_PER_PLOT), (herb_garden_tiles_martlock, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(farms_martlock, TILES_PER_PLOT), (farm_tiles_martlock, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(pastures_martlock, TILES_PER_PLOT), (pasture_tiles_martlock, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(herb_gardens_thetford, TILES_PER_PLOT), (herb_garden_tiles_thetford, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(farms_thetford, TILES_PER_PLOT), (farm_tiles_thetford, -1.0)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(pastures_thetford, TILES_PER_PLOT), (pasture_tiles_thetford, -1.0)], ComparisonOp::Eq, 0.0);

    //// tiles to filled tiles constraints
    problem.add_constraint(&[(yarrow_tiles_brecilien, 1.0), (muellin_tiles_brecilien, 1.0), (foxglove_tiles_brecilien, 1.0), (teasel_tiles_brecilien, 1.0), (burdock_tiles_brecilien, 1.0), (comfrey_tiles_brecilien, 1.0), (agaric_tiles_brecilien, 1.0), (herb_garden_tiles_brecilien, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(cow_tiles_brecilien, 1.0), (pig_tiles_brecilien, 1.0), (sheep_tiles_brecilien, 1.0), (goose_tiles_brecilien, 1.0), (goat_tiles_brecilien, 1.0), (chicken_tiles_brecilien, 1.0), (calf_tiles_brecilien, 1.0), (piglet_tiles_brecilien, 1.0), (lamb_tiles_brecilien, 1.0), (gosling_tiles_brecilien, 1.0), (kid_tiles_brecilien, 1.0), (baby_chicken_tiles_brecilien, 1.0), (pasture_tiles_brecilien, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(pumpkin_tiles_brecilien, 1.0), (corn_tiles_brecilien, 1.0), (potato_tiles_brecilien, 1.0), (cabbage_tiles_brecilien, 1.0), (turnip_tiles_brecilien, 1.0), (wheat_tiles_brecilien, 1.0), (bean_tiles_brecilien, 1.0), (carrot_tiles_brecilien, 1.0), (farm_tiles_brecilien, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(yarrow_tiles_bridgewatch, 1.0), (muellin_tiles_bridgewatch, 1.0), (foxglove_tiles_bridgewatch, 1.0), (teasel_tiles_bridgewatch, 1.0), (burdock_tiles_bridgewatch, 1.0), (comfrey_tiles_bridgewatch, 1.0), (agaric_tiles_bridgewatch, 1.0), (herb_garden_tiles_bridgewatch, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(cow_tiles_bridgewatch, 1.0), (pig_tiles_bridgewatch, 1.0), (sheep_tiles_bridgewatch, 1.0), (goose_tiles_bridgewatch, 1.0), (goat_tiles_bridgewatch, 1.0), (chicken_tiles_bridgewatch, 1.0), (calf_tiles_bridgewatch, 1.0), (piglet_tiles_bridgewatch, 1.0), (lamb_tiles_bridgewatch, 1.0), (gosling_tiles_bridgewatch, 1.0), (kid_tiles_bridgewatch, 1.0), (baby_chicken_tiles_bridgewatch, 1.0), (pasture_tiles_bridgewatch, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(pumpkin_tiles_bridgewatch, 1.0), (corn_tiles_bridgewatch, 1.0), (potato_tiles_bridgewatch, 1.0), (cabbage_tiles_bridgewatch, 1.0), (turnip_tiles_bridgewatch, 1.0), (wheat_tiles_bridgewatch, 1.0), (bean_tiles_bridgewatch, 1.0), (carrot_tiles_bridgewatch, 1.0), (farm_tiles_bridgewatch, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(yarrow_tiles_caerleon, 1.0), (muellin_tiles_caerleon, 1.0), (foxglove_tiles_caerleon, 1.0), (teasel_tiles_caerleon, 1.0), (burdock_tiles_caerleon, 1.0), (comfrey_tiles_caerleon, 1.0), (agaric_tiles_caerleon, 1.0), (herb_garden_tiles_caerleon, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(cow_tiles_caerleon, 1.0), (pig_tiles_caerleon, 1.0), (sheep_tiles_caerleon, 1.0), (goose_tiles_caerleon, 1.0), (goat_tiles_caerleon, 1.0), (chicken_tiles_caerleon, 1.0), (calf_tiles_caerleon, 1.0), (piglet_tiles_caerleon, 1.0), (lamb_tiles_caerleon, 1.0), (gosling_tiles_caerleon, 1.0), (kid_tiles_caerleon, 1.0), (baby_chicken_tiles_caerleon, 1.0), (pasture_tiles_caerleon, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(pumpkin_tiles_caerleon, 1.0), (corn_tiles_caerleon, 1.0), (potato_tiles_caerleon, 1.0), (cabbage_tiles_caerleon, 1.0), (turnip_tiles_caerleon, 1.0), (wheat_tiles_caerleon, 1.0), (bean_tiles_caerleon, 1.0), (carrot_tiles_caerleon, 1.0), (farm_tiles_caerleon, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(yarrow_tiles_fort_sterling, 1.0), (muellin_tiles_fort_sterling, 1.0), (foxglove_tiles_fort_sterling, 1.0), (teasel_tiles_fort_sterling, 1.0), (burdock_tiles_fort_sterling, 1.0), (comfrey_tiles_fort_sterling, 1.0), (agaric_tiles_fort_sterling, 1.0), (herb_garden_tiles_fort_sterling, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(cow_tiles_fort_sterling, 1.0), (pig_tiles_fort_sterling, 1.0), (sheep_tiles_fort_sterling, 1.0), (goose_tiles_fort_sterling, 1.0), (goat_tiles_fort_sterling, 1.0), (chicken_tiles_fort_sterling, 1.0), (calf_tiles_fort_sterling, 1.0), (piglet_tiles_fort_sterling, 1.0), (lamb_tiles_fort_sterling, 1.0), (gosling_tiles_fort_sterling, 1.0), (kid_tiles_fort_sterling, 1.0), (baby_chicken_tiles_fort_sterling, 1.0), (pasture_tiles_fort_sterling, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(pumpkin_tiles_fort_sterling, 1.0), (corn_tiles_fort_sterling, 1.0), (potato_tiles_fort_sterling, 1.0), (cabbage_tiles_fort_sterling, 1.0), (turnip_tiles_fort_sterling, 1.0), (wheat_tiles_fort_sterling, 1.0), (bean_tiles_fort_sterling, 1.0), (carrot_tiles_fort_sterling, 1.0), (farm_tiles_fort_sterling, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(yarrow_tiles_lymhurst, 1.0), (muellin_tiles_lymhurst, 1.0), (foxglove_tiles_lymhurst, 1.0), (teasel_tiles_lymhurst, 1.0), (burdock_tiles_lymhurst, 1.0), (comfrey_tiles_lymhurst, 1.0), (agaric_tiles_lymhurst, 1.0), (herb_garden_tiles_lymhurst, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(cow_tiles_lymhurst, 1.0), (pig_tiles_lymhurst, 1.0), (sheep_tiles_lymhurst, 1.0), (goose_tiles_lymhurst, 1.0), (goat_tiles_lymhurst, 1.0), (chicken_tiles_lymhurst, 1.0), (calf_tiles_lymhurst, 1.0), (piglet_tiles_lymhurst, 1.0), (lamb_tiles_lymhurst, 1.0), (gosling_tiles_lymhurst, 1.0), (kid_tiles_lymhurst, 1.0), (baby_chicken_tiles_lymhurst, 1.0), (pasture_tiles_lymhurst, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(pumpkin_tiles_lymhurst, 1.0), (corn_tiles_lymhurst, 1.0), (potato_tiles_lymhurst, 1.0), (cabbage_tiles_lymhurst, 1.0), (turnip_tiles_lymhurst, 1.0), (wheat_tiles_lymhurst, 1.0), (bean_tiles_lymhurst, 1.0), (carrot_tiles_lymhurst, 1.0), (farm_tiles_lymhurst, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(yarrow_tiles_martlock, 1.0), (muellin_tiles_martlock, 1.0), (foxglove_tiles_martlock, 1.0), (teasel_tiles_martlock, 1.0), (burdock_tiles_martlock, 1.0), (comfrey_tiles_martlock, 1.0), (agaric_tiles_martlock, 1.0), (herb_garden_tiles_martlock, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(cow_tiles_martlock, 1.0), (pig_tiles_martlock, 1.0), (sheep_tiles_martlock, 1.0), (goose_tiles_martlock, 1.0), (goat_tiles_martlock, 1.0), (chicken_tiles_martlock, 1.0), (calf_tiles_martlock, 1.0), (piglet_tiles_martlock, 1.0), (lamb_tiles_martlock, 1.0), (gosling_tiles_martlock, 1.0), (kid_tiles_martlock, 1.0), (baby_chicken_tiles_martlock, 1.0), (pasture_tiles_martlock, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(pumpkin_tiles_martlock, 1.0), (corn_tiles_martlock, 1.0), (potato_tiles_martlock, 1.0), (cabbage_tiles_martlock, 1.0), (turnip_tiles_martlock, 1.0), (wheat_tiles_martlock, 1.0), (bean_tiles_martlock, 1.0), (carrot_tiles_martlock, 1.0), (farm_tiles_martlock, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(yarrow_tiles_thetford, 1.0), (muellin_tiles_thetford, 1.0), (foxglove_tiles_thetford, 1.0), (teasel_tiles_thetford, 1.0), (burdock_tiles_thetford, 1.0), (comfrey_tiles_thetford, 1.0), (agaric_tiles_thetford, 1.0), (herb_garden_tiles_thetford, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(cow_tiles_thetford, 1.0), (pig_tiles_thetford, 1.0), (sheep_tiles_thetford, 1.0), (goose_tiles_thetford, 1.0), (goat_tiles_thetford, 1.0), (chicken_tiles_thetford, 1.0), (calf_tiles_thetford, 1.0), (piglet_tiles_thetford, 1.0), (lamb_tiles_thetford, 1.0), (gosling_tiles_thetford, 1.0), (kid_tiles_thetford, 1.0), (baby_chicken_tiles_thetford, 1.0), (pasture_tiles_thetford, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(pumpkin_tiles_thetford, 1.0), (corn_tiles_thetford, 1.0), (potato_tiles_thetford, 1.0), (cabbage_tiles_thetford, 1.0), (turnip_tiles_thetford, 1.0), (wheat_tiles_thetford, 1.0), (bean_tiles_thetford, 1.0), (carrot_tiles_thetford, 1.0), (farm_tiles_thetford, -1.0)], ComparisonOp::Le, 0.0);

    //// filled tiles to products constraints
    ////// herb constraints
    problem.add_constraint(&[(agaric_herbs, 1.0), (agaric_tiles_brecilien, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (agaric_tiles_bridgewatch, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (agaric_tiles_caerleon, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (agaric_tiles_fort_sterling, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (agaric_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (agaric_tiles_martlock, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (agaric_tiles_thetford, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(comfrey_herbs, 1.0), (comfrey_tiles_brecilien, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (comfrey_tiles_bridgewatch, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (comfrey_tiles_caerleon, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (comfrey_tiles_fort_sterling, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (comfrey_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (comfrey_tiles_martlock, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (comfrey_tiles_thetford, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(burdock_herbs, 1.0), (burdock_tiles_brecilien, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (burdock_tiles_bridgewatch, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (burdock_tiles_caerleon, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (burdock_tiles_fort_sterling, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (burdock_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (burdock_tiles_martlock, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (burdock_tiles_thetford, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(teasel_herbs, 1.0), (teasel_tiles_brecilien, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (teasel_tiles_bridgewatch, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (teasel_tiles_caerleon, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (teasel_tiles_fort_sterling, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (teasel_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (teasel_tiles_martlock, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (teasel_tiles_thetford, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(foxglove_herbs, 1.0), (foxglove_tiles_brecilien, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (foxglove_tiles_bridgewatch, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (foxglove_tiles_caerleon, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (foxglove_tiles_fort_sterling, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (foxglove_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (foxglove_tiles_martlock, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (foxglove_tiles_thetford, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(muellin_herbs, 1.0), (muellin_tiles_brecilien, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (muellin_tiles_bridgewatch, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (muellin_tiles_caerleon, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (muellin_tiles_fort_sterling, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (muellin_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (muellin_tiles_martlock, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (muellin_tiles_thetford, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(yarrow_herbs, 1.0), (yarrow_tiles_brecilien, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (yarrow_tiles_bridgewatch, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (yarrow_tiles_caerleon, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (yarrow_tiles_fort_sterling, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (yarrow_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (yarrow_tiles_martlock, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (yarrow_tiles_thetford, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor)], ComparisonOp::Eq, 0.0);

    ////// crop constraints
    problem.add_constraint(&[(carrot_crops, 1.0), (carrot_tiles_brecilien, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (carrot_tiles_bridgewatch, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (carrot_tiles_caerleon, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (carrot_tiles_fort_sterling, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (carrot_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (carrot_tiles_martlock, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (carrot_tiles_thetford, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(bean_crops, 1.0), (bean_tiles_brecilien, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (bean_tiles_bridgewatch, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (bean_tiles_caerleon, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (bean_tiles_fort_sterling, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (bean_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (bean_tiles_martlock, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (bean_tiles_thetford, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(wheat_crops, 1.0), (wheat_tiles_brecilien, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (baby_chicken_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (chicken_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (wheat_tiles_bridgewatch, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (baby_chicken_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (chicken_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (wheat_tiles_caerleon, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (baby_chicken_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (chicken_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (wheat_tiles_fort_sterling, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (baby_chicken_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (chicken_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (wheat_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (baby_chicken_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (chicken_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (wheat_tiles_martlock, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (baby_chicken_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (chicken_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (wheat_tiles_thetford, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (baby_chicken_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (chicken_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(turnip_crops, 1.0), (turnip_tiles_brecilien, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (kid_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (goat_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (turnip_tiles_bridgewatch, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (kid_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (goat_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (turnip_tiles_caerleon, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (kid_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (goat_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (turnip_tiles_fort_sterling, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (kid_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (goat_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (turnip_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (kid_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (goat_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (turnip_tiles_martlock, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (kid_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (goat_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (turnip_tiles_thetford, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (kid_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (goat_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(cabbage_crops, 1.0), (cabbage_tiles_brecilien, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (gosling_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (goose_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (cabbage_tiles_bridgewatch, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (gosling_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (goose_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (cabbage_tiles_caerleon, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (gosling_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (goose_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (cabbage_tiles_fort_sterling, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (gosling_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (goose_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (cabbage_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (gosling_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (goose_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (cabbage_tiles_martlock, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (gosling_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (goose_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (cabbage_tiles_thetford, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (gosling_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (goose_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(potato_crops, 1.0), (potato_tiles_brecilien, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (lamb_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (sheep_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (potato_tiles_bridgewatch, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (lamb_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (sheep_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (potato_tiles_caerleon, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (lamb_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (sheep_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (potato_tiles_fort_sterling, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (lamb_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (sheep_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (potato_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (lamb_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (sheep_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (potato_tiles_martlock, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (lamb_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (sheep_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (potato_tiles_thetford, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (lamb_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (sheep_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(corn_crops, 1.0), (corn_tiles_brecilien, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (piglet_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (pig_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (corn_tiles_bridgewatch, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (piglet_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (pig_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (corn_tiles_caerleon, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (piglet_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (pig_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (corn_tiles_fort_sterling, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (piglet_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (pig_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (corn_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (piglet_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (pig_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (corn_tiles_martlock, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (piglet_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (pig_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (corn_tiles_thetford, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (piglet_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (pig_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(pumpkin_crops, 1.0), (pumpkin_tiles_brecilien, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (calf_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (cow_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (pumpkin_tiles_bridgewatch, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (calf_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (cow_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (pumpkin_tiles_caerleon, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (calf_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (cow_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (pumpkin_tiles_fort_sterling, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (calf_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (cow_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (pumpkin_tiles_lymhurst, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR), (calf_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (cow_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (pumpkin_tiles_martlock, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (calf_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (cow_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (pumpkin_tiles_thetford, -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor), (calf_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION), (cow_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION)], ComparisonOp::Eq, 0.0);

    ////// animal product constraints
    problem.add_constraint(&[(hen_eggs, 1.0), (chicken_tiles_brecilien, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (chicken_tiles_bridgewatch, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (chicken_tiles_caerleon, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (chicken_tiles_fort_sterling, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor * BONUS_FACTOR), (chicken_tiles_lymhurst, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (chicken_tiles_martlock, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (chicken_tiles_thetford, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(goats_milk, 1.0), (goat_tiles_brecilien, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (goat_tiles_bridgewatch, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor * BONUS_FACTOR), (goat_tiles_caerleon, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (goat_tiles_fort_sterling, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (goat_tiles_lymhurst, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (goat_tiles_martlock, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (goat_tiles_thetford, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(goose_eggs, 1.0), (goose_tiles_brecilien, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (goose_tiles_bridgewatch, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (goose_tiles_caerleon, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (goose_tiles_fort_sterling, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (goose_tiles_lymhurst, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor * BONUS_FACTOR), (goose_tiles_martlock, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (goose_tiles_thetford, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(sheeps_milk, 1.0), (sheep_tiles_brecilien, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (sheep_tiles_bridgewatch, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (sheep_tiles_caerleon, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (sheep_tiles_fort_sterling, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor * BONUS_FACTOR), (sheep_tiles_lymhurst, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (sheep_tiles_martlock, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (sheep_tiles_thetford, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(cows_milk, 1.0), (cow_tiles_brecilien, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (cow_tiles_bridgewatch, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (cow_tiles_caerleon, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (cow_tiles_fort_sterling, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (cow_tiles_lymhurst, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor), (cow_tiles_martlock, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor * BONUS_FACTOR), (cow_tiles_thetford, -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor)], ComparisonOp::Eq, 0.0);

    ////// meat constraints
    problem.add_constraint(&[(raw_chicken, 1.0), (baby_chicken_tiles_brecilien, -1.0 * MEAT_FACTOR), (baby_chicken_tiles_bridgewatch, -1.0 * MEAT_FACTOR), (baby_chicken_tiles_caerleon, -1.0 * MEAT_FACTOR), (baby_chicken_tiles_fort_sterling, -1.0 * MEAT_FACTOR * BONUS_FACTOR), (baby_chicken_tiles_lymhurst, -1.0 * MEAT_FACTOR), (baby_chicken_tiles_martlock, -1.0 * MEAT_FACTOR), (baby_chicken_tiles_thetford, -1.0 * MEAT_FACTOR)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(raw_goat, 1.0), (kid_tiles_brecilien, -1.0 * MEAT_FACTOR), (kid_tiles_bridgewatch, -1.0 * MEAT_FACTOR * BONUS_FACTOR), (kid_tiles_caerleon, -1.0 * MEAT_FACTOR), (kid_tiles_fort_sterling, -1.0 * MEAT_FACTOR), (kid_tiles_lymhurst, -1.0 * MEAT_FACTOR), (kid_tiles_martlock, -1.0 * MEAT_FACTOR), (kid_tiles_thetford, -1.0 * MEAT_FACTOR)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(raw_goose, 1.0), (gosling_tiles_brecilien, -1.0 * MEAT_FACTOR), (gosling_tiles_bridgewatch, -1.0 * MEAT_FACTOR), (gosling_tiles_caerleon, -1.0 * MEAT_FACTOR), (gosling_tiles_fort_sterling, -1.0 * MEAT_FACTOR), (gosling_tiles_lymhurst, -1.0 * MEAT_FACTOR * BONUS_FACTOR), (gosling_tiles_martlock, -1.0 * MEAT_FACTOR), (gosling_tiles_thetford, -1.0 * MEAT_FACTOR)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(raw_mutton, 1.0), (lamb_tiles_brecilien, -1.0 * MEAT_FACTOR), (lamb_tiles_bridgewatch, -1.0 * MEAT_FACTOR), (lamb_tiles_caerleon, -1.0 * MEAT_FACTOR), (lamb_tiles_fort_sterling, -1.0 * MEAT_FACTOR * BONUS_FACTOR), (lamb_tiles_lymhurst, -1.0 * MEAT_FACTOR), (lamb_tiles_martlock, -1.0 * MEAT_FACTOR), (lamb_tiles_thetford, -1.0 * MEAT_FACTOR)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(raw_pork, 1.0), (piglet_tiles_brecilien, -1.0 * MEAT_FACTOR), (piglet_tiles_bridgewatch, -1.0 * MEAT_FACTOR), (piglet_tiles_caerleon, -1.0 * MEAT_FACTOR), (piglet_tiles_fort_sterling, -1.0 * MEAT_FACTOR), (piglet_tiles_lymhurst, -1.0 * MEAT_FACTOR), (piglet_tiles_martlock, -1.0 * MEAT_FACTOR), (piglet_tiles_thetford, -1.0 * MEAT_FACTOR * BONUS_FACTOR)], ComparisonOp::Eq, 0.0);
    problem.add_constraint(&[(raw_beef, 1.0), (calf_tiles_brecilien, -1.0 * MEAT_FACTOR), (calf_tiles_bridgewatch, -1.0 * MEAT_FACTOR), (calf_tiles_caerleon, -1.0 * MEAT_FACTOR), (calf_tiles_fort_sterling, -1.0 * MEAT_FACTOR), (calf_tiles_lymhurst, -1.0 * MEAT_FACTOR), (calf_tiles_martlock, -1.0 * MEAT_FACTOR * BONUS_FACTOR), (calf_tiles_thetford, -1.0 * MEAT_FACTOR)], ComparisonOp::Eq, 0.0);

    ////// recipe constraints
    // TODO: fix recipe constraints to work like the commented out section above

    match context.target {
        Product::MinorEnergyPotion => {
            // problem.add_constraint(&[(minor_energy_potion, POTIONS_PER_CRAFT), (agaric_herbs, -1.0 * L04_INGREDIENT)], ComparisonOp::Eq, 0.0);
            
        },
        Product::MinorHealingPotion => {
            // problem.add_constraint(&[(minor_healing_potion, POTIONS_PER_CRAFT), (agaric_herbs, -1.0 * L04_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::MinorGigantifyPotion => {
            // problem.add_constraint(&[(minor_gigantify_potion, POTIONS_PER_CRAFT), (comfrey_herbs, -1.0 * L04_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::MinorResistancePotion => {
            // problem.add_constraint(&[(minor_resistance_potion, POTIONS_PER_CRAFT), (comfrey_herbs, -1.0 * L04_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::MinorStickyPotion => {
            // problem.add_constraint(&[(minor_sticky_potion, POTIONS_PER_CRAFT), (comfrey_herbs, -1.0 * L04_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::MinorPoisonPotion => {
            // problem.add_constraint(&[(minor_poison_potion, POTIONS_PER_CRAFT), (burdock_herbs, -1.0 * L04_INGREDIENT), (comfrey_herbs, -1.0 * L02_INGREDIENT)], ComparisonOp::Eq, 0.0);
        },
        Product::EnergyPotion => {
            // problem.add_constraint(&[(energy_potion, POTIONS_PER_CRAFT), (burdock_herbs , -1.0 * L08_INGREDIENT), (goats_milk, -1.0 * L03_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::HealingPotion => {
            // problem.add_constraint(&[(healing_potion, POTIONS_PER_CRAFT), (burdock_herbs, -1.0 * L08_INGREDIENT), (hen_eggs, -1.0 * L03_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::GigantifyPotion => {
            // problem.add_constraint(&[(gigantify_potion, POTIONS_PER_CRAFT), (teasel_herbs, -1.0 * L08_INGREDIENT), (burdock_herbs, -1.0 * L05_INGREDIENT), (goose_eggs, -1.0 * L03_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::ResistancePotion => {
            // problem.add_constraint(&[(resistance_potion, POTIONS_PER_CRAFT), (teasel_herbs, -1.0 * L08_INGREDIENT), (burdock_herbs, -1.0 * L05_INGREDIENT), (goats_milk, -1.0 * L03_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::StickyPotion => {
            // problem.add_constraint(&[(sticky_potion, POTIONS_PER_CRAFT), (teasel_herbs, -1.0 * L08_INGREDIENT), (burdock_herbs, -1.0 * L05_INGREDIENT), (goose_eggs, -1.0 * L03_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::PoisonPotion => {
            // problem.add_constraint(&[(poison_potion, POTIONS_PER_CRAFT), (foxglove_herbs, -1.0 * L08_INGREDIENT), (teasel_herbs, -1.0 * L05_INGREDIENT), (comfrey_herbs, -1.0 * L05_INGREDIENT), (sheeps_milk, -1.0 * L03_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::MajorEnergyPotion => {
            // problem.add_constraint(&[(major_energy_potion, POTIONS_PER_CRAFT), (foxglove_herbs, -1.0 * L11_INGREDIENT), (sheeps_milk, -1.0 * L07_INGREDIENT), (potato_crops, -1.0 * L07_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::MajorHealingPotion => {
            // problem.add_constraint(&[(major_healing_potion, POTIONS_PER_CRAFT), (foxglove_herbs, -1.0 * L11_INGREDIENT), (goose_eggs, -1.0 * L07_INGREDIENT), (potato_crops, -1.0 * L07_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::MajorGigantifyPotion => {
            // problem.add_constraint(&[(major_gigantify_potion, POTIONS_PER_CRAFT), (muellin_herbs, -1.0 * L11_INGREDIENT), (foxglove_herbs, -1.0 * L09_INGREDIENT), (goose_eggs, -1.0 * L07_INGREDIENT), (corn_crops, -1.0 * L07_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::MajorResistancePotion => {
            // problem.add_constraint(&[(major_resistance_potion, POTIONS_PER_CRAFT), (muellin_herbs, -1.0 * L11_INGREDIENT), (foxglove_herbs, -1.0 * L09_INGREDIENT), (burdock_herbs, -1.0 * L09_INGREDIENT), (sheeps_milk, -1.0 * L07_INGREDIENT), (corn_crops, -1.0 * L07_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::MajorStickyPotion => {
            // problem.add_constraint(&[(major_sticky_potion, POTIONS_PER_CRAFT), (muellin_herbs, -1.0 * L11_INGREDIENT), (foxglove_herbs, -1.0 * L09_INGREDIENT), (burdock_herbs, -1.0 * L09_INGREDIENT), (goose_eggs, -1.0 * L07_INGREDIENT), (corn_crops, -1.0 * L07_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::MajorPoisonPotion => {
            // problem.add_constraint(&[(major_poison_potion, POTIONS_PER_CRAFT), (yarrow_herbs, -1.0 * L11_INGREDIENT), (muellin_herbs, -1.0 * L09_INGREDIENT), (teasel_herbs, -1.0 * L09_INGREDIENT), (cows_milk, -1.0 * L07_INGREDIENT), (pumpkin_crops, -1.0 * L07_INGREDIENT)], ComparisonOp::Eq, 0.0);
            problem.add_constraint(&[(cows_milk, 1.0 * L07_INGREDIENT), (pumpkin_crops, -1.0 * L07_INGREDIENT)], ComparisonOp::Eq, 0.0);
            problem.add_constraint(&[(teasel_herbs, 1.0 * L07_INGREDIENT), (cows_milk, -1.0 * L09_INGREDIENT)], ComparisonOp::Eq, 0.0);
            problem.add_constraint(&[(muellin_herbs, 1.0 * L09_INGREDIENT), (teasel_herbs, -1.0 * L09_INGREDIENT)], ComparisonOp::Eq, 0.0);
            problem.add_constraint(&[(yarrow_herbs, 1.0 * L09_INGREDIENT), (muellin_herbs, -1.0 * L11_INGREDIENT)], ComparisonOp::Eq, 0.0);
            problem.add_constraint(&[(major_poison_potion, 1.0 * L11_INGREDIENT), (yarrow_herbs, -1.0 * POTIONS_PER_CRAFT)], ComparisonOp::Eq, 0.0);        
        },
        Product::InvisibilityPotion => {
            // problem.add_constraint(&[(invisibility_potion, POTIONS_PER_CRAFT), (yarrow_herbs, -1.0 * L11_INGREDIENT), (muellin_herbs, -1.0 * L09_INGREDIENT), (teasel_herbs, -1.0 * L09_INGREDIENT), (cows_milk, -1.0 * L07_INGREDIENT), (pumpkin_crops, -1.0 * L07_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::ChickenOmelette => {
            // problem.add_constraint(&[(chicken_omelette, FOOD_PER_CRAFT), (wheat_crops, -1.0 * L02_INGREDIENT), (raw_chicken, -1.0 * L04_INGREDIENT), (hen_eggs, -1.0 * L01_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::GooseOmelette => {
            // problem.add_constraint(&[(goose_omelette, FOOD_PER_CRAFT), (cabbage_crops, -1.0 * L05_INGREDIENT), (raw_goose, -1.0 * L08_INGREDIENT), (goose_eggs, -1.0 * L03_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::PorkOmelette => {
            // problem.add_constraint(&[(pork_omelette, FOOD_PER_CRAFT), (corn_crops, -1.0 * L09_INGREDIENT), (raw_pork, -1.0 * L11_INGREDIENT), (goose_eggs, -1.0 * L07_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::BeanSalad => {
            // problem.add_constraint(&[(bean_salad, FOOD_PER_CRAFT), (bean_crops, -1.0 * L04_INGREDIENT), (carrot_crops, -1.0 * L04_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::TurnipSalad => {
            // problem.add_constraint(&[(turnip_salad, FOOD_PER_CRAFT), (turnip_crops, -1.0 * L08_INGREDIENT), (wheat_crops, -1.0 * L08_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::PotatoSalad => {
            // problem.add_constraint(&[(potato_salad, FOOD_PER_CRAFT), (potato_crops, -1.0 * L11_INGREDIENT), (cabbage_crops, -1.0 * L11_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::GoatSandwich => {
            // problem.add_constraint(&[(goat_sandwich, FOOD_PER_CRAFT), (wheat_crops, -1.0 * L02_INGREDIENT), (raw_goat, -1.0 * L04_INGREDIENT), (goats_milk, -1.0 * L01_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::MuttonSandwich => {
            // problem.add_constraint(&[(mutton_sandwich, FOOD_PER_CRAFT), (wheat_crops, -1.0 * L05_INGREDIENT), (raw_mutton, -1.0 * L08_INGREDIENT), (sheeps_milk, -1.0 * L03_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::BeefSandwich => {
            // problem.add_constraint(&[(beef_sandwich, FOOD_PER_CRAFT), (wheat_crops, -1.0 * L09_INGREDIENT), (raw_beef, -1.0 * L11_INGREDIENT), (cows_milk, -1.0 * L07_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::CarrotSoup => {
            // problem.add_constraint(&[(carrot_soup, FOOD_PER_CRAFT), (carrot_crops, -1.0 * L06_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::WheatSoup => {
            // problem.add_constraint(&[(wheat_soup, FOOD_PER_CRAFT), (wheat_crops, -1.0 * L10_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::CabbageSoup => {
            // problem.add_constraint(&[(cabbage_soup, FOOD_PER_CRAFT), (cabbage_crops, -1.0 * L12_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::GoatStew => {
            // problem.add_constraint(&[(goat_stew, FOOD_PER_CRAFT), (wheat_crops, -1.0 * L02_INGREDIENT), (raw_goat, -1.0 * L04_INGREDIENT), (turnip_crops, -1.0 * L02_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::MuttonStew => {
            // problem.add_constraint(&[(mutton_stew, FOOD_PER_CRAFT), (wheat_crops, -1.0 * L05_INGREDIENT), (raw_mutton, -1.0 * L08_INGREDIENT), (potato_crops, -1.0 * L05_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::BeefStew => {
            // problem.add_constraint(&[(beef_stew, FOOD_PER_CRAFT), (pumpkin_crops, -1.0 * L09_INGREDIENT), (raw_beef, -1.0 * L11_INGREDIENT), (wheat_crops, -1.0 * L09_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::RoastChicken => {
            // problem.add_constraint(&[(roast_chicken, FOOD_PER_CRAFT), (raw_chicken, -1.0 * L04_INGREDIENT), (bean_crops, -1.0 * L02_INGREDIENT), (goats_milk, -1.0 * L02_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::RoastGoose => {
            // problem.add_constraint(&[(roast_goose, FOOD_PER_CRAFT), (raw_goose, -1.0 * L08_INGREDIENT), (cabbage_crops, -1.0 * L05_INGREDIENT), (sheeps_milk, -1.0 * L05_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
        Product::RoastPork => {
            // problem.add_constraint(&[(roast_pork, FOOD_PER_CRAFT), (raw_pork, -1.0 * L11_INGREDIENT), (corn_crops, -1.0 * L09_INGREDIENT), (cows_milk, -1.0 * L09_INGREDIENT)], ComparisonOp::Eq, 0.0);

        },
    }

    let solution = problem.solve().unwrap();
    println!("{}", solution[teasel_tiles_lymhurst]);
    solution
}
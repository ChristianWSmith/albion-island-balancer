use minilp::{Problem, OptimizationDirection, ComparisonOp, Solution};


use crate::{constants::*, types::{PlotPlan, Product}, ModelContext};

 
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

pub fn optimize_plots(context: ModelContext) -> PlotPlan {
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
    let mut minor_energy_potion_objective_value: f64 = 0.0;
    let mut minor_healing_potion_objective_value: f64 = 0.0;
    let mut minor_gigantify_potion_objective_value: f64 = 0.0;
    let mut minor_resistance_potion_objective_value: f64 = 0.0;
    let mut minor_sticky_potion_objective_value: f64 = 0.0;
    let mut minor_poison_potion_objective_value: f64 = 0.0;
    let mut energy_potion_objective_value: f64 = 0.0;
    let mut healing_potion_objective_value: f64 = 0.0;
    let mut gigantify_potion_objective_value: f64 = 0.0;
    let mut resistance_potion_objective_value: f64 = 0.0;
    let mut sticky_potion_objective_value: f64 = 0.0;
    let mut poison_potion_objective_value: f64 = 0.0;
    let mut major_energy_potion_objective_value: f64 = 0.0;
    let mut major_healing_potion_objective_value: f64 = 0.0;
    let mut major_gigantify_potion_objective_value: f64 = 0.0;
    let mut major_resistance_potion_objective_value: f64 = 0.0;
    let mut major_sticky_potion_objective_value: f64 = 0.0;
    let mut major_poison_potion_objective_value: f64 = 0.0;
    let mut invisibility_potion_objective_value: f64 = 0.0;
    let mut chicken_omelette_objective_value: f64 = 0.0;
    let mut goose_omelette_objective_value: f64 = 0.0;
    let mut pork_omelette_objective_value: f64 = 0.0;
    let mut bean_salad_objective_value: f64 = 0.0;
    let mut turnip_salad_objective_value: f64 = 0.0;
    let mut potato_salad_objective_value: f64 = 0.0;
    let mut goat_sandwich_objective_value: f64 = 0.0;
    let mut mutton_sandwich_objective_value: f64 = 0.0;
    let mut beef_sandwich_objective_value: f64 = 0.0;
    let mut carrot_soup_objective_value: f64 = 0.0;
    let mut wheat_soup_objective_value: f64 = 0.0;
    let mut cabbage_soup_objective_value: f64 = 0.0;
    let mut goat_stew_objective_value: f64 = 0.0;
    let mut mutton_stew_objective_value: f64 = 0.0;
    let mut beef_stew_objective_value: f64 = 0.0;
    let mut roast_chicken_objective_value: f64 = 0.0;
    let mut roast_goose_objective_value: f64 = 0.0;
    let mut roast_pork_objective_value: f64 = 0.0;

    match context.target {
        Product::MinorEnergyPotion => {
            minor_energy_potion_objective_value = 1.0;
            
        },
        Product::MinorHealingPotion => {
            minor_healing_potion_objective_value = 1.0;

        },
        Product::MinorGigantifyPotion => {
            minor_gigantify_potion_objective_value = 1.0;

        },
        Product::MinorResistancePotion => {
            minor_resistance_potion_objective_value = 1.0;

        },
        Product::MinorStickyPotion => {
            minor_sticky_potion_objective_value = 1.0;

        },
        Product::MinorPoisonPotion => {
            minor_poison_potion_objective_value = 1.0;
        },
        Product::EnergyPotion => {
            energy_potion_objective_value = 1.0;

        },
        Product::HealingPotion => {
            healing_potion_objective_value = 1.0;

        },
        Product::GigantifyPotion => {
            gigantify_potion_objective_value = 1.0;

        },
        Product::ResistancePotion => {
            resistance_potion_objective_value = 1.0;

        },
        Product::StickyPotion => {
            sticky_potion_objective_value = 1.0;

        },
        Product::PoisonPotion => {
            poison_potion_objective_value = 1.0;

        },
        Product::MajorEnergyPotion => {
            major_energy_potion_objective_value = 1.0;

        },
        Product::MajorHealingPotion => {
            major_healing_potion_objective_value = 1.0;

        },
        Product::MajorGigantifyPotion => {
            major_gigantify_potion_objective_value = 1.0;

        },
        Product::MajorResistancePotion => {
            major_resistance_potion_objective_value = 1.0;

        },
        Product::MajorStickyPotion => {
            major_sticky_potion_objective_value = 1.0;

        },
        Product::MajorPoisonPotion => {
            major_poison_potion_objective_value = 1.0;
        },
        Product::InvisibilityPotion => {
            invisibility_potion_objective_value = 1.0;

        },
        Product::ChickenOmelette => {
            chicken_omelette_objective_value = 1.0;

        },
        Product::GooseOmelette => {
            goose_omelette_objective_value = 1.0;

        },
        Product::PorkOmelette => {
            pork_omelette_objective_value = 1.0;

        },
        Product::BeanSalad => {
            bean_salad_objective_value = 1.0;

        },
        Product::TurnipSalad => {
            turnip_salad_objective_value = 1.0;

        },
        Product::PotatoSalad => {
            potato_salad_objective_value = 1.0;
        },
        Product::GoatSandwich => {
            goat_sandwich_objective_value = 1.0;
        },
        Product::MuttonSandwich => {
            mutton_sandwich_objective_value = 1.0;
        },
        Product::BeefSandwich => {
            beef_sandwich_objective_value = 1.0;
        },
        Product::CarrotSoup => {
            carrot_soup_objective_value = 1.0;
        },
        Product::WheatSoup => {
            wheat_soup_objective_value = 1.0;
        },
        Product::CabbageSoup => {
            cabbage_soup_objective_value = 1.0;
        },
        Product::GoatStew => {
            goat_stew_objective_value = 1.0;
        },
        Product::MuttonStew => {
            mutton_stew_objective_value = 1.0;
        },
        Product::BeefStew => {
            beef_stew_objective_value = 1.0;
        },
        Product::RoastChicken => {
            roast_chicken_objective_value = 1.0;
        },
        Product::RoastGoose => {
            roast_goose_objective_value = 1.0;
        },
        Product::RoastPork => {
            roast_pork_objective_value = 1.0;
        },
    }

    let minor_energy_potion = problem.add_var(minor_energy_potion_objective_value, (0.0, f64::INFINITY));
    let minor_healing_potion = problem.add_var(minor_healing_potion_objective_value, (0.0, f64::INFINITY));
    let minor_gigantify_potion = problem.add_var(minor_gigantify_potion_objective_value, (0.0, f64::INFINITY));
    let minor_resistance_potion = problem.add_var(minor_resistance_potion_objective_value, (0.0, f64::INFINITY));
    let minor_sticky_potion = problem.add_var(minor_sticky_potion_objective_value, (0.0, f64::INFINITY));
    let minor_poison_potion = problem.add_var(minor_poison_potion_objective_value, (0.0, f64::INFINITY));
    let energy_potion = problem.add_var(energy_potion_objective_value, (0.0, f64::INFINITY));
    let healing_potion = problem.add_var(healing_potion_objective_value, (0.0, f64::INFINITY));
    let gigantify_potion = problem.add_var(gigantify_potion_objective_value, (0.0, f64::INFINITY));
    let resistance_potion = problem.add_var(resistance_potion_objective_value, (0.0, f64::INFINITY));
    let sticky_potion = problem.add_var(sticky_potion_objective_value, (0.0, f64::INFINITY));
    let poison_potion = problem.add_var(poison_potion_objective_value, (0.0, f64::INFINITY));
    let major_energy_potion = problem.add_var(major_energy_potion_objective_value, (0.0, f64::INFINITY));
    let major_healing_potion = problem.add_var(major_healing_potion_objective_value, (0.0, f64::INFINITY));
    let major_gigantify_potion = problem.add_var(major_gigantify_potion_objective_value, (0.0, f64::INFINITY));
    let major_resistance_potion = problem.add_var(major_resistance_potion_objective_value, (0.0, f64::INFINITY));
    let major_sticky_potion = problem.add_var(major_sticky_potion_objective_value, (0.0, f64::INFINITY));
    let major_poison_potion = problem.add_var(major_poison_potion_objective_value, (0.0, f64::INFINITY));
    let invisibility_potion = problem.add_var(invisibility_potion_objective_value, (0.0, f64::INFINITY));
    let chicken_omelette = problem.add_var(chicken_omelette_objective_value, (0.0, f64::INFINITY));
    let goose_omelette = problem.add_var(goose_omelette_objective_value, (0.0, f64::INFINITY));
    let pork_omelette = problem.add_var(pork_omelette_objective_value, (0.0, f64::INFINITY));
    let bean_salad = problem.add_var(bean_salad_objective_value, (0.0, f64::INFINITY));
    let turnip_salad = problem.add_var(turnip_salad_objective_value, (0.0, f64::INFINITY));
    let potato_salad = problem.add_var(potato_salad_objective_value, (0.0, f64::INFINITY));
    let goat_sandwich = problem.add_var(goat_sandwich_objective_value, (0.0, f64::INFINITY));
    let mutton_sandwich = problem.add_var(mutton_sandwich_objective_value, (0.0, f64::INFINITY));
    let beef_sandwich = problem.add_var(beef_sandwich_objective_value, (0.0, f64::INFINITY));
    let carrot_soup = problem.add_var(carrot_soup_objective_value, (0.0, f64::INFINITY));
    let wheat_soup = problem.add_var(wheat_soup_objective_value, (0.0, f64::INFINITY));
    let cabbage_soup = problem.add_var(cabbage_soup_objective_value, (0.0, f64::INFINITY));
    let goat_stew = problem.add_var(goat_stew_objective_value, (0.0, f64::INFINITY));
    let mutton_stew = problem.add_var(mutton_stew_objective_value, (0.0, f64::INFINITY));
    let beef_stew = problem.add_var(beef_stew_objective_value, (0.0, f64::INFINITY));
    let roast_chicken = problem.add_var(roast_chicken_objective_value, (0.0, f64::INFINITY));
    let roast_goose = problem.add_var(roast_goose_objective_value, (0.0, f64::INFINITY));
    let roast_pork = problem.add_var(roast_pork_objective_value, (0.0, f64::INFINITY));
    
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
    problem.add_constraint(&[(herb_gardens_brecilien, 1.0), (farms_brecilien, 1.0), (pastures_brecilien, 1.0), (plots_brecilien, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(herb_gardens_bridgewatch, 1.0), (farms_bridgewatch, 1.0), (pastures_bridgewatch, 1.0), (plots_bridgewatch, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(herb_gardens_caerleon, 1.0), (farms_caerleon, 1.0), (pastures_caerleon, 1.0), (plots_caerleon, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(herb_gardens_fort_sterling, 1.0), (farms_fort_sterling, 1.0), (pastures_fort_sterling, 1.0), (plots_fort_sterling, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(herb_gardens_lymhurst, 1.0), (farms_lymhurst, 1.0), (pastures_lymhurst, 1.0), (plots_lymhurst, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(herb_gardens_martlock, 1.0), (farms_martlock, 1.0), (pastures_martlock, 1.0), (plots_martlock, -1.0)], ComparisonOp::Le, 0.0);
    problem.add_constraint(&[(herb_gardens_thetford, 1.0), (farms_thetford, 1.0), (pastures_thetford, 1.0), (plots_thetford, -1.0)], ComparisonOp::Le, 0.0);

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

    // constrain patches
    problem.add_constraint(&[(herb_gardens_brecilien, 1.0)], ComparisonOp::Eq, solution[herb_gardens_brecilien].floor());
    problem.add_constraint(&[(farms_brecilien, 1.0)], ComparisonOp::Eq, solution[farms_brecilien].floor());
    problem.add_constraint(&[(pastures_brecilien, 1.0)], ComparisonOp::Eq, solution[pastures_brecilien].floor());
    problem.add_constraint(&[(herb_gardens_bridgewatch, 1.0)], ComparisonOp::Eq, solution[herb_gardens_bridgewatch].floor());
    problem.add_constraint(&[(farms_bridgewatch, 1.0)], ComparisonOp::Eq, solution[farms_bridgewatch].floor());
    problem.add_constraint(&[(pastures_bridgewatch, 1.0)], ComparisonOp::Eq, solution[pastures_bridgewatch].floor());
    problem.add_constraint(&[(herb_gardens_caerleon, 1.0)], ComparisonOp::Eq, solution[herb_gardens_caerleon].floor());
    problem.add_constraint(&[(farms_caerleon, 1.0)], ComparisonOp::Eq, solution[farms_caerleon].floor());
    problem.add_constraint(&[(pastures_caerleon, 1.0)], ComparisonOp::Eq, solution[pastures_caerleon].floor());
    problem.add_constraint(&[(herb_gardens_fort_sterling, 1.0)], ComparisonOp::Eq, solution[herb_gardens_fort_sterling].floor());
    problem.add_constraint(&[(farms_fort_sterling, 1.0)], ComparisonOp::Eq, solution[farms_fort_sterling].floor());
    problem.add_constraint(&[(pastures_fort_sterling, 1.0)], ComparisonOp::Eq, solution[pastures_fort_sterling].floor());
    problem.add_constraint(&[(herb_gardens_lymhurst, 1.0)], ComparisonOp::Eq, solution[herb_gardens_lymhurst].floor());
    problem.add_constraint(&[(farms_lymhurst, 1.0)], ComparisonOp::Eq, solution[farms_lymhurst].floor());
    problem.add_constraint(&[(pastures_lymhurst, 1.0)], ComparisonOp::Eq, solution[pastures_lymhurst].floor());
    problem.add_constraint(&[(herb_gardens_martlock, 1.0)], ComparisonOp::Eq, solution[herb_gardens_martlock].floor());
    problem.add_constraint(&[(farms_martlock, 1.0)], ComparisonOp::Eq, solution[farms_martlock].floor());
    problem.add_constraint(&[(pastures_martlock, 1.0)], ComparisonOp::Eq, solution[pastures_martlock].floor());
    problem.add_constraint(&[(herb_gardens_thetford, 1.0)], ComparisonOp::Eq, solution[herb_gardens_thetford].floor());
    problem.add_constraint(&[(farms_thetford, 1.0)], ComparisonOp::Eq, solution[farms_thetford].floor());
    problem.add_constraint(&[(pastures_thetford, 1.0)], ComparisonOp::Eq, solution[pastures_thetford].floor());

    let solution = problem.solve().unwrap();

    let plot_plan = PlotPlan {
        output: solution.objective().floor(),

        herb_gardens_brecilien: solution[herb_gardens_brecilien].ceil(),
        farms_brecilien: solution[farms_brecilien].ceil(),
        pastures_brecilien: solution[pastures_brecilien].ceil(),
        herb_gardens_bridgewatch: solution[herb_gardens_bridgewatch].ceil(),
        farms_bridgewatch: solution[farms_bridgewatch].ceil(),
        pastures_bridgewatch: solution[pastures_bridgewatch].ceil(),
        herb_gardens_caerleon: solution[herb_gardens_caerleon].ceil(),
        farms_caerleon: solution[farms_caerleon].ceil(),
        pastures_caerleon: solution[pastures_caerleon].ceil(),
        herb_gardens_fort_sterling: solution[herb_gardens_fort_sterling].ceil(),
        farms_fort_sterling: solution[farms_fort_sterling].ceil(),
        pastures_fort_sterling: solution[pastures_fort_sterling].ceil(),
        herb_gardens_lymhurst: solution[herb_gardens_lymhurst].ceil(),
        farms_lymhurst: solution[farms_lymhurst].ceil(),
        pastures_lymhurst: solution[pastures_lymhurst].ceil(),
        herb_gardens_martlock: solution[herb_gardens_martlock].ceil(),
        farms_martlock: solution[farms_martlock].ceil(),
        pastures_martlock: solution[pastures_martlock].ceil(),
        herb_gardens_thetford: solution[herb_gardens_thetford].ceil(),
        farms_thetford: solution[farms_thetford].ceil(),
        pastures_thetford: solution[pastures_thetford].ceil(),
        
        agaric_tiles_brecilien: solution[agaric_tiles_brecilien].ceil(),
        comfrey_tiles_brecilien: solution[comfrey_tiles_brecilien].ceil(),
        burdock_tiles_brecilien: solution[burdock_tiles_brecilien].ceil(),
        teasel_tiles_brecilien: solution[teasel_tiles_brecilien].ceil(),
        foxglove_tiles_brecilien: solution[foxglove_tiles_brecilien].ceil(),
        muellin_tiles_brecilien: solution[muellin_tiles_brecilien].ceil(),
        yarrow_tiles_brecilien: solution[yarrow_tiles_brecilien].ceil(),
        carrot_tiles_brecilien: solution[carrot_tiles_brecilien].ceil(),
        bean_tiles_brecilien: solution[bean_tiles_brecilien].ceil(),
        wheat_tiles_brecilien: solution[wheat_tiles_brecilien].ceil(),
        turnip_tiles_brecilien: solution[turnip_tiles_brecilien].ceil(),
        cabbage_tiles_brecilien: solution[cabbage_tiles_brecilien].ceil(),
        potato_tiles_brecilien: solution[potato_tiles_brecilien].ceil(),
        corn_tiles_brecilien: solution[corn_tiles_brecilien].ceil(),
        pumpkin_tiles_brecilien: solution[pumpkin_tiles_brecilien].ceil(),
        baby_chicken_tiles_brecilien: solution[baby_chicken_tiles_brecilien].ceil(),
        kid_tiles_brecilien: solution[kid_tiles_brecilien].ceil(),
        gosling_tiles_brecilien: solution[gosling_tiles_brecilien].ceil(),
        lamb_tiles_brecilien: solution[lamb_tiles_brecilien].ceil(),
        piglet_tiles_brecilien: solution[piglet_tiles_brecilien].ceil(),
        calf_tiles_brecilien: solution[calf_tiles_brecilien].ceil(),
        chicken_tiles_brecilien: solution[chicken_tiles_brecilien].ceil(),
        goat_tiles_brecilien: solution[goat_tiles_brecilien].ceil(),
        goose_tiles_brecilien: solution[goose_tiles_brecilien].ceil(),
        sheep_tiles_brecilien: solution[sheep_tiles_brecilien].ceil(),
        pig_tiles_brecilien: solution[pig_tiles_brecilien].ceil(),
        cow_tiles_brecilien: solution[cow_tiles_brecilien].ceil(),
        agaric_tiles_bridgewatch: solution[agaric_tiles_bridgewatch].ceil(),
        comfrey_tiles_bridgewatch: solution[comfrey_tiles_bridgewatch].ceil(),
        burdock_tiles_bridgewatch: solution[burdock_tiles_bridgewatch].ceil(),
        teasel_tiles_bridgewatch: solution[teasel_tiles_bridgewatch].ceil(),
        foxglove_tiles_bridgewatch: solution[foxglove_tiles_bridgewatch].ceil(),
        muellin_tiles_bridgewatch: solution[muellin_tiles_bridgewatch].ceil(),
        yarrow_tiles_bridgewatch: solution[yarrow_tiles_bridgewatch].ceil(),
        carrot_tiles_bridgewatch: solution[carrot_tiles_bridgewatch].ceil(),
        bean_tiles_bridgewatch: solution[bean_tiles_bridgewatch].ceil(),
        wheat_tiles_bridgewatch: solution[wheat_tiles_bridgewatch].ceil(),
        turnip_tiles_bridgewatch: solution[turnip_tiles_bridgewatch].ceil(),
        cabbage_tiles_bridgewatch: solution[cabbage_tiles_bridgewatch].ceil(),
        potato_tiles_bridgewatch: solution[potato_tiles_bridgewatch].ceil(),
        corn_tiles_bridgewatch: solution[corn_tiles_bridgewatch].ceil(),
        pumpkin_tiles_bridgewatch: solution[pumpkin_tiles_bridgewatch].ceil(),
        baby_chicken_tiles_bridgewatch: solution[baby_chicken_tiles_bridgewatch].ceil(),
        kid_tiles_bridgewatch: solution[kid_tiles_bridgewatch].ceil(),
        gosling_tiles_bridgewatch: solution[gosling_tiles_bridgewatch].ceil(),
        lamb_tiles_bridgewatch: solution[lamb_tiles_bridgewatch].ceil(),
        piglet_tiles_bridgewatch: solution[piglet_tiles_bridgewatch].ceil(),
        calf_tiles_bridgewatch: solution[calf_tiles_bridgewatch].ceil(),
        chicken_tiles_bridgewatch: solution[chicken_tiles_bridgewatch].ceil(),
        goat_tiles_bridgewatch: solution[goat_tiles_bridgewatch].ceil(),
        goose_tiles_bridgewatch: solution[goose_tiles_bridgewatch].ceil(),
        sheep_tiles_bridgewatch: solution[sheep_tiles_bridgewatch].ceil(),
        pig_tiles_bridgewatch: solution[pig_tiles_bridgewatch].ceil(),
        cow_tiles_bridgewatch: solution[cow_tiles_bridgewatch].ceil(),
        agaric_tiles_caerleon: solution[agaric_tiles_caerleon].ceil(),
        comfrey_tiles_caerleon: solution[comfrey_tiles_caerleon].ceil(),
        burdock_tiles_caerleon: solution[burdock_tiles_caerleon].ceil(),
        teasel_tiles_caerleon: solution[teasel_tiles_caerleon].ceil(),
        foxglove_tiles_caerleon: solution[foxglove_tiles_caerleon].ceil(),
        muellin_tiles_caerleon: solution[muellin_tiles_caerleon].ceil(),
        yarrow_tiles_caerleon: solution[yarrow_tiles_caerleon].ceil(),
        carrot_tiles_caerleon: solution[carrot_tiles_caerleon].ceil(),
        bean_tiles_caerleon: solution[bean_tiles_caerleon].ceil(),
        wheat_tiles_caerleon: solution[wheat_tiles_caerleon].ceil(),
        turnip_tiles_caerleon: solution[turnip_tiles_caerleon].ceil(),
        cabbage_tiles_caerleon: solution[cabbage_tiles_caerleon].ceil(),
        potato_tiles_caerleon: solution[potato_tiles_caerleon].ceil(),
        corn_tiles_caerleon: solution[corn_tiles_caerleon].ceil(),
        pumpkin_tiles_caerleon: solution[pumpkin_tiles_caerleon].ceil(),
        baby_chicken_tiles_caerleon: solution[baby_chicken_tiles_caerleon].ceil(),
        kid_tiles_caerleon: solution[kid_tiles_caerleon].ceil(),
        gosling_tiles_caerleon: solution[gosling_tiles_caerleon].ceil(),
        lamb_tiles_caerleon: solution[lamb_tiles_caerleon].ceil(),
        piglet_tiles_caerleon: solution[piglet_tiles_caerleon].ceil(),
        calf_tiles_caerleon: solution[calf_tiles_caerleon].ceil(),
        chicken_tiles_caerleon: solution[chicken_tiles_caerleon].ceil(),
        goat_tiles_caerleon: solution[goat_tiles_caerleon].ceil(),
        goose_tiles_caerleon: solution[goose_tiles_caerleon].ceil(),
        sheep_tiles_caerleon: solution[sheep_tiles_caerleon].ceil(),
        pig_tiles_caerleon: solution[pig_tiles_caerleon].ceil(),
        cow_tiles_caerleon: solution[cow_tiles_caerleon].ceil(),
        agaric_tiles_fort_sterling: solution[agaric_tiles_fort_sterling].ceil(),
        comfrey_tiles_fort_sterling: solution[comfrey_tiles_fort_sterling].ceil(),
        burdock_tiles_fort_sterling: solution[burdock_tiles_fort_sterling].ceil(),
        teasel_tiles_fort_sterling: solution[teasel_tiles_fort_sterling].ceil(),
        foxglove_tiles_fort_sterling: solution[foxglove_tiles_fort_sterling].ceil(),
        muellin_tiles_fort_sterling: solution[muellin_tiles_fort_sterling].ceil(),
        yarrow_tiles_fort_sterling: solution[yarrow_tiles_fort_sterling].ceil(),
        carrot_tiles_fort_sterling: solution[carrot_tiles_fort_sterling].ceil(),
        bean_tiles_fort_sterling: solution[bean_tiles_fort_sterling].ceil(),
        wheat_tiles_fort_sterling: solution[wheat_tiles_fort_sterling].ceil(),
        turnip_tiles_fort_sterling: solution[turnip_tiles_fort_sterling].ceil(),
        cabbage_tiles_fort_sterling: solution[cabbage_tiles_fort_sterling].ceil(),
        potato_tiles_fort_sterling: solution[potato_tiles_fort_sterling].ceil(),
        corn_tiles_fort_sterling: solution[corn_tiles_fort_sterling].ceil(),
        pumpkin_tiles_fort_sterling: solution[pumpkin_tiles_fort_sterling].ceil(),
        baby_chicken_tiles_fort_sterling: solution[baby_chicken_tiles_fort_sterling].ceil(),
        kid_tiles_fort_sterling: solution[kid_tiles_fort_sterling].ceil(),
        gosling_tiles_fort_sterling: solution[gosling_tiles_fort_sterling].ceil(),
        lamb_tiles_fort_sterling: solution[lamb_tiles_fort_sterling].ceil(),
        piglet_tiles_fort_sterling: solution[piglet_tiles_fort_sterling].ceil(),
        calf_tiles_fort_sterling: solution[calf_tiles_fort_sterling].ceil(),
        chicken_tiles_fort_sterling: solution[chicken_tiles_fort_sterling].ceil(),
        goat_tiles_fort_sterling: solution[goat_tiles_fort_sterling].ceil(),
        goose_tiles_fort_sterling: solution[goose_tiles_fort_sterling].ceil(),
        sheep_tiles_fort_sterling: solution[sheep_tiles_fort_sterling].ceil(),
        pig_tiles_fort_sterling: solution[pig_tiles_fort_sterling].ceil(),
        cow_tiles_fort_sterling: solution[cow_tiles_fort_sterling].ceil(),
        agaric_tiles_lymhurst: solution[agaric_tiles_lymhurst].ceil(),
        comfrey_tiles_lymhurst: solution[comfrey_tiles_lymhurst].ceil(),
        burdock_tiles_lymhurst: solution[burdock_tiles_lymhurst].ceil(),
        teasel_tiles_lymhurst: solution[teasel_tiles_lymhurst].ceil(),
        foxglove_tiles_lymhurst: solution[foxglove_tiles_lymhurst].ceil(),
        muellin_tiles_lymhurst: solution[muellin_tiles_lymhurst].ceil(),
        yarrow_tiles_lymhurst: solution[yarrow_tiles_lymhurst].ceil(),
        carrot_tiles_lymhurst: solution[carrot_tiles_lymhurst].ceil(),
        bean_tiles_lymhurst: solution[bean_tiles_lymhurst].ceil(),
        wheat_tiles_lymhurst: solution[wheat_tiles_lymhurst].ceil(),
        turnip_tiles_lymhurst: solution[turnip_tiles_lymhurst].ceil(),
        cabbage_tiles_lymhurst: solution[cabbage_tiles_lymhurst].ceil(),
        potato_tiles_lymhurst: solution[potato_tiles_lymhurst].ceil(),
        corn_tiles_lymhurst: solution[corn_tiles_lymhurst].ceil(),
        pumpkin_tiles_lymhurst: solution[pumpkin_tiles_lymhurst].ceil(),
        baby_chicken_tiles_lymhurst: solution[baby_chicken_tiles_lymhurst].ceil(),
        kid_tiles_lymhurst: solution[kid_tiles_lymhurst].ceil(),
        gosling_tiles_lymhurst: solution[gosling_tiles_lymhurst].ceil(),
        lamb_tiles_lymhurst: solution[lamb_tiles_lymhurst].ceil(),
        piglet_tiles_lymhurst: solution[piglet_tiles_lymhurst].ceil(),
        calf_tiles_lymhurst: solution[calf_tiles_lymhurst].ceil(),
        chicken_tiles_lymhurst: solution[chicken_tiles_lymhurst].ceil(),
        goat_tiles_lymhurst: solution[goat_tiles_lymhurst].ceil(),
        goose_tiles_lymhurst: solution[goose_tiles_lymhurst].ceil(),
        sheep_tiles_lymhurst: solution[sheep_tiles_lymhurst].ceil(),
        pig_tiles_lymhurst: solution[pig_tiles_lymhurst].ceil(),
        cow_tiles_lymhurst: solution[cow_tiles_lymhurst].ceil(),
        agaric_tiles_martlock: solution[agaric_tiles_martlock].ceil(),
        comfrey_tiles_martlock: solution[comfrey_tiles_martlock].ceil(),
        burdock_tiles_martlock: solution[burdock_tiles_martlock].ceil(),
        teasel_tiles_martlock: solution[teasel_tiles_martlock].ceil(),
        foxglove_tiles_martlock: solution[foxglove_tiles_martlock].ceil(),
        muellin_tiles_martlock: solution[muellin_tiles_martlock].ceil(),
        yarrow_tiles_martlock: solution[yarrow_tiles_martlock].ceil(),
        carrot_tiles_martlock: solution[carrot_tiles_martlock].ceil(),
        bean_tiles_martlock: solution[bean_tiles_martlock].ceil(),
        wheat_tiles_martlock: solution[wheat_tiles_martlock].ceil(),
        turnip_tiles_martlock: solution[turnip_tiles_martlock].ceil(),
        cabbage_tiles_martlock: solution[cabbage_tiles_martlock].ceil(),
        potato_tiles_martlock: solution[potato_tiles_martlock].ceil(),
        corn_tiles_martlock: solution[corn_tiles_martlock].ceil(),
        pumpkin_tiles_martlock: solution[pumpkin_tiles_martlock].ceil(),
        baby_chicken_tiles_martlock: solution[baby_chicken_tiles_martlock].ceil(),
        kid_tiles_martlock: solution[kid_tiles_martlock].ceil(),
        gosling_tiles_martlock: solution[gosling_tiles_martlock].ceil(),
        lamb_tiles_martlock: solution[lamb_tiles_martlock].ceil(),
        piglet_tiles_martlock: solution[piglet_tiles_martlock].ceil(),
        calf_tiles_martlock: solution[calf_tiles_martlock].ceil(),
        chicken_tiles_martlock: solution[chicken_tiles_martlock].ceil(),
        goat_tiles_martlock: solution[goat_tiles_martlock].ceil(),
        goose_tiles_martlock: solution[goose_tiles_martlock].ceil(),
        sheep_tiles_martlock: solution[sheep_tiles_martlock].ceil(),
        pig_tiles_martlock: solution[pig_tiles_martlock].ceil(),
        cow_tiles_martlock: solution[cow_tiles_martlock].ceil(),
        agaric_tiles_thetford: solution[agaric_tiles_thetford].ceil(),
        comfrey_tiles_thetford: solution[comfrey_tiles_thetford].ceil(),
        burdock_tiles_thetford: solution[burdock_tiles_thetford].ceil(),
        teasel_tiles_thetford: solution[teasel_tiles_thetford].ceil(),
        foxglove_tiles_thetford: solution[foxglove_tiles_thetford].ceil(),
        muellin_tiles_thetford: solution[muellin_tiles_thetford].ceil(),
        yarrow_tiles_thetford: solution[yarrow_tiles_thetford].ceil(),
        carrot_tiles_thetford: solution[carrot_tiles_thetford].ceil(),
        bean_tiles_thetford: solution[bean_tiles_thetford].ceil(),
        wheat_tiles_thetford: solution[wheat_tiles_thetford].ceil(),
        turnip_tiles_thetford: solution[turnip_tiles_thetford].ceil(),
        cabbage_tiles_thetford: solution[cabbage_tiles_thetford].ceil(),
        potato_tiles_thetford: solution[potato_tiles_thetford].ceil(),
        corn_tiles_thetford: solution[corn_tiles_thetford].ceil(),
        pumpkin_tiles_thetford: solution[pumpkin_tiles_thetford].ceil(),
        baby_chicken_tiles_thetford: solution[baby_chicken_tiles_thetford].ceil(),
        kid_tiles_thetford: solution[kid_tiles_thetford].ceil(),
        gosling_tiles_thetford: solution[gosling_tiles_thetford].ceil(),
        lamb_tiles_thetford: solution[lamb_tiles_thetford].ceil(),
        piglet_tiles_thetford: solution[piglet_tiles_thetford].ceil(),
        calf_tiles_thetford: solution[calf_tiles_thetford].ceil(),
        chicken_tiles_thetford: solution[chicken_tiles_thetford].ceil(),
        goat_tiles_thetford: solution[goat_tiles_thetford].ceil(),
        goose_tiles_thetford: solution[goose_tiles_thetford].ceil(),
        sheep_tiles_thetford: solution[sheep_tiles_thetford].ceil(),
        pig_tiles_thetford: solution[pig_tiles_thetford].ceil(),
        cow_tiles_thetford: solution[cow_tiles_thetford].ceil(),
    };
    
    plot_plan
}
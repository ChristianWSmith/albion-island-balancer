use std::collections::HashMap;

use crate::{
    constants::*,
    types::{PlotPlan, Product},
    ModelContext,
};
use minilp::{ComparisonOp, OptimizationDirection, Problem, Variable};

pub fn optimize_plots(context: ModelContext) -> PlotPlan {
    let mut best_output = -f64::INFINITY;
    let mut best_plot_plan: Option<PlotPlan> = None;
    // minilp has some bugs, best approach is to just perform the
    // math a lot of times and take the best result
    for _ in 0..MINI_LP_RETRIES {
        let plot_plan = _optimize_plots(context);
        if plot_plan.is_ok() {
            let plot_plan_unwrapped = plot_plan.unwrap();
            if plot_plan_unwrapped.output > best_output {
                best_output = plot_plan_unwrapped.output;
                best_plot_plan = Some(plot_plan_unwrapped);
            }
        }
    }
    return best_plot_plan.unwrap();
}

fn _optimize_plots(context: ModelContext) -> core::result::Result<PlotPlan, minilp::Error> {
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
        }
        Product::MinorHealingPotion => {
            minor_healing_potion_objective_value = 1.0;
        }
        Product::MinorGigantifyPotion => {
            minor_gigantify_potion_objective_value = 1.0;
        }
        Product::MinorResistancePotion => {
            minor_resistance_potion_objective_value = 1.0;
        }
        Product::MinorStickyPotion => {
            minor_sticky_potion_objective_value = 1.0;
        }
        Product::MinorPoisonPotion => {
            minor_poison_potion_objective_value = 1.0;
        }
        Product::EnergyPotion => {
            energy_potion_objective_value = 1.0;
        }
        Product::HealingPotion => {
            healing_potion_objective_value = 1.0;
        }
        Product::GigantifyPotion => {
            gigantify_potion_objective_value = 1.0;
        }
        Product::ResistancePotion => {
            resistance_potion_objective_value = 1.0;
        }
        Product::StickyPotion => {
            sticky_potion_objective_value = 1.0;
        }
        Product::PoisonPotion => {
            poison_potion_objective_value = 1.0;
        }
        Product::MajorEnergyPotion => {
            major_energy_potion_objective_value = 1.0;
        }
        Product::MajorHealingPotion => {
            major_healing_potion_objective_value = 1.0;
        }
        Product::MajorGigantifyPotion => {
            major_gigantify_potion_objective_value = 1.0;
        }
        Product::MajorResistancePotion => {
            major_resistance_potion_objective_value = 1.0;
        }
        Product::MajorStickyPotion => {
            major_sticky_potion_objective_value = 1.0;
        }
        Product::MajorPoisonPotion => {
            major_poison_potion_objective_value = 1.0;
        }
        Product::InvisibilityPotion => {
            invisibility_potion_objective_value = 1.0;
        }
        Product::ChickenOmelette => {
            chicken_omelette_objective_value = 1.0;
        }
        Product::GooseOmelette => {
            goose_omelette_objective_value = 1.0;
        }
        Product::PorkOmelette => {
            pork_omelette_objective_value = 1.0;
        }
        Product::BeanSalad => {
            bean_salad_objective_value = 1.0;
        }
        Product::TurnipSalad => {
            turnip_salad_objective_value = 1.0;
        }
        Product::PotatoSalad => {
            potato_salad_objective_value = 1.0;
        }
        Product::GoatSandwich => {
            goat_sandwich_objective_value = 1.0;
        }
        Product::MuttonSandwich => {
            mutton_sandwich_objective_value = 1.0;
        }
        Product::BeefSandwich => {
            beef_sandwich_objective_value = 1.0;
        }
        Product::CarrotSoup => {
            carrot_soup_objective_value = 1.0;
        }
        Product::WheatSoup => {
            wheat_soup_objective_value = 1.0;
        }
        Product::CabbageSoup => {
            cabbage_soup_objective_value = 1.0;
        }
        Product::GoatStew => {
            goat_stew_objective_value = 1.0;
        }
        Product::MuttonStew => {
            mutton_stew_objective_value = 1.0;
        }
        Product::BeefStew => {
            beef_stew_objective_value = 1.0;
        }
        Product::RoastChicken => {
            roast_chicken_objective_value = 1.0;
        }
        Product::RoastGoose => {
            roast_goose_objective_value = 1.0;
        }
        Product::RoastPork => {
            roast_pork_objective_value = 1.0;
        }
    }

    let minor_energy_potion =
        problem.add_var(minor_energy_potion_objective_value, (0.0, f64::INFINITY));
    let minor_healing_potion =
        problem.add_var(minor_healing_potion_objective_value, (0.0, f64::INFINITY));
    let minor_gigantify_potion =
        problem.add_var(minor_gigantify_potion_objective_value, (0.0, f64::INFINITY));
    let minor_resistance_potion = problem.add_var(
        minor_resistance_potion_objective_value,
        (0.0, f64::INFINITY),
    );
    let minor_sticky_potion =
        problem.add_var(minor_sticky_potion_objective_value, (0.0, f64::INFINITY));
    let minor_poison_potion =
        problem.add_var(minor_poison_potion_objective_value, (0.0, f64::INFINITY));
    let energy_potion = problem.add_var(energy_potion_objective_value, (0.0, f64::INFINITY));
    let healing_potion = problem.add_var(healing_potion_objective_value, (0.0, f64::INFINITY));
    let gigantify_potion = problem.add_var(gigantify_potion_objective_value, (0.0, f64::INFINITY));
    let resistance_potion =
        problem.add_var(resistance_potion_objective_value, (0.0, f64::INFINITY));
    let sticky_potion = problem.add_var(sticky_potion_objective_value, (0.0, f64::INFINITY));
    let poison_potion = problem.add_var(poison_potion_objective_value, (0.0, f64::INFINITY));
    let major_energy_potion =
        problem.add_var(major_energy_potion_objective_value, (0.0, f64::INFINITY));
    let major_healing_potion =
        problem.add_var(major_healing_potion_objective_value, (0.0, f64::INFINITY));
    let major_gigantify_potion =
        problem.add_var(major_gigantify_potion_objective_value, (0.0, f64::INFINITY));
    let major_resistance_potion = problem.add_var(
        major_resistance_potion_objective_value,
        (0.0, f64::INFINITY),
    );
    let major_sticky_potion =
        problem.add_var(major_sticky_potion_objective_value, (0.0, f64::INFINITY));
    let major_poison_potion =
        problem.add_var(major_poison_potion_objective_value, (0.0, f64::INFINITY));
    let invisibility_potion =
        problem.add_var(invisibility_potion_objective_value, (0.0, f64::INFINITY));
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
    problem.add_constraint(
        &[(plots_brecilien, 1.0)],
        ComparisonOp::Eq,
        context.brecilien_plots,
    );
    problem.add_constraint(
        &[(plots_bridgewatch, 1.0)],
        ComparisonOp::Eq,
        context.bridgewatch_plots,
    );
    problem.add_constraint(
        &[(plots_caerleon, 1.0)],
        ComparisonOp::Eq,
        context.caerleon_plots,
    );
    problem.add_constraint(
        &[(plots_fort_sterling, 1.0)],
        ComparisonOp::Eq,
        context.fort_sterling_plots,
    );
    problem.add_constraint(
        &[(plots_lymhurst, 1.0)],
        ComparisonOp::Eq,
        context.lymhurst_plots,
    );
    problem.add_constraint(
        &[(plots_martlock, 1.0)],
        ComparisonOp::Eq,
        context.martlock_plots,
    );
    problem.add_constraint(
        &[(plots_thetford, 1.0)],
        ComparisonOp::Eq,
        context.thetford_plots,
    );

    //// plot count constraints
    problem.add_constraint(
        &[
            (herb_gardens_brecilien, 1.0),
            (farms_brecilien, 1.0),
            (pastures_brecilien, 1.0),
            (plots_brecilien, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (herb_gardens_bridgewatch, 1.0),
            (farms_bridgewatch, 1.0),
            (pastures_bridgewatch, 1.0),
            (plots_bridgewatch, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (herb_gardens_caerleon, 1.0),
            (farms_caerleon, 1.0),
            (pastures_caerleon, 1.0),
            (plots_caerleon, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (herb_gardens_fort_sterling, 1.0),
            (farms_fort_sterling, 1.0),
            (pastures_fort_sterling, 1.0),
            (plots_fort_sterling, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (herb_gardens_lymhurst, 1.0),
            (farms_lymhurst, 1.0),
            (pastures_lymhurst, 1.0),
            (plots_lymhurst, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (herb_gardens_martlock, 1.0),
            (farms_martlock, 1.0),
            (pastures_martlock, 1.0),
            (plots_martlock, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (herb_gardens_thetford, 1.0),
            (farms_thetford, 1.0),
            (pastures_thetford, 1.0),
            (plots_thetford, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );

    //// plots to tiles constraints
    problem.add_constraint(
        &[
            (herb_gardens_brecilien, TILES_PER_PLOT),
            (herb_garden_tiles_brecilien, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (farms_brecilien, TILES_PER_PLOT),
            (farm_tiles_brecilien, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (pastures_brecilien, TILES_PER_PLOT),
            (pasture_tiles_brecilien, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (herb_gardens_bridgewatch, TILES_PER_PLOT),
            (herb_garden_tiles_bridgewatch, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (farms_bridgewatch, TILES_PER_PLOT),
            (farm_tiles_bridgewatch, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (pastures_bridgewatch, TILES_PER_PLOT),
            (pasture_tiles_bridgewatch, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (herb_gardens_caerleon, TILES_PER_PLOT),
            (herb_garden_tiles_caerleon, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (farms_caerleon, TILES_PER_PLOT),
            (farm_tiles_caerleon, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (pastures_caerleon, TILES_PER_PLOT),
            (pasture_tiles_caerleon, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (herb_gardens_fort_sterling, TILES_PER_PLOT),
            (herb_garden_tiles_fort_sterling, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (farms_fort_sterling, TILES_PER_PLOT),
            (farm_tiles_fort_sterling, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (pastures_fort_sterling, TILES_PER_PLOT),
            (pasture_tiles_fort_sterling, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (herb_gardens_lymhurst, TILES_PER_PLOT),
            (herb_garden_tiles_lymhurst, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (farms_lymhurst, TILES_PER_PLOT),
            (farm_tiles_lymhurst, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (pastures_lymhurst, TILES_PER_PLOT),
            (pasture_tiles_lymhurst, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (herb_gardens_martlock, TILES_PER_PLOT),
            (herb_garden_tiles_martlock, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (farms_martlock, TILES_PER_PLOT),
            (farm_tiles_martlock, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (pastures_martlock, TILES_PER_PLOT),
            (pasture_tiles_martlock, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (herb_gardens_thetford, TILES_PER_PLOT),
            (herb_garden_tiles_thetford, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (farms_thetford, TILES_PER_PLOT),
            (farm_tiles_thetford, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (pastures_thetford, TILES_PER_PLOT),
            (pasture_tiles_thetford, -1.0),
        ],
        ComparisonOp::Eq,
        0.0,
    );

    //// tiles to filled tiles constraints
    problem.add_constraint(
        &[
            (yarrow_tiles_brecilien, 1.0),
            (muellin_tiles_brecilien, 1.0),
            (foxglove_tiles_brecilien, 1.0),
            (teasel_tiles_brecilien, 1.0),
            (burdock_tiles_brecilien, 1.0),
            (comfrey_tiles_brecilien, 1.0),
            (agaric_tiles_brecilien, 1.0),
            (herb_garden_tiles_brecilien, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (cow_tiles_brecilien, 1.0),
            (pig_tiles_brecilien, 1.0),
            (sheep_tiles_brecilien, 1.0),
            (goose_tiles_brecilien, 1.0),
            (goat_tiles_brecilien, 1.0),
            (chicken_tiles_brecilien, 1.0),
            (calf_tiles_brecilien, 1.0),
            (piglet_tiles_brecilien, 1.0),
            (lamb_tiles_brecilien, 1.0),
            (gosling_tiles_brecilien, 1.0),
            (kid_tiles_brecilien, 1.0),
            (baby_chicken_tiles_brecilien, 1.0),
            (pasture_tiles_brecilien, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (pumpkin_tiles_brecilien, 1.0),
            (corn_tiles_brecilien, 1.0),
            (potato_tiles_brecilien, 1.0),
            (cabbage_tiles_brecilien, 1.0),
            (turnip_tiles_brecilien, 1.0),
            (wheat_tiles_brecilien, 1.0),
            (bean_tiles_brecilien, 1.0),
            (carrot_tiles_brecilien, 1.0),
            (farm_tiles_brecilien, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (yarrow_tiles_bridgewatch, 1.0),
            (muellin_tiles_bridgewatch, 1.0),
            (foxglove_tiles_bridgewatch, 1.0),
            (teasel_tiles_bridgewatch, 1.0),
            (burdock_tiles_bridgewatch, 1.0),
            (comfrey_tiles_bridgewatch, 1.0),
            (agaric_tiles_bridgewatch, 1.0),
            (herb_garden_tiles_bridgewatch, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (cow_tiles_bridgewatch, 1.0),
            (pig_tiles_bridgewatch, 1.0),
            (sheep_tiles_bridgewatch, 1.0),
            (goose_tiles_bridgewatch, 1.0),
            (goat_tiles_bridgewatch, 1.0),
            (chicken_tiles_bridgewatch, 1.0),
            (calf_tiles_bridgewatch, 1.0),
            (piglet_tiles_bridgewatch, 1.0),
            (lamb_tiles_bridgewatch, 1.0),
            (gosling_tiles_bridgewatch, 1.0),
            (kid_tiles_bridgewatch, 1.0),
            (baby_chicken_tiles_bridgewatch, 1.0),
            (pasture_tiles_bridgewatch, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (pumpkin_tiles_bridgewatch, 1.0),
            (corn_tiles_bridgewatch, 1.0),
            (potato_tiles_bridgewatch, 1.0),
            (cabbage_tiles_bridgewatch, 1.0),
            (turnip_tiles_bridgewatch, 1.0),
            (wheat_tiles_bridgewatch, 1.0),
            (bean_tiles_bridgewatch, 1.0),
            (carrot_tiles_bridgewatch, 1.0),
            (farm_tiles_bridgewatch, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (yarrow_tiles_caerleon, 1.0),
            (muellin_tiles_caerleon, 1.0),
            (foxglove_tiles_caerleon, 1.0),
            (teasel_tiles_caerleon, 1.0),
            (burdock_tiles_caerleon, 1.0),
            (comfrey_tiles_caerleon, 1.0),
            (agaric_tiles_caerleon, 1.0),
            (herb_garden_tiles_caerleon, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (cow_tiles_caerleon, 1.0),
            (pig_tiles_caerleon, 1.0),
            (sheep_tiles_caerleon, 1.0),
            (goose_tiles_caerleon, 1.0),
            (goat_tiles_caerleon, 1.0),
            (chicken_tiles_caerleon, 1.0),
            (calf_tiles_caerleon, 1.0),
            (piglet_tiles_caerleon, 1.0),
            (lamb_tiles_caerleon, 1.0),
            (gosling_tiles_caerleon, 1.0),
            (kid_tiles_caerleon, 1.0),
            (baby_chicken_tiles_caerleon, 1.0),
            (pasture_tiles_caerleon, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (pumpkin_tiles_caerleon, 1.0),
            (corn_tiles_caerleon, 1.0),
            (potato_tiles_caerleon, 1.0),
            (cabbage_tiles_caerleon, 1.0),
            (turnip_tiles_caerleon, 1.0),
            (wheat_tiles_caerleon, 1.0),
            (bean_tiles_caerleon, 1.0),
            (carrot_tiles_caerleon, 1.0),
            (farm_tiles_caerleon, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (yarrow_tiles_fort_sterling, 1.0),
            (muellin_tiles_fort_sterling, 1.0),
            (foxglove_tiles_fort_sterling, 1.0),
            (teasel_tiles_fort_sterling, 1.0),
            (burdock_tiles_fort_sterling, 1.0),
            (comfrey_tiles_fort_sterling, 1.0),
            (agaric_tiles_fort_sterling, 1.0),
            (herb_garden_tiles_fort_sterling, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (cow_tiles_fort_sterling, 1.0),
            (pig_tiles_fort_sterling, 1.0),
            (sheep_tiles_fort_sterling, 1.0),
            (goose_tiles_fort_sterling, 1.0),
            (goat_tiles_fort_sterling, 1.0),
            (chicken_tiles_fort_sterling, 1.0),
            (calf_tiles_fort_sterling, 1.0),
            (piglet_tiles_fort_sterling, 1.0),
            (lamb_tiles_fort_sterling, 1.0),
            (gosling_tiles_fort_sterling, 1.0),
            (kid_tiles_fort_sterling, 1.0),
            (baby_chicken_tiles_fort_sterling, 1.0),
            (pasture_tiles_fort_sterling, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (pumpkin_tiles_fort_sterling, 1.0),
            (corn_tiles_fort_sterling, 1.0),
            (potato_tiles_fort_sterling, 1.0),
            (cabbage_tiles_fort_sterling, 1.0),
            (turnip_tiles_fort_sterling, 1.0),
            (wheat_tiles_fort_sterling, 1.0),
            (bean_tiles_fort_sterling, 1.0),
            (carrot_tiles_fort_sterling, 1.0),
            (farm_tiles_fort_sterling, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (yarrow_tiles_lymhurst, 1.0),
            (muellin_tiles_lymhurst, 1.0),
            (foxglove_tiles_lymhurst, 1.0),
            (teasel_tiles_lymhurst, 1.0),
            (burdock_tiles_lymhurst, 1.0),
            (comfrey_tiles_lymhurst, 1.0),
            (agaric_tiles_lymhurst, 1.0),
            (herb_garden_tiles_lymhurst, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (cow_tiles_lymhurst, 1.0),
            (pig_tiles_lymhurst, 1.0),
            (sheep_tiles_lymhurst, 1.0),
            (goose_tiles_lymhurst, 1.0),
            (goat_tiles_lymhurst, 1.0),
            (chicken_tiles_lymhurst, 1.0),
            (calf_tiles_lymhurst, 1.0),
            (piglet_tiles_lymhurst, 1.0),
            (lamb_tiles_lymhurst, 1.0),
            (gosling_tiles_lymhurst, 1.0),
            (kid_tiles_lymhurst, 1.0),
            (baby_chicken_tiles_lymhurst, 1.0),
            (pasture_tiles_lymhurst, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (pumpkin_tiles_lymhurst, 1.0),
            (corn_tiles_lymhurst, 1.0),
            (potato_tiles_lymhurst, 1.0),
            (cabbage_tiles_lymhurst, 1.0),
            (turnip_tiles_lymhurst, 1.0),
            (wheat_tiles_lymhurst, 1.0),
            (bean_tiles_lymhurst, 1.0),
            (carrot_tiles_lymhurst, 1.0),
            (farm_tiles_lymhurst, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (yarrow_tiles_martlock, 1.0),
            (muellin_tiles_martlock, 1.0),
            (foxglove_tiles_martlock, 1.0),
            (teasel_tiles_martlock, 1.0),
            (burdock_tiles_martlock, 1.0),
            (comfrey_tiles_martlock, 1.0),
            (agaric_tiles_martlock, 1.0),
            (herb_garden_tiles_martlock, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (cow_tiles_martlock, 1.0),
            (pig_tiles_martlock, 1.0),
            (sheep_tiles_martlock, 1.0),
            (goose_tiles_martlock, 1.0),
            (goat_tiles_martlock, 1.0),
            (chicken_tiles_martlock, 1.0),
            (calf_tiles_martlock, 1.0),
            (piglet_tiles_martlock, 1.0),
            (lamb_tiles_martlock, 1.0),
            (gosling_tiles_martlock, 1.0),
            (kid_tiles_martlock, 1.0),
            (baby_chicken_tiles_martlock, 1.0),
            (pasture_tiles_martlock, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (pumpkin_tiles_martlock, 1.0),
            (corn_tiles_martlock, 1.0),
            (potato_tiles_martlock, 1.0),
            (cabbage_tiles_martlock, 1.0),
            (turnip_tiles_martlock, 1.0),
            (wheat_tiles_martlock, 1.0),
            (bean_tiles_martlock, 1.0),
            (carrot_tiles_martlock, 1.0),
            (farm_tiles_martlock, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (yarrow_tiles_thetford, 1.0),
            (muellin_tiles_thetford, 1.0),
            (foxglove_tiles_thetford, 1.0),
            (teasel_tiles_thetford, 1.0),
            (burdock_tiles_thetford, 1.0),
            (comfrey_tiles_thetford, 1.0),
            (agaric_tiles_thetford, 1.0),
            (herb_garden_tiles_thetford, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (cow_tiles_thetford, 1.0),
            (pig_tiles_thetford, 1.0),
            (sheep_tiles_thetford, 1.0),
            (goose_tiles_thetford, 1.0),
            (goat_tiles_thetford, 1.0),
            (chicken_tiles_thetford, 1.0),
            (calf_tiles_thetford, 1.0),
            (piglet_tiles_thetford, 1.0),
            (lamb_tiles_thetford, 1.0),
            (gosling_tiles_thetford, 1.0),
            (kid_tiles_thetford, 1.0),
            (baby_chicken_tiles_thetford, 1.0),
            (pasture_tiles_thetford, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );
    problem.add_constraint(
        &[
            (pumpkin_tiles_thetford, 1.0),
            (corn_tiles_thetford, 1.0),
            (potato_tiles_thetford, 1.0),
            (cabbage_tiles_thetford, 1.0),
            (turnip_tiles_thetford, 1.0),
            (wheat_tiles_thetford, 1.0),
            (bean_tiles_thetford, 1.0),
            (carrot_tiles_thetford, 1.0),
            (farm_tiles_thetford, -1.0),
        ],
        ComparisonOp::Le,
        0.0,
    );

    //// filled tiles to products constraints
    ////// herb constraints
    problem.add_constraint(
        &[
            (agaric_herbs, 1.0),
            (
                agaric_tiles_brecilien,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                agaric_tiles_bridgewatch,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                agaric_tiles_caerleon,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                agaric_tiles_fort_sterling,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                agaric_tiles_lymhurst,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                agaric_tiles_martlock,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                agaric_tiles_thetford,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (comfrey_herbs, 1.0),
            (
                comfrey_tiles_brecilien,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                comfrey_tiles_bridgewatch,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                comfrey_tiles_caerleon,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                comfrey_tiles_fort_sterling,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                comfrey_tiles_lymhurst,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                comfrey_tiles_martlock,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                comfrey_tiles_thetford,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (burdock_herbs, 1.0),
            (
                burdock_tiles_brecilien,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                burdock_tiles_bridgewatch,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                burdock_tiles_caerleon,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                burdock_tiles_fort_sterling,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                burdock_tiles_lymhurst,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                burdock_tiles_martlock,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                burdock_tiles_thetford,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (teasel_herbs, 1.0),
            (
                teasel_tiles_brecilien,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                teasel_tiles_bridgewatch,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                teasel_tiles_caerleon,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                teasel_tiles_fort_sterling,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                teasel_tiles_lymhurst,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                teasel_tiles_martlock,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                teasel_tiles_thetford,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (foxglove_herbs, 1.0),
            (
                foxglove_tiles_brecilien,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                foxglove_tiles_bridgewatch,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                foxglove_tiles_caerleon,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                foxglove_tiles_fort_sterling,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                foxglove_tiles_lymhurst,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                foxglove_tiles_martlock,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                foxglove_tiles_thetford,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (muellin_herbs, 1.0),
            (
                muellin_tiles_brecilien,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                muellin_tiles_bridgewatch,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                muellin_tiles_caerleon,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                muellin_tiles_fort_sterling,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                muellin_tiles_lymhurst,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                muellin_tiles_martlock,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                muellin_tiles_thetford,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (yarrow_herbs, 1.0),
            (
                yarrow_tiles_brecilien,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                yarrow_tiles_bridgewatch,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                yarrow_tiles_caerleon,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                yarrow_tiles_fort_sterling,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                yarrow_tiles_lymhurst,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                yarrow_tiles_martlock,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                yarrow_tiles_thetford,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
        ],
        ComparisonOp::Eq,
        0.0,
    );

    ////// crop constraints
    problem.add_constraint(
        &[
            (carrot_crops, 1.0),
            (
                carrot_tiles_brecilien,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                carrot_tiles_bridgewatch,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                carrot_tiles_caerleon,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                carrot_tiles_fort_sterling,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                carrot_tiles_lymhurst,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                carrot_tiles_martlock,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                carrot_tiles_thetford,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (bean_crops, 1.0),
            (
                bean_tiles_brecilien,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                bean_tiles_bridgewatch,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                bean_tiles_caerleon,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                bean_tiles_fort_sterling,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                bean_tiles_lymhurst,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                bean_tiles_martlock,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                bean_tiles_thetford,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (wheat_crops, 1.0),
            (
                wheat_tiles_brecilien,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                baby_chicken_tiles_brecilien,
                ANIMAL_FAVORITE_FOOD_CONSUMPTION,
            ),
            (chicken_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                wheat_tiles_bridgewatch,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                baby_chicken_tiles_bridgewatch,
                ANIMAL_FAVORITE_FOOD_CONSUMPTION,
            ),
            (chicken_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                wheat_tiles_caerleon,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                baby_chicken_tiles_caerleon,
                ANIMAL_FAVORITE_FOOD_CONSUMPTION,
            ),
            (chicken_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                wheat_tiles_fort_sterling,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                baby_chicken_tiles_fort_sterling,
                ANIMAL_FAVORITE_FOOD_CONSUMPTION,
            ),
            (
                chicken_tiles_fort_sterling,
                ANIMAL_FAVORITE_FOOD_CONSUMPTION,
            ),
            (
                wheat_tiles_lymhurst,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                baby_chicken_tiles_lymhurst,
                ANIMAL_FAVORITE_FOOD_CONSUMPTION,
            ),
            (chicken_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                wheat_tiles_martlock,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                baby_chicken_tiles_martlock,
                ANIMAL_FAVORITE_FOOD_CONSUMPTION,
            ),
            (chicken_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                wheat_tiles_thetford,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                baby_chicken_tiles_thetford,
                ANIMAL_FAVORITE_FOOD_CONSUMPTION,
            ),
            (chicken_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (turnip_crops, 1.0),
            (
                turnip_tiles_brecilien,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (kid_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (goat_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                turnip_tiles_bridgewatch,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (kid_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (goat_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                turnip_tiles_caerleon,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (kid_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (goat_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                turnip_tiles_fort_sterling,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (kid_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (goat_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                turnip_tiles_lymhurst,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (kid_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (goat_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                turnip_tiles_martlock,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (kid_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (goat_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                turnip_tiles_thetford,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (kid_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (goat_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (cabbage_crops, 1.0),
            (
                cabbage_tiles_brecilien,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (gosling_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (goose_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                cabbage_tiles_bridgewatch,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (gosling_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (goose_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                cabbage_tiles_caerleon,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (gosling_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (goose_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                cabbage_tiles_fort_sterling,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (
                gosling_tiles_fort_sterling,
                ANIMAL_FAVORITE_FOOD_CONSUMPTION,
            ),
            (goose_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                cabbage_tiles_lymhurst,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (gosling_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (goose_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                cabbage_tiles_martlock,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (gosling_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (goose_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                cabbage_tiles_thetford,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (gosling_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (goose_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (potato_crops, 1.0),
            (
                potato_tiles_brecilien,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (lamb_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (sheep_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                potato_tiles_bridgewatch,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (lamb_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (sheep_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                potato_tiles_caerleon,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (lamb_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (sheep_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                potato_tiles_fort_sterling,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (lamb_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (sheep_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                potato_tiles_lymhurst,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (lamb_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (sheep_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                potato_tiles_martlock,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (lamb_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (sheep_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                potato_tiles_thetford,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (lamb_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (sheep_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (corn_crops, 1.0),
            (
                corn_tiles_brecilien,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (piglet_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (pig_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                corn_tiles_bridgewatch,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (piglet_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (pig_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                corn_tiles_caerleon,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (piglet_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (pig_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                corn_tiles_fort_sterling,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (piglet_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (pig_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                corn_tiles_lymhurst,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (piglet_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (pig_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                corn_tiles_martlock,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (piglet_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (pig_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                corn_tiles_thetford,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (piglet_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (pig_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (pumpkin_crops, 1.0),
            (
                pumpkin_tiles_brecilien,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (calf_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (cow_tiles_brecilien, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                pumpkin_tiles_bridgewatch,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (calf_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (cow_tiles_bridgewatch, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                pumpkin_tiles_caerleon,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (calf_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (cow_tiles_caerleon, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                pumpkin_tiles_fort_sterling,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (calf_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (cow_tiles_fort_sterling, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                pumpkin_tiles_lymhurst,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (calf_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (cow_tiles_lymhurst, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                pumpkin_tiles_martlock,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (calf_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (cow_tiles_martlock, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (
                pumpkin_tiles_thetford,
                -1.0 * CROP_AND_HERB_FACTOR * context.premium_factor,
            ),
            (calf_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
            (cow_tiles_thetford, ANIMAL_FAVORITE_FOOD_CONSUMPTION),
        ],
        ComparisonOp::Eq,
        0.0,
    );

    ////// animal product constraints
    problem.add_constraint(
        &[
            (hen_eggs, 1.0),
            (
                chicken_tiles_brecilien,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                chicken_tiles_bridgewatch,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                chicken_tiles_caerleon,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                chicken_tiles_fort_sterling,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                chicken_tiles_lymhurst,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                chicken_tiles_martlock,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                chicken_tiles_thetford,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (goats_milk, 1.0),
            (
                goat_tiles_brecilien,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                goat_tiles_bridgewatch,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                goat_tiles_caerleon,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                goat_tiles_fort_sterling,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                goat_tiles_lymhurst,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                goat_tiles_martlock,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                goat_tiles_thetford,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (goose_eggs, 1.0),
            (
                goose_tiles_brecilien,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                goose_tiles_bridgewatch,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                goose_tiles_caerleon,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                goose_tiles_fort_sterling,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                goose_tiles_lymhurst,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                goose_tiles_martlock,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                goose_tiles_thetford,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (sheeps_milk, 1.0),
            (
                sheep_tiles_brecilien,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                sheep_tiles_bridgewatch,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                sheep_tiles_caerleon,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                sheep_tiles_fort_sterling,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                sheep_tiles_lymhurst,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                sheep_tiles_martlock,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                sheep_tiles_thetford,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (cows_milk, 1.0),
            (
                cow_tiles_brecilien,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                cow_tiles_bridgewatch,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                cow_tiles_caerleon,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                cow_tiles_fort_sterling,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                cow_tiles_lymhurst,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
            (
                cow_tiles_martlock,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor * BONUS_FACTOR,
            ),
            (
                cow_tiles_thetford,
                -1.0 * ANIMAL_PRODUCT_FACTOR * context.premium_factor,
            ),
        ],
        ComparisonOp::Eq,
        0.0,
    );

    ////// meat constraints
    problem.add_constraint(
        &[
            (raw_chicken, 1.0),
            (baby_chicken_tiles_brecilien, -1.0 * MEAT_FACTOR),
            (baby_chicken_tiles_bridgewatch, -1.0 * MEAT_FACTOR),
            (baby_chicken_tiles_caerleon, -1.0 * MEAT_FACTOR),
            (
                baby_chicken_tiles_fort_sterling,
                -1.0 * MEAT_FACTOR * BONUS_FACTOR,
            ),
            (baby_chicken_tiles_lymhurst, -1.0 * MEAT_FACTOR),
            (baby_chicken_tiles_martlock, -1.0 * MEAT_FACTOR),
            (baby_chicken_tiles_thetford, -1.0 * MEAT_FACTOR),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (raw_goat, 1.0),
            (kid_tiles_brecilien, -1.0 * MEAT_FACTOR),
            (kid_tiles_bridgewatch, -1.0 * MEAT_FACTOR * BONUS_FACTOR),
            (kid_tiles_caerleon, -1.0 * MEAT_FACTOR),
            (kid_tiles_fort_sterling, -1.0 * MEAT_FACTOR),
            (kid_tiles_lymhurst, -1.0 * MEAT_FACTOR),
            (kid_tiles_martlock, -1.0 * MEAT_FACTOR),
            (kid_tiles_thetford, -1.0 * MEAT_FACTOR),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (raw_goose, 1.0),
            (gosling_tiles_brecilien, -1.0 * MEAT_FACTOR),
            (gosling_tiles_bridgewatch, -1.0 * MEAT_FACTOR),
            (gosling_tiles_caerleon, -1.0 * MEAT_FACTOR),
            (gosling_tiles_fort_sterling, -1.0 * MEAT_FACTOR),
            (gosling_tiles_lymhurst, -1.0 * MEAT_FACTOR * BONUS_FACTOR),
            (gosling_tiles_martlock, -1.0 * MEAT_FACTOR),
            (gosling_tiles_thetford, -1.0 * MEAT_FACTOR),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (raw_mutton, 1.0),
            (lamb_tiles_brecilien, -1.0 * MEAT_FACTOR),
            (lamb_tiles_bridgewatch, -1.0 * MEAT_FACTOR),
            (lamb_tiles_caerleon, -1.0 * MEAT_FACTOR),
            (lamb_tiles_fort_sterling, -1.0 * MEAT_FACTOR * BONUS_FACTOR),
            (lamb_tiles_lymhurst, -1.0 * MEAT_FACTOR),
            (lamb_tiles_martlock, -1.0 * MEAT_FACTOR),
            (lamb_tiles_thetford, -1.0 * MEAT_FACTOR),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (raw_pork, 1.0),
            (piglet_tiles_brecilien, -1.0 * MEAT_FACTOR),
            (piglet_tiles_bridgewatch, -1.0 * MEAT_FACTOR),
            (piglet_tiles_caerleon, -1.0 * MEAT_FACTOR),
            (piglet_tiles_fort_sterling, -1.0 * MEAT_FACTOR),
            (piglet_tiles_lymhurst, -1.0 * MEAT_FACTOR),
            (piglet_tiles_martlock, -1.0 * MEAT_FACTOR),
            (piglet_tiles_thetford, -1.0 * MEAT_FACTOR * BONUS_FACTOR),
        ],
        ComparisonOp::Eq,
        0.0,
    );
    problem.add_constraint(
        &[
            (raw_beef, 1.0),
            (calf_tiles_brecilien, -1.0 * MEAT_FACTOR),
            (calf_tiles_bridgewatch, -1.0 * MEAT_FACTOR),
            (calf_tiles_caerleon, -1.0 * MEAT_FACTOR),
            (calf_tiles_fort_sterling, -1.0 * MEAT_FACTOR),
            (calf_tiles_lymhurst, -1.0 * MEAT_FACTOR),
            (calf_tiles_martlock, -1.0 * MEAT_FACTOR * BONUS_FACTOR),
            (calf_tiles_thetford, -1.0 * MEAT_FACTOR),
        ],
        ComparisonOp::Eq,
        0.0,
    );

    ////// recipe constraints
    match context.target {
        Product::MinorEnergyPotion => {
            problem.add_constraint(
                &[
                    (minor_energy_potion, 1.0 * L04_INGREDIENT),
                    (agaric_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::MinorHealingPotion => {
            problem.add_constraint(
                &[
                    (minor_healing_potion, 1.0 * L04_INGREDIENT),
                    (agaric_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::MinorGigantifyPotion => {
            problem.add_constraint(
                &[
                    (minor_gigantify_potion, 1.0 * L04_INGREDIENT),
                    (comfrey_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::MinorResistancePotion => {
            problem.add_constraint(
                &[
                    (minor_resistance_potion, 1.0 * L04_INGREDIENT),
                    (comfrey_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::MinorStickyPotion => {
            problem.add_constraint(
                &[
                    (minor_sticky_potion, 1.0 * L04_INGREDIENT),
                    (comfrey_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::MinorPoisonPotion => {
            problem.add_constraint(
                &[
                    (minor_poison_potion, 1.0 * L04_INGREDIENT),
                    (burdock_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (burdock_herbs, 1.0 * L02_INGREDIENT),
                    (comfrey_herbs, -1.0 * L04_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::EnergyPotion => {
            problem.add_constraint(
                &[
                    (energy_potion, 1.0 * L08_INGREDIENT),
                    (burdock_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (burdock_herbs, 1.0 * L03_INGREDIENT),
                    (goats_milk, -1.0 * L08_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::HealingPotion => {
            problem.add_constraint(
                &[
                    (healing_potion, 1.0 * L08_INGREDIENT),
                    (burdock_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (burdock_herbs, 1.0 * L03_INGREDIENT),
                    (hen_eggs, -1.0 * L08_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::GigantifyPotion => {
            problem.add_constraint(
                &[
                    (gigantify_potion, 1.0 * L08_INGREDIENT),
                    (teasel_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (teasel_herbs, 1.0 * L05_INGREDIENT),
                    (burdock_herbs, -1.0 * L08_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (burdock_herbs, 1.0 * L03_INGREDIENT),
                    (goose_eggs, -1.0 * L05_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::ResistancePotion => {
            problem.add_constraint(
                &[
                    (resistance_potion, 1.0 * L08_INGREDIENT),
                    (teasel_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (teasel_herbs, 1.0 * L05_INGREDIENT),
                    (burdock_herbs, -1.0 * L08_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (burdock_herbs, 1.0 * L03_INGREDIENT),
                    (goats_milk, -1.0 * L05_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::StickyPotion => {
            problem.add_constraint(
                &[
                    (sticky_potion, 1.0 * L08_INGREDIENT),
                    (teasel_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (teasel_herbs, 1.0 * L05_INGREDIENT),
                    (burdock_herbs, -1.0 * L08_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (burdock_herbs, 1.0 * L03_INGREDIENT),
                    (goose_eggs, -1.0 * L05_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::PoisonPotion => {
            problem.add_constraint(
                &[
                    (poison_potion, 1.0 * L08_INGREDIENT),
                    (foxglove_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (foxglove_herbs, 1.0 * L05_INGREDIENT),
                    (teasel_herbs, -1.0 * L08_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (teasel_herbs, 1.0 * L05_INGREDIENT),
                    (comfrey_herbs, -1.0 * L05_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (comfrey_herbs, 1.0 * L03_INGREDIENT),
                    (sheeps_milk, -1.0 * L05_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::MajorEnergyPotion => {
            problem.add_constraint(
                &[
                    (major_energy_potion, 1.0 * L11_INGREDIENT),
                    (foxglove_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (foxglove_herbs, 1.0 * L07_INGREDIENT),
                    (sheeps_milk, -1.0 * L11_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (sheeps_milk, 1.0 * L07_INGREDIENT),
                    (potato_crops, -1.0 * L07_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::MajorHealingPotion => {
            problem.add_constraint(
                &[
                    (major_healing_potion, 1.0 * L11_INGREDIENT),
                    (foxglove_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (foxglove_herbs, 1.0 * L07_INGREDIENT),
                    (goose_eggs, -1.0 * L11_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (goose_eggs, 1.0 * L07_INGREDIENT),
                    (potato_crops, -1.0 * L07_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::MajorGigantifyPotion => {
            problem.add_constraint(
                &[
                    (major_gigantify_potion, 1.0 * L11_INGREDIENT),
                    (muellin_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (muellin_herbs, 1.0 * L09_INGREDIENT),
                    (foxglove_herbs, -1.0 * L11_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (foxglove_herbs, 1.0 * L07_INGREDIENT),
                    (goose_eggs, -1.0 * L09_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (goose_eggs, 1.0 * L07_INGREDIENT),
                    (corn_crops, -1.0 * L07_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::MajorResistancePotion => {
            problem.add_constraint(
                &[
                    (major_resistance_potion, 1.0 * L11_INGREDIENT),
                    (muellin_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (muellin_herbs, 1.0 * L09_INGREDIENT),
                    (foxglove_herbs, -1.0 * L11_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (foxglove_herbs, 1.0 * L09_INGREDIENT),
                    (burdock_herbs, -1.0 * L09_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (burdock_herbs, 1.0 * L07_INGREDIENT),
                    (sheeps_milk, -1.0 * L09_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (sheeps_milk, 1.0 * L07_INGREDIENT),
                    (corn_crops, -1.0 * L07_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::MajorStickyPotion => {
            problem.add_constraint(
                &[
                    (major_sticky_potion, 1.0 * L11_INGREDIENT),
                    (muellin_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (muellin_herbs, 1.0 * L09_INGREDIENT),
                    (foxglove_herbs, -1.0 * L11_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (foxglove_herbs, 1.0 * L09_INGREDIENT),
                    (burdock_herbs, -1.0 * L09_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (burdock_herbs, 1.0 * L07_INGREDIENT),
                    (goose_eggs, -1.0 * L09_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (goose_eggs, 1.0 * L07_INGREDIENT),
                    (corn_crops, -1.0 * L07_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::MajorPoisonPotion => {
            problem.add_constraint(
                &[
                    (major_poison_potion, 1.0 * L11_INGREDIENT),
                    (yarrow_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (yarrow_herbs, 1.0 * L09_INGREDIENT),
                    (muellin_herbs, -1.0 * L11_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (muellin_herbs, 1.0 * L09_INGREDIENT),
                    (teasel_herbs, -1.0 * L09_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (teasel_herbs, 1.0 * L07_INGREDIENT),
                    (cows_milk, -1.0 * L09_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (cows_milk, 1.0 * L07_INGREDIENT),
                    (pumpkin_crops, -1.0 * L07_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::InvisibilityPotion => {
            problem.add_constraint(
                &[
                    (invisibility_potion, 1.0 * L11_INGREDIENT),
                    (yarrow_herbs, -1.0 * POTIONS_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (yarrow_herbs, 1.0 * L09_INGREDIENT),
                    (muellin_herbs, -1.0 * L11_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (muellin_herbs, 1.0 * L09_INGREDIENT),
                    (teasel_herbs, -1.0 * L09_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (teasel_herbs, 1.0 * L07_INGREDIENT),
                    (cows_milk, -1.0 * L09_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (cows_milk, 1.0 * L07_INGREDIENT),
                    (pumpkin_crops, -1.0 * L07_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::ChickenOmelette => {
            problem.add_constraint(
                &[
                    (chicken_omelette, 1.0 * L02_INGREDIENT),
                    (wheat_crops, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (wheat_crops, 1.0 * L04_INGREDIENT),
                    (raw_chicken, -1.0 * L02_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (raw_chicken, 1.0 * L01_INGREDIENT),
                    (hen_eggs, -1.0 * L04_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::GooseOmelette => {
            problem.add_constraint(
                &[
                    (goose_omelette, 1.0 * L05_INGREDIENT),
                    (cabbage_crops, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (cabbage_crops, 1.0 * L08_INGREDIENT),
                    (raw_goose, -1.0 * L05_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (raw_goose, 1.0 * L03_INGREDIENT),
                    (goose_eggs, -1.0 * L08_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::PorkOmelette => {
            problem.add_constraint(
                &[
                    (pork_omelette, 1.0 * L09_INGREDIENT),
                    (corn_crops, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (corn_crops, 1.0 * L11_INGREDIENT),
                    (raw_pork, -1.0 * L09_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (raw_pork, 1.0 * L07_INGREDIENT),
                    (goose_eggs, -1.0 * L11_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::BeanSalad => {
            problem.add_constraint(
                &[
                    (bean_salad, 1.0 * L04_INGREDIENT),
                    (bean_crops, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (bean_crops, 1.0 * L04_INGREDIENT),
                    (carrot_crops, -1.0 * L04_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::TurnipSalad => {
            problem.add_constraint(
                &[
                    (turnip_salad, 1.0 * L08_INGREDIENT),
                    (turnip_crops, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (turnip_crops, 1.0 * L08_INGREDIENT),
                    (wheat_crops, -1.0 * L08_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::PotatoSalad => {
            problem.add_constraint(
                &[
                    (potato_salad, 1.0 * L11_INGREDIENT),
                    (potato_crops, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (potato_crops, 1.0 * L11_INGREDIENT),
                    (cabbage_crops, -1.0 * L11_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::GoatSandwich => {
            problem.add_constraint(
                &[
                    (goat_sandwich, 1.0 * L02_INGREDIENT),
                    (wheat_crops, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (wheat_crops, 1.0 * L04_INGREDIENT),
                    (raw_goat, -1.0 * L02_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (raw_goat, 1.0 * L01_INGREDIENT),
                    (goats_milk, -1.0 * L04_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::MuttonSandwich => {
            problem.add_constraint(
                &[
                    (mutton_sandwich, 1.0 * L05_INGREDIENT),
                    (wheat_crops, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (wheat_crops, 1.0 * L08_INGREDIENT),
                    (raw_mutton, -1.0 * L05_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (raw_mutton, 1.0 * L03_INGREDIENT),
                    (sheeps_milk, -1.0 * L08_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::BeefSandwich => {
            problem.add_constraint(
                &[
                    (beef_sandwich, 1.0 * L09_INGREDIENT),
                    (wheat_crops, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (wheat_crops, 1.0 * L11_INGREDIENT),
                    (raw_beef, -1.0 * L09_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (raw_beef, 1.0 * L07_INGREDIENT),
                    (cows_milk, -1.0 * L11_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::CarrotSoup => {
            problem.add_constraint(
                &[
                    (carrot_soup, 1.0 * L06_INGREDIENT),
                    (carrot_crops, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::WheatSoup => {
            problem.add_constraint(
                &[
                    (wheat_soup, 1.0 * L10_INGREDIENT),
                    (wheat_crops, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::CabbageSoup => {
            problem.add_constraint(
                &[
                    (cabbage_soup, 1.0 * L12_INGREDIENT),
                    (cabbage_crops, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::GoatStew => {
            problem.add_constraint(
                &[
                    (goat_stew, 1.0 * L02_INGREDIENT),
                    (wheat_crops, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (wheat_crops, 1.0 * L04_INGREDIENT),
                    (raw_goat, -1.0 * L02_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (raw_goat, 1.0 * L02_INGREDIENT),
                    (turnip_crops, -1.0 * L04_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::MuttonStew => {
            problem.add_constraint(
                &[
                    (mutton_stew, 1.0 * L05_INGREDIENT),
                    (wheat_crops, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (wheat_crops, 1.0 * L08_INGREDIENT),
                    (raw_mutton, -1.0 * L05_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (raw_mutton, 1.0 * L05_INGREDIENT),
                    (potato_crops, -1.0 * L08_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::BeefStew => {
            problem.add_constraint(
                &[
                    (beef_stew, 1.0 * L09_INGREDIENT),
                    (pumpkin_crops, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (pumpkin_crops, 1.0 * L11_INGREDIENT),
                    (raw_beef, -1.0 * L09_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (raw_beef, 1.0 * L09_INGREDIENT),
                    (wheat_crops, -1.0 * L11_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::RoastChicken => {
            problem.add_constraint(
                &[
                    (roast_chicken, 1.0 * L04_INGREDIENT),
                    (raw_chicken, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (raw_chicken, 1.0 * L02_INGREDIENT),
                    (bean_crops, -1.0 * L04_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (bean_crops, 1.0 * L02_INGREDIENT),
                    (goats_milk, -1.0 * L02_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::RoastGoose => {
            problem.add_constraint(
                &[
                    (roast_goose, 1.0 * L08_INGREDIENT),
                    (raw_goose, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (raw_goose, 1.0 * L05_INGREDIENT),
                    (cabbage_crops, -1.0 * L08_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (cabbage_crops, 1.0 * L05_INGREDIENT),
                    (sheeps_milk, -1.0 * L05_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
        Product::RoastPork => {
            problem.add_constraint(
                &[
                    (roast_pork, 1.0 * L11_INGREDIENT),
                    (raw_pork, -1.0 * FOOD_PER_CRAFT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (raw_pork, 1.0 * L09_INGREDIENT),
                    (corn_crops, -1.0 * L11_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
            problem.add_constraint(
                &[
                    (corn_crops, 1.0 * L09_INGREDIENT),
                    (cows_milk, -1.0 * L09_INGREDIENT),
                ],
                ComparisonOp::Eq,
                0.0,
            );
        }
    }

    let solution = problem.clone().solve()?;

    let mut solution_herb_gardens_brecilien = solution[herb_gardens_brecilien];
    let mut solution_farms_brecilien = solution[farms_brecilien];
    let mut solution_pastures_brecilien = solution[pastures_brecilien];
    let mut solution_herb_gardens_bridgewatch = solution[herb_gardens_bridgewatch];
    let mut solution_farms_bridgewatch = solution[farms_bridgewatch];
    let mut solution_pastures_bridgewatch = solution[pastures_bridgewatch];
    let mut solution_herb_gardens_caerleon = solution[herb_gardens_caerleon];
    let mut solution_farms_caerleon = solution[farms_caerleon];
    let mut solution_pastures_caerleon = solution[pastures_caerleon];
    let mut solution_herb_gardens_fort_sterling = solution[herb_gardens_fort_sterling];
    let mut solution_farms_fort_sterling = solution[farms_fort_sterling];
    let mut solution_pastures_fort_sterling = solution[pastures_fort_sterling];
    let mut solution_herb_gardens_lymhurst = solution[herb_gardens_lymhurst];
    let mut solution_farms_lymhurst = solution[farms_lymhurst];
    let mut solution_pastures_lymhurst = solution[pastures_lymhurst];
    let mut solution_herb_gardens_martlock = solution[herb_gardens_martlock];
    let mut solution_farms_martlock = solution[farms_martlock];
    let mut solution_pastures_martlock = solution[pastures_martlock];
    let mut solution_herb_gardens_thetford = solution[herb_gardens_thetford];
    let mut solution_farms_thetford = solution[farms_thetford];
    let mut solution_pastures_thetford = solution[pastures_thetford];

    let mut plot_map = HashMap::new();
    plot_map.insert(herb_gardens_brecilien, solution_herb_gardens_brecilien);
    plot_map.insert(farms_brecilien, solution_farms_brecilien);
    plot_map.insert(pastures_brecilien, solution_pastures_brecilien);
    plot_map.insert(herb_gardens_bridgewatch, solution_herb_gardens_bridgewatch);
    plot_map.insert(farms_bridgewatch, solution_farms_bridgewatch);
    plot_map.insert(pastures_bridgewatch, solution_pastures_bridgewatch);
    plot_map.insert(herb_gardens_caerleon, solution_herb_gardens_caerleon);
    plot_map.insert(farms_caerleon, solution_farms_caerleon);
    plot_map.insert(pastures_caerleon, solution_pastures_caerleon);
    plot_map.insert(
        herb_gardens_fort_sterling,
        solution_herb_gardens_fort_sterling,
    );
    plot_map.insert(farms_fort_sterling, solution_farms_fort_sterling);
    plot_map.insert(pastures_fort_sterling, solution_pastures_fort_sterling);
    plot_map.insert(herb_gardens_lymhurst, solution_herb_gardens_lymhurst);
    plot_map.insert(farms_lymhurst, solution_farms_lymhurst);
    plot_map.insert(pastures_lymhurst, solution_pastures_lymhurst);
    plot_map.insert(herb_gardens_martlock, solution_herb_gardens_martlock);
    plot_map.insert(farms_martlock, solution_farms_martlock);
    plot_map.insert(pastures_martlock, solution_pastures_martlock);
    plot_map.insert(herb_gardens_thetford, solution_herb_gardens_thetford);
    plot_map.insert(farms_thetford, solution_farms_thetford);
    plot_map.insert(pastures_thetford, solution_pastures_thetford);

    smart_round(&mut plot_map);

    solution_herb_gardens_brecilien = *plot_map.get(&herb_gardens_brecilien).unwrap();
    solution_farms_brecilien = *plot_map.get(&farms_brecilien).unwrap();
    solution_pastures_brecilien = *plot_map.get(&pastures_brecilien).unwrap();
    solution_herb_gardens_bridgewatch = *plot_map.get(&herb_gardens_bridgewatch).unwrap();
    solution_farms_bridgewatch = *plot_map.get(&farms_bridgewatch).unwrap();
    solution_pastures_bridgewatch = *plot_map.get(&pastures_bridgewatch).unwrap();
    solution_herb_gardens_caerleon = *plot_map.get(&herb_gardens_caerleon).unwrap();
    solution_farms_caerleon = *plot_map.get(&farms_caerleon).unwrap();
    solution_pastures_caerleon = *plot_map.get(&pastures_caerleon).unwrap();
    solution_herb_gardens_fort_sterling = *plot_map.get(&herb_gardens_fort_sterling).unwrap();
    solution_farms_fort_sterling = *plot_map.get(&farms_fort_sterling).unwrap();
    solution_pastures_fort_sterling = *plot_map.get(&pastures_fort_sterling).unwrap();
    solution_herb_gardens_lymhurst = *plot_map.get(&herb_gardens_lymhurst).unwrap();
    solution_farms_lymhurst = *plot_map.get(&farms_lymhurst).unwrap();
    solution_pastures_lymhurst = *plot_map.get(&pastures_lymhurst).unwrap();
    solution_herb_gardens_martlock = *plot_map.get(&herb_gardens_martlock).unwrap();
    solution_farms_martlock = *plot_map.get(&farms_martlock).unwrap();
    solution_pastures_martlock = *plot_map.get(&pastures_martlock).unwrap();
    solution_herb_gardens_thetford = *plot_map.get(&herb_gardens_thetford).unwrap();
    solution_farms_thetford = *plot_map.get(&farms_thetford).unwrap();
    solution_pastures_thetford = *plot_map.get(&pastures_thetford).unwrap();

    // constrain patches
    problem.add_constraint(
        &[(
            herb_gardens_brecilien,
            if solution_herb_gardens_brecilien == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_herb_gardens_brecilien,
    );
    problem.add_constraint(
        &[(
            farms_brecilien,
            if solution_farms_brecilien == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_farms_brecilien,
    );
    problem.add_constraint(
        &[(
            pastures_brecilien,
            if solution_pastures_brecilien == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_pastures_brecilien,
    );
    problem.add_constraint(
        &[(
            herb_gardens_bridgewatch,
            if solution_herb_gardens_bridgewatch == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_herb_gardens_bridgewatch,
    );
    problem.add_constraint(
        &[(
            farms_bridgewatch,
            if solution_farms_bridgewatch == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_farms_bridgewatch,
    );
    problem.add_constraint(
        &[(
            pastures_bridgewatch,
            if solution_pastures_bridgewatch == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_pastures_bridgewatch,
    );
    problem.add_constraint(
        &[(
            herb_gardens_caerleon,
            if solution_herb_gardens_caerleon == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_herb_gardens_caerleon,
    );
    problem.add_constraint(
        &[(
            farms_caerleon,
            if solution_farms_caerleon == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_farms_caerleon,
    );
    problem.add_constraint(
        &[(
            pastures_caerleon,
            if solution_pastures_caerleon == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_pastures_caerleon,
    );
    problem.add_constraint(
        &[(
            herb_gardens_fort_sterling,
            if solution_herb_gardens_fort_sterling == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_herb_gardens_fort_sterling,
    );
    problem.add_constraint(
        &[(
            farms_fort_sterling,
            if solution_farms_fort_sterling == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_farms_fort_sterling,
    );
    problem.add_constraint(
        &[(
            pastures_fort_sterling,
            if solution_pastures_fort_sterling == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_pastures_fort_sterling,
    );
    problem.add_constraint(
        &[(
            herb_gardens_lymhurst,
            if solution_herb_gardens_lymhurst == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_herb_gardens_lymhurst,
    );
    problem.add_constraint(
        &[(
            farms_lymhurst,
            if solution_farms_lymhurst == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_farms_lymhurst,
    );
    problem.add_constraint(
        &[(
            pastures_lymhurst,
            if solution_pastures_lymhurst == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_pastures_lymhurst,
    );
    problem.add_constraint(
        &[(
            herb_gardens_martlock,
            if solution_herb_gardens_martlock == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_herb_gardens_martlock,
    );
    problem.add_constraint(
        &[(
            farms_martlock,
            if solution_farms_martlock == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_farms_martlock,
    );
    problem.add_constraint(
        &[(
            pastures_martlock,
            if solution_pastures_martlock == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_pastures_martlock,
    );
    problem.add_constraint(
        &[(
            herb_gardens_thetford,
            if solution_herb_gardens_thetford == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_herb_gardens_thetford,
    );
    problem.add_constraint(
        &[(
            farms_thetford,
            if solution_farms_thetford == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_farms_thetford,
    );
    problem.add_constraint(
        &[(
            pastures_thetford,
            if solution_pastures_thetford == 0.0 {
                10.0
            } else {
                1.0
            },
        )],
        ComparisonOp::Eq,
        solution_pastures_thetford,
    );

    let solution = problem.solve()?;

    // pack up the plot plan
    let mut herb_garden_tile_map_brecilien = HashMap::new();
    let mut solution_agaric_tiles_brecilien = solution[agaric_tiles_brecilien];
    let mut solution_comfrey_tiles_brecilien = solution[comfrey_tiles_brecilien];
    let mut solution_burdock_tiles_brecilien = solution[burdock_tiles_brecilien];
    let mut solution_teasel_tiles_brecilien = solution[teasel_tiles_brecilien];
    let mut solution_foxglove_tiles_brecilien = solution[foxglove_tiles_brecilien];
    let mut solution_muellin_tiles_brecilien = solution[muellin_tiles_brecilien];
    let mut solution_yarrow_tiles_brecilien = solution[yarrow_tiles_brecilien];

    herb_garden_tile_map_brecilien.insert(agaric_tiles_brecilien, solution_agaric_tiles_brecilien);
    herb_garden_tile_map_brecilien
        .insert(comfrey_tiles_brecilien, solution_comfrey_tiles_brecilien);
    herb_garden_tile_map_brecilien
        .insert(burdock_tiles_brecilien, solution_burdock_tiles_brecilien);
    herb_garden_tile_map_brecilien.insert(teasel_tiles_brecilien, solution_teasel_tiles_brecilien);
    herb_garden_tile_map_brecilien
        .insert(foxglove_tiles_brecilien, solution_foxglove_tiles_brecilien);
    herb_garden_tile_map_brecilien
        .insert(muellin_tiles_brecilien, solution_muellin_tiles_brecilien);
    herb_garden_tile_map_brecilien.insert(yarrow_tiles_brecilien, solution_yarrow_tiles_brecilien);

    smart_round(&mut herb_garden_tile_map_brecilien);

    solution_agaric_tiles_brecilien = *herb_garden_tile_map_brecilien
        .get(&agaric_tiles_brecilien)
        .unwrap();
    solution_comfrey_tiles_brecilien = *herb_garden_tile_map_brecilien
        .get(&comfrey_tiles_brecilien)
        .unwrap();
    solution_burdock_tiles_brecilien = *herb_garden_tile_map_brecilien
        .get(&burdock_tiles_brecilien)
        .unwrap();
    solution_teasel_tiles_brecilien = *herb_garden_tile_map_brecilien
        .get(&teasel_tiles_brecilien)
        .unwrap();
    solution_foxglove_tiles_brecilien = *herb_garden_tile_map_brecilien
        .get(&foxglove_tiles_brecilien)
        .unwrap();
    solution_muellin_tiles_brecilien = *herb_garden_tile_map_brecilien
        .get(&muellin_tiles_brecilien)
        .unwrap();
    solution_yarrow_tiles_brecilien = *herb_garden_tile_map_brecilien
        .get(&yarrow_tiles_brecilien)
        .unwrap();

    let mut farm_tile_map_brecilien = HashMap::new();
    let mut solution_carrot_tiles_brecilien = solution[carrot_tiles_brecilien];
    let mut solution_bean_tiles_brecilien = solution[bean_tiles_brecilien];
    let mut solution_wheat_tiles_brecilien = solution[wheat_tiles_brecilien];
    let mut solution_turnip_tiles_brecilien = solution[turnip_tiles_brecilien];
    let mut solution_cabbage_tiles_brecilien = solution[cabbage_tiles_brecilien];
    let mut solution_potato_tiles_brecilien = solution[potato_tiles_brecilien];
    let mut solution_corn_tiles_brecilien = solution[corn_tiles_brecilien];
    let mut solution_pumpkin_tiles_brecilien = solution[pumpkin_tiles_brecilien];

    farm_tile_map_brecilien.insert(carrot_tiles_brecilien, solution_carrot_tiles_brecilien);
    farm_tile_map_brecilien.insert(bean_tiles_brecilien, solution_bean_tiles_brecilien);
    farm_tile_map_brecilien.insert(wheat_tiles_brecilien, solution_wheat_tiles_brecilien);
    farm_tile_map_brecilien.insert(turnip_tiles_brecilien, solution_turnip_tiles_brecilien);
    farm_tile_map_brecilien.insert(cabbage_tiles_brecilien, solution_cabbage_tiles_brecilien);
    farm_tile_map_brecilien.insert(potato_tiles_brecilien, solution_potato_tiles_brecilien);
    farm_tile_map_brecilien.insert(corn_tiles_brecilien, solution_corn_tiles_brecilien);
    farm_tile_map_brecilien.insert(pumpkin_tiles_brecilien, solution_pumpkin_tiles_brecilien);

    smart_round(&mut farm_tile_map_brecilien);

    solution_carrot_tiles_brecilien = *farm_tile_map_brecilien
        .get(&carrot_tiles_brecilien)
        .unwrap();
    solution_bean_tiles_brecilien = *farm_tile_map_brecilien.get(&bean_tiles_brecilien).unwrap();
    solution_wheat_tiles_brecilien = *farm_tile_map_brecilien.get(&wheat_tiles_brecilien).unwrap();
    solution_turnip_tiles_brecilien = *farm_tile_map_brecilien
        .get(&turnip_tiles_brecilien)
        .unwrap();
    solution_cabbage_tiles_brecilien = *farm_tile_map_brecilien
        .get(&cabbage_tiles_brecilien)
        .unwrap();
    solution_potato_tiles_brecilien = *farm_tile_map_brecilien
        .get(&potato_tiles_brecilien)
        .unwrap();
    solution_corn_tiles_brecilien = *farm_tile_map_brecilien.get(&corn_tiles_brecilien).unwrap();
    solution_pumpkin_tiles_brecilien = *farm_tile_map_brecilien
        .get(&pumpkin_tiles_brecilien)
        .unwrap();

    let mut pasture_map_brecilien = HashMap::new();
    let mut solution_baby_chicken_tiles_brecilien = solution[baby_chicken_tiles_brecilien];
    let mut solution_kid_tiles_brecilien = solution[kid_tiles_brecilien];
    let mut solution_gosling_tiles_brecilien = solution[gosling_tiles_brecilien];
    let mut solution_lamb_tiles_brecilien = solution[lamb_tiles_brecilien];
    let mut solution_piglet_tiles_brecilien = solution[piglet_tiles_brecilien];
    let mut solution_calf_tiles_brecilien = solution[calf_tiles_brecilien];
    let mut solution_chicken_tiles_brecilien = solution[chicken_tiles_brecilien];
    let mut solution_goat_tiles_brecilien = solution[goat_tiles_brecilien];
    let mut solution_goose_tiles_brecilien = solution[goose_tiles_brecilien];
    let mut solution_sheep_tiles_brecilien = solution[sheep_tiles_brecilien];
    let mut solution_pig_tiles_brecilien = solution[pig_tiles_brecilien];
    let mut solution_cow_tiles_brecilien = solution[cow_tiles_brecilien];

    pasture_map_brecilien.insert(
        baby_chicken_tiles_brecilien,
        solution_baby_chicken_tiles_brecilien,
    );
    pasture_map_brecilien.insert(kid_tiles_brecilien, solution_kid_tiles_brecilien);
    pasture_map_brecilien.insert(gosling_tiles_brecilien, solution_gosling_tiles_brecilien);
    pasture_map_brecilien.insert(lamb_tiles_brecilien, solution_lamb_tiles_brecilien);
    pasture_map_brecilien.insert(piglet_tiles_brecilien, solution_piglet_tiles_brecilien);
    pasture_map_brecilien.insert(calf_tiles_brecilien, solution_calf_tiles_brecilien);
    pasture_map_brecilien.insert(chicken_tiles_brecilien, solution_chicken_tiles_brecilien);
    pasture_map_brecilien.insert(goat_tiles_brecilien, solution_goat_tiles_brecilien);
    pasture_map_brecilien.insert(goose_tiles_brecilien, solution_goose_tiles_brecilien);
    pasture_map_brecilien.insert(sheep_tiles_brecilien, solution_sheep_tiles_brecilien);
    pasture_map_brecilien.insert(pig_tiles_brecilien, solution_pig_tiles_brecilien);
    pasture_map_brecilien.insert(cow_tiles_brecilien, solution_cow_tiles_brecilien);

    smart_round(&mut pasture_map_brecilien);

    solution_baby_chicken_tiles_brecilien = *pasture_map_brecilien
        .get(&baby_chicken_tiles_brecilien)
        .unwrap();
    solution_kid_tiles_brecilien = *pasture_map_brecilien.get(&kid_tiles_brecilien).unwrap();
    solution_gosling_tiles_brecilien =
        *pasture_map_brecilien.get(&gosling_tiles_brecilien).unwrap();
    solution_lamb_tiles_brecilien = *pasture_map_brecilien.get(&lamb_tiles_brecilien).unwrap();
    solution_piglet_tiles_brecilien = *pasture_map_brecilien.get(&piglet_tiles_brecilien).unwrap();
    solution_calf_tiles_brecilien = *pasture_map_brecilien.get(&calf_tiles_brecilien).unwrap();
    solution_chicken_tiles_brecilien =
        *pasture_map_brecilien.get(&chicken_tiles_brecilien).unwrap();
    solution_goat_tiles_brecilien = *pasture_map_brecilien.get(&goat_tiles_brecilien).unwrap();
    solution_goose_tiles_brecilien = *pasture_map_brecilien.get(&goose_tiles_brecilien).unwrap();
    solution_sheep_tiles_brecilien = *pasture_map_brecilien.get(&sheep_tiles_brecilien).unwrap();
    solution_pig_tiles_brecilien = *pasture_map_brecilien.get(&pig_tiles_brecilien).unwrap();
    solution_cow_tiles_brecilien = *pasture_map_brecilien.get(&cow_tiles_brecilien).unwrap();

    let mut herb_garden_tile_map_bridgewatch = HashMap::new();
    let mut solution_agaric_tiles_bridgewatch = solution[agaric_tiles_bridgewatch];
    let mut solution_comfrey_tiles_bridgewatch = solution[comfrey_tiles_bridgewatch];
    let mut solution_burdock_tiles_bridgewatch = solution[burdock_tiles_bridgewatch];
    let mut solution_teasel_tiles_bridgewatch = solution[teasel_tiles_bridgewatch];
    let mut solution_foxglove_tiles_bridgewatch = solution[foxglove_tiles_bridgewatch];
    let mut solution_muellin_tiles_bridgewatch = solution[muellin_tiles_bridgewatch];
    let mut solution_yarrow_tiles_bridgewatch = solution[yarrow_tiles_bridgewatch];

    herb_garden_tile_map_bridgewatch
        .insert(agaric_tiles_bridgewatch, solution_agaric_tiles_bridgewatch);
    herb_garden_tile_map_bridgewatch.insert(
        comfrey_tiles_bridgewatch,
        solution_comfrey_tiles_bridgewatch,
    );
    herb_garden_tile_map_bridgewatch.insert(
        burdock_tiles_bridgewatch,
        solution_burdock_tiles_bridgewatch,
    );
    herb_garden_tile_map_bridgewatch
        .insert(teasel_tiles_bridgewatch, solution_teasel_tiles_bridgewatch);
    herb_garden_tile_map_bridgewatch.insert(
        foxglove_tiles_bridgewatch,
        solution_foxglove_tiles_bridgewatch,
    );
    herb_garden_tile_map_bridgewatch.insert(
        muellin_tiles_bridgewatch,
        solution_muellin_tiles_bridgewatch,
    );
    herb_garden_tile_map_bridgewatch
        .insert(yarrow_tiles_bridgewatch, solution_yarrow_tiles_bridgewatch);

    smart_round(&mut herb_garden_tile_map_bridgewatch);

    solution_agaric_tiles_bridgewatch = *herb_garden_tile_map_bridgewatch
        .get(&agaric_tiles_bridgewatch)
        .unwrap();
    solution_comfrey_tiles_bridgewatch = *herb_garden_tile_map_bridgewatch
        .get(&comfrey_tiles_bridgewatch)
        .unwrap();
    solution_burdock_tiles_bridgewatch = *herb_garden_tile_map_bridgewatch
        .get(&burdock_tiles_bridgewatch)
        .unwrap();
    solution_teasel_tiles_bridgewatch = *herb_garden_tile_map_bridgewatch
        .get(&teasel_tiles_bridgewatch)
        .unwrap();
    solution_foxglove_tiles_bridgewatch = *herb_garden_tile_map_bridgewatch
        .get(&foxglove_tiles_bridgewatch)
        .unwrap();
    solution_muellin_tiles_bridgewatch = *herb_garden_tile_map_bridgewatch
        .get(&muellin_tiles_bridgewatch)
        .unwrap();
    solution_yarrow_tiles_bridgewatch = *herb_garden_tile_map_bridgewatch
        .get(&yarrow_tiles_bridgewatch)
        .unwrap();

    let mut farm_tile_map_bridgewatch = HashMap::new();
    let mut solution_carrot_tiles_bridgewatch = solution[carrot_tiles_bridgewatch];
    let mut solution_bean_tiles_bridgewatch = solution[bean_tiles_bridgewatch];
    let mut solution_wheat_tiles_bridgewatch = solution[wheat_tiles_bridgewatch];
    let mut solution_turnip_tiles_bridgewatch = solution[turnip_tiles_bridgewatch];
    let mut solution_cabbage_tiles_bridgewatch = solution[cabbage_tiles_bridgewatch];
    let mut solution_potato_tiles_bridgewatch = solution[potato_tiles_bridgewatch];
    let mut solution_corn_tiles_bridgewatch = solution[corn_tiles_bridgewatch];
    let mut solution_pumpkin_tiles_bridgewatch = solution[pumpkin_tiles_bridgewatch];

    farm_tile_map_bridgewatch.insert(carrot_tiles_bridgewatch, solution_carrot_tiles_bridgewatch);
    farm_tile_map_bridgewatch.insert(bean_tiles_bridgewatch, solution_bean_tiles_bridgewatch);
    farm_tile_map_bridgewatch.insert(wheat_tiles_bridgewatch, solution_wheat_tiles_bridgewatch);
    farm_tile_map_bridgewatch.insert(turnip_tiles_bridgewatch, solution_turnip_tiles_bridgewatch);
    farm_tile_map_bridgewatch.insert(
        cabbage_tiles_bridgewatch,
        solution_cabbage_tiles_bridgewatch,
    );
    farm_tile_map_bridgewatch.insert(potato_tiles_bridgewatch, solution_potato_tiles_bridgewatch);
    farm_tile_map_bridgewatch.insert(corn_tiles_bridgewatch, solution_corn_tiles_bridgewatch);
    farm_tile_map_bridgewatch.insert(
        pumpkin_tiles_bridgewatch,
        solution_pumpkin_tiles_bridgewatch,
    );

    smart_round(&mut farm_tile_map_bridgewatch);

    solution_carrot_tiles_bridgewatch = *farm_tile_map_bridgewatch
        .get(&carrot_tiles_bridgewatch)
        .unwrap();
    solution_bean_tiles_bridgewatch = *farm_tile_map_bridgewatch
        .get(&bean_tiles_bridgewatch)
        .unwrap();
    solution_wheat_tiles_bridgewatch = *farm_tile_map_bridgewatch
        .get(&wheat_tiles_bridgewatch)
        .unwrap();
    solution_turnip_tiles_bridgewatch = *farm_tile_map_bridgewatch
        .get(&turnip_tiles_bridgewatch)
        .unwrap();
    solution_cabbage_tiles_bridgewatch = *farm_tile_map_bridgewatch
        .get(&cabbage_tiles_bridgewatch)
        .unwrap();
    solution_potato_tiles_bridgewatch = *farm_tile_map_bridgewatch
        .get(&potato_tiles_bridgewatch)
        .unwrap();
    solution_corn_tiles_bridgewatch = *farm_tile_map_bridgewatch
        .get(&corn_tiles_bridgewatch)
        .unwrap();
    solution_pumpkin_tiles_bridgewatch = *farm_tile_map_bridgewatch
        .get(&pumpkin_tiles_bridgewatch)
        .unwrap();

    let mut pasture_map_bridgewatch = HashMap::new();
    let mut solution_baby_chicken_tiles_bridgewatch = solution[baby_chicken_tiles_bridgewatch];
    let mut solution_kid_tiles_bridgewatch = solution[kid_tiles_bridgewatch];
    let mut solution_gosling_tiles_bridgewatch = solution[gosling_tiles_bridgewatch];
    let mut solution_lamb_tiles_bridgewatch = solution[lamb_tiles_bridgewatch];
    let mut solution_piglet_tiles_bridgewatch = solution[piglet_tiles_bridgewatch];
    let mut solution_calf_tiles_bridgewatch = solution[calf_tiles_bridgewatch];
    let mut solution_chicken_tiles_bridgewatch = solution[chicken_tiles_bridgewatch];
    let mut solution_goat_tiles_bridgewatch = solution[goat_tiles_bridgewatch];
    let mut solution_goose_tiles_bridgewatch = solution[goose_tiles_bridgewatch];
    let mut solution_sheep_tiles_bridgewatch = solution[sheep_tiles_bridgewatch];
    let mut solution_pig_tiles_bridgewatch = solution[pig_tiles_bridgewatch];
    let mut solution_cow_tiles_bridgewatch = solution[cow_tiles_bridgewatch];

    pasture_map_bridgewatch.insert(
        baby_chicken_tiles_bridgewatch,
        solution_baby_chicken_tiles_bridgewatch,
    );
    pasture_map_bridgewatch.insert(kid_tiles_bridgewatch, solution_kid_tiles_bridgewatch);
    pasture_map_bridgewatch.insert(
        gosling_tiles_bridgewatch,
        solution_gosling_tiles_bridgewatch,
    );
    pasture_map_bridgewatch.insert(lamb_tiles_bridgewatch, solution_lamb_tiles_bridgewatch);
    pasture_map_bridgewatch.insert(piglet_tiles_bridgewatch, solution_piglet_tiles_bridgewatch);
    pasture_map_bridgewatch.insert(calf_tiles_bridgewatch, solution_calf_tiles_bridgewatch);
    pasture_map_bridgewatch.insert(
        chicken_tiles_bridgewatch,
        solution_chicken_tiles_bridgewatch,
    );
    pasture_map_bridgewatch.insert(goat_tiles_bridgewatch, solution_goat_tiles_bridgewatch);
    pasture_map_bridgewatch.insert(goose_tiles_bridgewatch, solution_goose_tiles_bridgewatch);
    pasture_map_bridgewatch.insert(sheep_tiles_bridgewatch, solution_sheep_tiles_bridgewatch);
    pasture_map_bridgewatch.insert(pig_tiles_bridgewatch, solution_pig_tiles_bridgewatch);
    pasture_map_bridgewatch.insert(cow_tiles_bridgewatch, solution_cow_tiles_bridgewatch);

    smart_round(&mut pasture_map_bridgewatch);

    solution_baby_chicken_tiles_bridgewatch = *pasture_map_bridgewatch
        .get(&baby_chicken_tiles_bridgewatch)
        .unwrap();
    solution_kid_tiles_bridgewatch = *pasture_map_bridgewatch.get(&kid_tiles_bridgewatch).unwrap();
    solution_gosling_tiles_bridgewatch = *pasture_map_bridgewatch
        .get(&gosling_tiles_bridgewatch)
        .unwrap();
    solution_lamb_tiles_bridgewatch = *pasture_map_bridgewatch
        .get(&lamb_tiles_bridgewatch)
        .unwrap();
    solution_piglet_tiles_bridgewatch = *pasture_map_bridgewatch
        .get(&piglet_tiles_bridgewatch)
        .unwrap();
    solution_calf_tiles_bridgewatch = *pasture_map_bridgewatch
        .get(&calf_tiles_bridgewatch)
        .unwrap();
    solution_chicken_tiles_bridgewatch = *pasture_map_bridgewatch
        .get(&chicken_tiles_bridgewatch)
        .unwrap();
    solution_goat_tiles_bridgewatch = *pasture_map_bridgewatch
        .get(&goat_tiles_bridgewatch)
        .unwrap();
    solution_goose_tiles_bridgewatch = *pasture_map_bridgewatch
        .get(&goose_tiles_bridgewatch)
        .unwrap();
    solution_sheep_tiles_bridgewatch = *pasture_map_bridgewatch
        .get(&sheep_tiles_bridgewatch)
        .unwrap();
    solution_pig_tiles_bridgewatch = *pasture_map_bridgewatch.get(&pig_tiles_bridgewatch).unwrap();
    solution_cow_tiles_bridgewatch = *pasture_map_bridgewatch.get(&cow_tiles_bridgewatch).unwrap();

    let mut herb_garden_tile_map_caerleon = HashMap::new();
    let mut solution_agaric_tiles_caerleon = solution[agaric_tiles_caerleon];
    let mut solution_comfrey_tiles_caerleon = solution[comfrey_tiles_caerleon];
    let mut solution_burdock_tiles_caerleon = solution[burdock_tiles_caerleon];
    let mut solution_teasel_tiles_caerleon = solution[teasel_tiles_caerleon];
    let mut solution_foxglove_tiles_caerleon = solution[foxglove_tiles_caerleon];
    let mut solution_muellin_tiles_caerleon = solution[muellin_tiles_caerleon];
    let mut solution_yarrow_tiles_caerleon = solution[yarrow_tiles_caerleon];

    herb_garden_tile_map_caerleon.insert(agaric_tiles_caerleon, solution_agaric_tiles_caerleon);
    herb_garden_tile_map_caerleon.insert(comfrey_tiles_caerleon, solution_comfrey_tiles_caerleon);
    herb_garden_tile_map_caerleon.insert(burdock_tiles_caerleon, solution_burdock_tiles_caerleon);
    herb_garden_tile_map_caerleon.insert(teasel_tiles_caerleon, solution_teasel_tiles_caerleon);
    herb_garden_tile_map_caerleon.insert(foxglove_tiles_caerleon, solution_foxglove_tiles_caerleon);
    herb_garden_tile_map_caerleon.insert(muellin_tiles_caerleon, solution_muellin_tiles_caerleon);
    herb_garden_tile_map_caerleon.insert(yarrow_tiles_caerleon, solution_yarrow_tiles_caerleon);

    smart_round(&mut herb_garden_tile_map_caerleon);

    solution_agaric_tiles_caerleon = *herb_garden_tile_map_caerleon
        .get(&agaric_tiles_caerleon)
        .unwrap();
    solution_comfrey_tiles_caerleon = *herb_garden_tile_map_caerleon
        .get(&comfrey_tiles_caerleon)
        .unwrap();
    solution_burdock_tiles_caerleon = *herb_garden_tile_map_caerleon
        .get(&burdock_tiles_caerleon)
        .unwrap();
    solution_teasel_tiles_caerleon = *herb_garden_tile_map_caerleon
        .get(&teasel_tiles_caerleon)
        .unwrap();
    solution_foxglove_tiles_caerleon = *herb_garden_tile_map_caerleon
        .get(&foxglove_tiles_caerleon)
        .unwrap();
    solution_muellin_tiles_caerleon = *herb_garden_tile_map_caerleon
        .get(&muellin_tiles_caerleon)
        .unwrap();
    solution_yarrow_tiles_caerleon = *herb_garden_tile_map_caerleon
        .get(&yarrow_tiles_caerleon)
        .unwrap();

    let mut farm_tile_map_caerleon = HashMap::new();
    let mut solution_carrot_tiles_caerleon = solution[carrot_tiles_caerleon];
    let mut solution_bean_tiles_caerleon = solution[bean_tiles_caerleon];
    let mut solution_wheat_tiles_caerleon = solution[wheat_tiles_caerleon];
    let mut solution_turnip_tiles_caerleon = solution[turnip_tiles_caerleon];
    let mut solution_cabbage_tiles_caerleon = solution[cabbage_tiles_caerleon];
    let mut solution_potato_tiles_caerleon = solution[potato_tiles_caerleon];
    let mut solution_corn_tiles_caerleon = solution[corn_tiles_caerleon];
    let mut solution_pumpkin_tiles_caerleon = solution[pumpkin_tiles_caerleon];

    farm_tile_map_caerleon.insert(carrot_tiles_caerleon, solution_carrot_tiles_caerleon);
    farm_tile_map_caerleon.insert(bean_tiles_caerleon, solution_bean_tiles_caerleon);
    farm_tile_map_caerleon.insert(wheat_tiles_caerleon, solution_wheat_tiles_caerleon);
    farm_tile_map_caerleon.insert(turnip_tiles_caerleon, solution_turnip_tiles_caerleon);
    farm_tile_map_caerleon.insert(cabbage_tiles_caerleon, solution_cabbage_tiles_caerleon);
    farm_tile_map_caerleon.insert(potato_tiles_caerleon, solution_potato_tiles_caerleon);
    farm_tile_map_caerleon.insert(corn_tiles_caerleon, solution_corn_tiles_caerleon);
    farm_tile_map_caerleon.insert(pumpkin_tiles_caerleon, solution_pumpkin_tiles_caerleon);

    smart_round(&mut farm_tile_map_caerleon);

    solution_carrot_tiles_caerleon = *farm_tile_map_caerleon.get(&carrot_tiles_caerleon).unwrap();
    solution_bean_tiles_caerleon = *farm_tile_map_caerleon.get(&bean_tiles_caerleon).unwrap();
    solution_wheat_tiles_caerleon = *farm_tile_map_caerleon.get(&wheat_tiles_caerleon).unwrap();
    solution_turnip_tiles_caerleon = *farm_tile_map_caerleon.get(&turnip_tiles_caerleon).unwrap();
    solution_cabbage_tiles_caerleon = *farm_tile_map_caerleon.get(&cabbage_tiles_caerleon).unwrap();
    solution_potato_tiles_caerleon = *farm_tile_map_caerleon.get(&potato_tiles_caerleon).unwrap();
    solution_corn_tiles_caerleon = *farm_tile_map_caerleon.get(&corn_tiles_caerleon).unwrap();
    solution_pumpkin_tiles_caerleon = *farm_tile_map_caerleon.get(&pumpkin_tiles_caerleon).unwrap();

    let mut pasture_map_caerleon = HashMap::new();
    let mut solution_baby_chicken_tiles_caerleon = solution[baby_chicken_tiles_caerleon];
    let mut solution_kid_tiles_caerleon = solution[kid_tiles_caerleon];
    let mut solution_gosling_tiles_caerleon = solution[gosling_tiles_caerleon];
    let mut solution_lamb_tiles_caerleon = solution[lamb_tiles_caerleon];
    let mut solution_piglet_tiles_caerleon = solution[piglet_tiles_caerleon];
    let mut solution_calf_tiles_caerleon = solution[calf_tiles_caerleon];
    let mut solution_chicken_tiles_caerleon = solution[chicken_tiles_caerleon];
    let mut solution_goat_tiles_caerleon = solution[goat_tiles_caerleon];
    let mut solution_goose_tiles_caerleon = solution[goose_tiles_caerleon];
    let mut solution_sheep_tiles_caerleon = solution[sheep_tiles_caerleon];
    let mut solution_pig_tiles_caerleon = solution[pig_tiles_caerleon];
    let mut solution_cow_tiles_caerleon = solution[cow_tiles_caerleon];

    pasture_map_caerleon.insert(
        baby_chicken_tiles_caerleon,
        solution_baby_chicken_tiles_caerleon,
    );
    pasture_map_caerleon.insert(kid_tiles_caerleon, solution_kid_tiles_caerleon);
    pasture_map_caerleon.insert(gosling_tiles_caerleon, solution_gosling_tiles_caerleon);
    pasture_map_caerleon.insert(lamb_tiles_caerleon, solution_lamb_tiles_caerleon);
    pasture_map_caerleon.insert(piglet_tiles_caerleon, solution_piglet_tiles_caerleon);
    pasture_map_caerleon.insert(calf_tiles_caerleon, solution_calf_tiles_caerleon);
    pasture_map_caerleon.insert(chicken_tiles_caerleon, solution_chicken_tiles_caerleon);
    pasture_map_caerleon.insert(goat_tiles_caerleon, solution_goat_tiles_caerleon);
    pasture_map_caerleon.insert(goose_tiles_caerleon, solution_goose_tiles_caerleon);
    pasture_map_caerleon.insert(sheep_tiles_caerleon, solution_sheep_tiles_caerleon);
    pasture_map_caerleon.insert(pig_tiles_caerleon, solution_pig_tiles_caerleon);
    pasture_map_caerleon.insert(cow_tiles_caerleon, solution_cow_tiles_caerleon);

    smart_round(&mut pasture_map_caerleon);

    solution_baby_chicken_tiles_caerleon = *pasture_map_caerleon
        .get(&baby_chicken_tiles_caerleon)
        .unwrap();
    solution_kid_tiles_caerleon = *pasture_map_caerleon.get(&kid_tiles_caerleon).unwrap();
    solution_gosling_tiles_caerleon = *pasture_map_caerleon.get(&gosling_tiles_caerleon).unwrap();
    solution_lamb_tiles_caerleon = *pasture_map_caerleon.get(&lamb_tiles_caerleon).unwrap();
    solution_piglet_tiles_caerleon = *pasture_map_caerleon.get(&piglet_tiles_caerleon).unwrap();
    solution_calf_tiles_caerleon = *pasture_map_caerleon.get(&calf_tiles_caerleon).unwrap();
    solution_chicken_tiles_caerleon = *pasture_map_caerleon.get(&chicken_tiles_caerleon).unwrap();
    solution_goat_tiles_caerleon = *pasture_map_caerleon.get(&goat_tiles_caerleon).unwrap();
    solution_goose_tiles_caerleon = *pasture_map_caerleon.get(&goose_tiles_caerleon).unwrap();
    solution_sheep_tiles_caerleon = *pasture_map_caerleon.get(&sheep_tiles_caerleon).unwrap();
    solution_pig_tiles_caerleon = *pasture_map_caerleon.get(&pig_tiles_caerleon).unwrap();
    solution_cow_tiles_caerleon = *pasture_map_caerleon.get(&cow_tiles_caerleon).unwrap();

    let mut herb_garden_tile_map_fort_sterling = HashMap::new();
    let mut solution_agaric_tiles_fort_sterling = solution[agaric_tiles_fort_sterling];
    let mut solution_comfrey_tiles_fort_sterling = solution[comfrey_tiles_fort_sterling];
    let mut solution_burdock_tiles_fort_sterling = solution[burdock_tiles_fort_sterling];
    let mut solution_teasel_tiles_fort_sterling = solution[teasel_tiles_fort_sterling];
    let mut solution_foxglove_tiles_fort_sterling = solution[foxglove_tiles_fort_sterling];
    let mut solution_muellin_tiles_fort_sterling = solution[muellin_tiles_fort_sterling];
    let mut solution_yarrow_tiles_fort_sterling = solution[yarrow_tiles_fort_sterling];

    herb_garden_tile_map_fort_sterling.insert(
        agaric_tiles_fort_sterling,
        solution_agaric_tiles_fort_sterling,
    );
    herb_garden_tile_map_fort_sterling.insert(
        comfrey_tiles_fort_sterling,
        solution_comfrey_tiles_fort_sterling,
    );
    herb_garden_tile_map_fort_sterling.insert(
        burdock_tiles_fort_sterling,
        solution_burdock_tiles_fort_sterling,
    );
    herb_garden_tile_map_fort_sterling.insert(
        teasel_tiles_fort_sterling,
        solution_teasel_tiles_fort_sterling,
    );
    herb_garden_tile_map_fort_sterling.insert(
        foxglove_tiles_fort_sterling,
        solution_foxglove_tiles_fort_sterling,
    );
    herb_garden_tile_map_fort_sterling.insert(
        muellin_tiles_fort_sterling,
        solution_muellin_tiles_fort_sterling,
    );
    herb_garden_tile_map_fort_sterling.insert(
        yarrow_tiles_fort_sterling,
        solution_yarrow_tiles_fort_sterling,
    );

    smart_round(&mut herb_garden_tile_map_fort_sterling);

    solution_agaric_tiles_fort_sterling = *herb_garden_tile_map_fort_sterling
        .get(&agaric_tiles_fort_sterling)
        .unwrap();
    solution_comfrey_tiles_fort_sterling = *herb_garden_tile_map_fort_sterling
        .get(&comfrey_tiles_fort_sterling)
        .unwrap();
    solution_burdock_tiles_fort_sterling = *herb_garden_tile_map_fort_sterling
        .get(&burdock_tiles_fort_sterling)
        .unwrap();
    solution_teasel_tiles_fort_sterling = *herb_garden_tile_map_fort_sterling
        .get(&teasel_tiles_fort_sterling)
        .unwrap();
    solution_foxglove_tiles_fort_sterling = *herb_garden_tile_map_fort_sterling
        .get(&foxglove_tiles_fort_sterling)
        .unwrap();
    solution_muellin_tiles_fort_sterling = *herb_garden_tile_map_fort_sterling
        .get(&muellin_tiles_fort_sterling)
        .unwrap();
    solution_yarrow_tiles_fort_sterling = *herb_garden_tile_map_fort_sterling
        .get(&yarrow_tiles_fort_sterling)
        .unwrap();

    let mut farm_tile_map_fort_sterling = HashMap::new();
    let mut solution_carrot_tiles_fort_sterling = solution[carrot_tiles_fort_sterling];
    let mut solution_bean_tiles_fort_sterling = solution[bean_tiles_fort_sterling];
    let mut solution_wheat_tiles_fort_sterling = solution[wheat_tiles_fort_sterling];
    let mut solution_turnip_tiles_fort_sterling = solution[turnip_tiles_fort_sterling];
    let mut solution_cabbage_tiles_fort_sterling = solution[cabbage_tiles_fort_sterling];
    let mut solution_potato_tiles_fort_sterling = solution[potato_tiles_fort_sterling];
    let mut solution_corn_tiles_fort_sterling = solution[corn_tiles_fort_sterling];
    let mut solution_pumpkin_tiles_fort_sterling = solution[pumpkin_tiles_fort_sterling];

    farm_tile_map_fort_sterling.insert(
        carrot_tiles_fort_sterling,
        solution_carrot_tiles_fort_sterling,
    );
    farm_tile_map_fort_sterling.insert(bean_tiles_fort_sterling, solution_bean_tiles_fort_sterling);
    farm_tile_map_fort_sterling.insert(
        wheat_tiles_fort_sterling,
        solution_wheat_tiles_fort_sterling,
    );
    farm_tile_map_fort_sterling.insert(
        turnip_tiles_fort_sterling,
        solution_turnip_tiles_fort_sterling,
    );
    farm_tile_map_fort_sterling.insert(
        cabbage_tiles_fort_sterling,
        solution_cabbage_tiles_fort_sterling,
    );
    farm_tile_map_fort_sterling.insert(
        potato_tiles_fort_sterling,
        solution_potato_tiles_fort_sterling,
    );
    farm_tile_map_fort_sterling.insert(corn_tiles_fort_sterling, solution_corn_tiles_fort_sterling);
    farm_tile_map_fort_sterling.insert(
        pumpkin_tiles_fort_sterling,
        solution_pumpkin_tiles_fort_sterling,
    );

    smart_round(&mut farm_tile_map_fort_sterling);

    solution_carrot_tiles_fort_sterling = *farm_tile_map_fort_sterling
        .get(&carrot_tiles_fort_sterling)
        .unwrap();
    solution_bean_tiles_fort_sterling = *farm_tile_map_fort_sterling
        .get(&bean_tiles_fort_sterling)
        .unwrap();
    solution_wheat_tiles_fort_sterling = *farm_tile_map_fort_sterling
        .get(&wheat_tiles_fort_sterling)
        .unwrap();
    solution_turnip_tiles_fort_sterling = *farm_tile_map_fort_sterling
        .get(&turnip_tiles_fort_sterling)
        .unwrap();
    solution_cabbage_tiles_fort_sterling = *farm_tile_map_fort_sterling
        .get(&cabbage_tiles_fort_sterling)
        .unwrap();
    solution_potato_tiles_fort_sterling = *farm_tile_map_fort_sterling
        .get(&potato_tiles_fort_sterling)
        .unwrap();
    solution_corn_tiles_fort_sterling = *farm_tile_map_fort_sterling
        .get(&corn_tiles_fort_sterling)
        .unwrap();
    solution_pumpkin_tiles_fort_sterling = *farm_tile_map_fort_sterling
        .get(&pumpkin_tiles_fort_sterling)
        .unwrap();

    let mut pasture_map_fort_sterling = HashMap::new();
    let mut solution_baby_chicken_tiles_fort_sterling = solution[baby_chicken_tiles_fort_sterling];
    let mut solution_kid_tiles_fort_sterling = solution[kid_tiles_fort_sterling];
    let mut solution_gosling_tiles_fort_sterling = solution[gosling_tiles_fort_sterling];
    let mut solution_lamb_tiles_fort_sterling = solution[lamb_tiles_fort_sterling];
    let mut solution_piglet_tiles_fort_sterling = solution[piglet_tiles_fort_sterling];
    let mut solution_calf_tiles_fort_sterling = solution[calf_tiles_fort_sterling];
    let mut solution_chicken_tiles_fort_sterling = solution[chicken_tiles_fort_sterling];
    let mut solution_goat_tiles_fort_sterling = solution[goat_tiles_fort_sterling];
    let mut solution_goose_tiles_fort_sterling = solution[goose_tiles_fort_sterling];
    let mut solution_sheep_tiles_fort_sterling = solution[sheep_tiles_fort_sterling];
    let mut solution_pig_tiles_fort_sterling = solution[pig_tiles_fort_sterling];
    let mut solution_cow_tiles_fort_sterling = solution[cow_tiles_fort_sterling];

    pasture_map_fort_sterling.insert(
        baby_chicken_tiles_fort_sterling,
        solution_baby_chicken_tiles_fort_sterling,
    );
    pasture_map_fort_sterling.insert(kid_tiles_fort_sterling, solution_kid_tiles_fort_sterling);
    pasture_map_fort_sterling.insert(
        gosling_tiles_fort_sterling,
        solution_gosling_tiles_fort_sterling,
    );
    pasture_map_fort_sterling.insert(lamb_tiles_fort_sterling, solution_lamb_tiles_fort_sterling);
    pasture_map_fort_sterling.insert(
        piglet_tiles_fort_sterling,
        solution_piglet_tiles_fort_sterling,
    );
    pasture_map_fort_sterling.insert(calf_tiles_fort_sterling, solution_calf_tiles_fort_sterling);
    pasture_map_fort_sterling.insert(
        chicken_tiles_fort_sterling,
        solution_chicken_tiles_fort_sterling,
    );
    pasture_map_fort_sterling.insert(goat_tiles_fort_sterling, solution_goat_tiles_fort_sterling);
    pasture_map_fort_sterling.insert(
        goose_tiles_fort_sterling,
        solution_goose_tiles_fort_sterling,
    );
    pasture_map_fort_sterling.insert(
        sheep_tiles_fort_sterling,
        solution_sheep_tiles_fort_sterling,
    );
    pasture_map_fort_sterling.insert(pig_tiles_fort_sterling, solution_pig_tiles_fort_sterling);
    pasture_map_fort_sterling.insert(cow_tiles_fort_sterling, solution_cow_tiles_fort_sterling);

    smart_round(&mut pasture_map_fort_sterling);

    solution_baby_chicken_tiles_fort_sterling = *pasture_map_fort_sterling
        .get(&baby_chicken_tiles_fort_sterling)
        .unwrap();
    solution_kid_tiles_fort_sterling = *pasture_map_fort_sterling
        .get(&kid_tiles_fort_sterling)
        .unwrap();
    solution_gosling_tiles_fort_sterling = *pasture_map_fort_sterling
        .get(&gosling_tiles_fort_sterling)
        .unwrap();
    solution_lamb_tiles_fort_sterling = *pasture_map_fort_sterling
        .get(&lamb_tiles_fort_sterling)
        .unwrap();
    solution_piglet_tiles_fort_sterling = *pasture_map_fort_sterling
        .get(&piglet_tiles_fort_sterling)
        .unwrap();
    solution_calf_tiles_fort_sterling = *pasture_map_fort_sterling
        .get(&calf_tiles_fort_sterling)
        .unwrap();
    solution_chicken_tiles_fort_sterling = *pasture_map_fort_sterling
        .get(&chicken_tiles_fort_sterling)
        .unwrap();
    solution_goat_tiles_fort_sterling = *pasture_map_fort_sterling
        .get(&goat_tiles_fort_sterling)
        .unwrap();
    solution_goose_tiles_fort_sterling = *pasture_map_fort_sterling
        .get(&goose_tiles_fort_sterling)
        .unwrap();
    solution_sheep_tiles_fort_sterling = *pasture_map_fort_sterling
        .get(&sheep_tiles_fort_sterling)
        .unwrap();
    solution_pig_tiles_fort_sterling = *pasture_map_fort_sterling
        .get(&pig_tiles_fort_sterling)
        .unwrap();
    solution_cow_tiles_fort_sterling = *pasture_map_fort_sterling
        .get(&cow_tiles_fort_sterling)
        .unwrap();

    let mut herb_garden_tile_map_lymhurst = HashMap::new();
    let mut solution_agaric_tiles_lymhurst = solution[agaric_tiles_lymhurst];
    let mut solution_comfrey_tiles_lymhurst = solution[comfrey_tiles_lymhurst];
    let mut solution_burdock_tiles_lymhurst = solution[burdock_tiles_lymhurst];
    let mut solution_teasel_tiles_lymhurst = solution[teasel_tiles_lymhurst];
    let mut solution_foxglove_tiles_lymhurst = solution[foxglove_tiles_lymhurst];
    let mut solution_muellin_tiles_lymhurst = solution[muellin_tiles_lymhurst];
    let mut solution_yarrow_tiles_lymhurst = solution[yarrow_tiles_lymhurst];

    herb_garden_tile_map_lymhurst.insert(agaric_tiles_lymhurst, solution_agaric_tiles_lymhurst);
    herb_garden_tile_map_lymhurst.insert(comfrey_tiles_lymhurst, solution_comfrey_tiles_lymhurst);
    herb_garden_tile_map_lymhurst.insert(burdock_tiles_lymhurst, solution_burdock_tiles_lymhurst);
    herb_garden_tile_map_lymhurst.insert(teasel_tiles_lymhurst, solution_teasel_tiles_lymhurst);
    herb_garden_tile_map_lymhurst.insert(foxglove_tiles_lymhurst, solution_foxglove_tiles_lymhurst);
    herb_garden_tile_map_lymhurst.insert(muellin_tiles_lymhurst, solution_muellin_tiles_lymhurst);
    herb_garden_tile_map_lymhurst.insert(yarrow_tiles_lymhurst, solution_yarrow_tiles_lymhurst);

    smart_round(&mut herb_garden_tile_map_lymhurst);

    solution_agaric_tiles_lymhurst = *herb_garden_tile_map_lymhurst
        .get(&agaric_tiles_lymhurst)
        .unwrap();
    solution_comfrey_tiles_lymhurst = *herb_garden_tile_map_lymhurst
        .get(&comfrey_tiles_lymhurst)
        .unwrap();
    solution_burdock_tiles_lymhurst = *herb_garden_tile_map_lymhurst
        .get(&burdock_tiles_lymhurst)
        .unwrap();
    solution_teasel_tiles_lymhurst = *herb_garden_tile_map_lymhurst
        .get(&teasel_tiles_lymhurst)
        .unwrap();
    solution_foxglove_tiles_lymhurst = *herb_garden_tile_map_lymhurst
        .get(&foxglove_tiles_lymhurst)
        .unwrap();
    solution_muellin_tiles_lymhurst = *herb_garden_tile_map_lymhurst
        .get(&muellin_tiles_lymhurst)
        .unwrap();
    solution_yarrow_tiles_lymhurst = *herb_garden_tile_map_lymhurst
        .get(&yarrow_tiles_lymhurst)
        .unwrap();

    let mut farm_tile_map_lymhurst = HashMap::new();
    let mut solution_carrot_tiles_lymhurst = solution[carrot_tiles_lymhurst];
    let mut solution_bean_tiles_lymhurst = solution[bean_tiles_lymhurst];
    let mut solution_wheat_tiles_lymhurst = solution[wheat_tiles_lymhurst];
    let mut solution_turnip_tiles_lymhurst = solution[turnip_tiles_lymhurst];
    let mut solution_cabbage_tiles_lymhurst = solution[cabbage_tiles_lymhurst];
    let mut solution_potato_tiles_lymhurst = solution[potato_tiles_lymhurst];
    let mut solution_corn_tiles_lymhurst = solution[corn_tiles_lymhurst];
    let mut solution_pumpkin_tiles_lymhurst = solution[pumpkin_tiles_lymhurst];

    farm_tile_map_lymhurst.insert(carrot_tiles_lymhurst, solution_carrot_tiles_lymhurst);
    farm_tile_map_lymhurst.insert(bean_tiles_lymhurst, solution_bean_tiles_lymhurst);
    farm_tile_map_lymhurst.insert(wheat_tiles_lymhurst, solution_wheat_tiles_lymhurst);
    farm_tile_map_lymhurst.insert(turnip_tiles_lymhurst, solution_turnip_tiles_lymhurst);
    farm_tile_map_lymhurst.insert(cabbage_tiles_lymhurst, solution_cabbage_tiles_lymhurst);
    farm_tile_map_lymhurst.insert(potato_tiles_lymhurst, solution_potato_tiles_lymhurst);
    farm_tile_map_lymhurst.insert(corn_tiles_lymhurst, solution_corn_tiles_lymhurst);
    farm_tile_map_lymhurst.insert(pumpkin_tiles_lymhurst, solution_pumpkin_tiles_lymhurst);

    smart_round(&mut farm_tile_map_lymhurst);

    solution_carrot_tiles_lymhurst = *farm_tile_map_lymhurst.get(&carrot_tiles_lymhurst).unwrap();
    solution_bean_tiles_lymhurst = *farm_tile_map_lymhurst.get(&bean_tiles_lymhurst).unwrap();
    solution_wheat_tiles_lymhurst = *farm_tile_map_lymhurst.get(&wheat_tiles_lymhurst).unwrap();
    solution_turnip_tiles_lymhurst = *farm_tile_map_lymhurst.get(&turnip_tiles_lymhurst).unwrap();
    solution_cabbage_tiles_lymhurst = *farm_tile_map_lymhurst.get(&cabbage_tiles_lymhurst).unwrap();
    solution_potato_tiles_lymhurst = *farm_tile_map_lymhurst.get(&potato_tiles_lymhurst).unwrap();
    solution_corn_tiles_lymhurst = *farm_tile_map_lymhurst.get(&corn_tiles_lymhurst).unwrap();
    solution_pumpkin_tiles_lymhurst = *farm_tile_map_lymhurst.get(&pumpkin_tiles_lymhurst).unwrap();

    let mut pasture_map_lymhurst = HashMap::new();
    let mut solution_baby_chicken_tiles_lymhurst = solution[baby_chicken_tiles_lymhurst];
    let mut solution_kid_tiles_lymhurst = solution[kid_tiles_lymhurst];
    let mut solution_gosling_tiles_lymhurst = solution[gosling_tiles_lymhurst];
    let mut solution_lamb_tiles_lymhurst = solution[lamb_tiles_lymhurst];
    let mut solution_piglet_tiles_lymhurst = solution[piglet_tiles_lymhurst];
    let mut solution_calf_tiles_lymhurst = solution[calf_tiles_lymhurst];
    let mut solution_chicken_tiles_lymhurst = solution[chicken_tiles_lymhurst];
    let mut solution_goat_tiles_lymhurst = solution[goat_tiles_lymhurst];
    let mut solution_goose_tiles_lymhurst = solution[goose_tiles_lymhurst];
    let mut solution_sheep_tiles_lymhurst = solution[sheep_tiles_lymhurst];
    let mut solution_pig_tiles_lymhurst = solution[pig_tiles_lymhurst];
    let mut solution_cow_tiles_lymhurst = solution[cow_tiles_lymhurst];

    pasture_map_lymhurst.insert(
        baby_chicken_tiles_lymhurst,
        solution_baby_chicken_tiles_lymhurst,
    );
    pasture_map_lymhurst.insert(kid_tiles_lymhurst, solution_kid_tiles_lymhurst);
    pasture_map_lymhurst.insert(gosling_tiles_lymhurst, solution_gosling_tiles_lymhurst);
    pasture_map_lymhurst.insert(lamb_tiles_lymhurst, solution_lamb_tiles_lymhurst);
    pasture_map_lymhurst.insert(piglet_tiles_lymhurst, solution_piglet_tiles_lymhurst);
    pasture_map_lymhurst.insert(calf_tiles_lymhurst, solution_calf_tiles_lymhurst);
    pasture_map_lymhurst.insert(chicken_tiles_lymhurst, solution_chicken_tiles_lymhurst);
    pasture_map_lymhurst.insert(goat_tiles_lymhurst, solution_goat_tiles_lymhurst);
    pasture_map_lymhurst.insert(goose_tiles_lymhurst, solution_goose_tiles_lymhurst);
    pasture_map_lymhurst.insert(sheep_tiles_lymhurst, solution_sheep_tiles_lymhurst);
    pasture_map_lymhurst.insert(pig_tiles_lymhurst, solution_pig_tiles_lymhurst);
    pasture_map_lymhurst.insert(cow_tiles_lymhurst, solution_cow_tiles_lymhurst);

    smart_round(&mut pasture_map_lymhurst);

    solution_baby_chicken_tiles_lymhurst = *pasture_map_lymhurst
        .get(&baby_chicken_tiles_lymhurst)
        .unwrap();
    solution_kid_tiles_lymhurst = *pasture_map_lymhurst.get(&kid_tiles_lymhurst).unwrap();
    solution_gosling_tiles_lymhurst = *pasture_map_lymhurst.get(&gosling_tiles_lymhurst).unwrap();
    solution_lamb_tiles_lymhurst = *pasture_map_lymhurst.get(&lamb_tiles_lymhurst).unwrap();
    solution_piglet_tiles_lymhurst = *pasture_map_lymhurst.get(&piglet_tiles_lymhurst).unwrap();
    solution_calf_tiles_lymhurst = *pasture_map_lymhurst.get(&calf_tiles_lymhurst).unwrap();
    solution_chicken_tiles_lymhurst = *pasture_map_lymhurst.get(&chicken_tiles_lymhurst).unwrap();
    solution_goat_tiles_lymhurst = *pasture_map_lymhurst.get(&goat_tiles_lymhurst).unwrap();
    solution_goose_tiles_lymhurst = *pasture_map_lymhurst.get(&goose_tiles_lymhurst).unwrap();
    solution_sheep_tiles_lymhurst = *pasture_map_lymhurst.get(&sheep_tiles_lymhurst).unwrap();
    solution_pig_tiles_lymhurst = *pasture_map_lymhurst.get(&pig_tiles_lymhurst).unwrap();
    solution_cow_tiles_lymhurst = *pasture_map_lymhurst.get(&cow_tiles_lymhurst).unwrap();

    let mut herb_garden_tile_map_martlock = HashMap::new();
    let mut solution_agaric_tiles_martlock = solution[agaric_tiles_martlock];
    let mut solution_comfrey_tiles_martlock = solution[comfrey_tiles_martlock];
    let mut solution_burdock_tiles_martlock = solution[burdock_tiles_martlock];
    let mut solution_teasel_tiles_martlock = solution[teasel_tiles_martlock];
    let mut solution_foxglove_tiles_martlock = solution[foxglove_tiles_martlock];
    let mut solution_muellin_tiles_martlock = solution[muellin_tiles_martlock];
    let mut solution_yarrow_tiles_martlock = solution[yarrow_tiles_martlock];

    herb_garden_tile_map_martlock.insert(agaric_tiles_martlock, solution_agaric_tiles_martlock);
    herb_garden_tile_map_martlock.insert(comfrey_tiles_martlock, solution_comfrey_tiles_martlock);
    herb_garden_tile_map_martlock.insert(burdock_tiles_martlock, solution_burdock_tiles_martlock);
    herb_garden_tile_map_martlock.insert(teasel_tiles_martlock, solution_teasel_tiles_martlock);
    herb_garden_tile_map_martlock.insert(foxglove_tiles_martlock, solution_foxglove_tiles_martlock);
    herb_garden_tile_map_martlock.insert(muellin_tiles_martlock, solution_muellin_tiles_martlock);
    herb_garden_tile_map_martlock.insert(yarrow_tiles_martlock, solution_yarrow_tiles_martlock);

    smart_round(&mut herb_garden_tile_map_martlock);

    solution_agaric_tiles_martlock = *herb_garden_tile_map_martlock
        .get(&agaric_tiles_martlock)
        .unwrap();
    solution_comfrey_tiles_martlock = *herb_garden_tile_map_martlock
        .get(&comfrey_tiles_martlock)
        .unwrap();
    solution_burdock_tiles_martlock = *herb_garden_tile_map_martlock
        .get(&burdock_tiles_martlock)
        .unwrap();
    solution_teasel_tiles_martlock = *herb_garden_tile_map_martlock
        .get(&teasel_tiles_martlock)
        .unwrap();
    solution_foxglove_tiles_martlock = *herb_garden_tile_map_martlock
        .get(&foxglove_tiles_martlock)
        .unwrap();
    solution_muellin_tiles_martlock = *herb_garden_tile_map_martlock
        .get(&muellin_tiles_martlock)
        .unwrap();
    solution_yarrow_tiles_martlock = *herb_garden_tile_map_martlock
        .get(&yarrow_tiles_martlock)
        .unwrap();

    let mut farm_tile_map_martlock = HashMap::new();
    let mut solution_carrot_tiles_martlock = solution[carrot_tiles_martlock];
    let mut solution_bean_tiles_martlock = solution[bean_tiles_martlock];
    let mut solution_wheat_tiles_martlock = solution[wheat_tiles_martlock];
    let mut solution_turnip_tiles_martlock = solution[turnip_tiles_martlock];
    let mut solution_cabbage_tiles_martlock = solution[cabbage_tiles_martlock];
    let mut solution_potato_tiles_martlock = solution[potato_tiles_martlock];
    let mut solution_corn_tiles_martlock = solution[corn_tiles_martlock];
    let mut solution_pumpkin_tiles_martlock = solution[pumpkin_tiles_martlock];

    farm_tile_map_martlock.insert(carrot_tiles_martlock, solution_carrot_tiles_martlock);
    farm_tile_map_martlock.insert(bean_tiles_martlock, solution_bean_tiles_martlock);
    farm_tile_map_martlock.insert(wheat_tiles_martlock, solution_wheat_tiles_martlock);
    farm_tile_map_martlock.insert(turnip_tiles_martlock, solution_turnip_tiles_martlock);
    farm_tile_map_martlock.insert(cabbage_tiles_martlock, solution_cabbage_tiles_martlock);
    farm_tile_map_martlock.insert(potato_tiles_martlock, solution_potato_tiles_martlock);
    farm_tile_map_martlock.insert(corn_tiles_martlock, solution_corn_tiles_martlock);
    farm_tile_map_martlock.insert(pumpkin_tiles_martlock, solution_pumpkin_tiles_martlock);

    smart_round(&mut farm_tile_map_martlock);

    solution_carrot_tiles_martlock = *farm_tile_map_martlock.get(&carrot_tiles_martlock).unwrap();
    solution_bean_tiles_martlock = *farm_tile_map_martlock.get(&bean_tiles_martlock).unwrap();
    solution_wheat_tiles_martlock = *farm_tile_map_martlock.get(&wheat_tiles_martlock).unwrap();
    solution_turnip_tiles_martlock = *farm_tile_map_martlock.get(&turnip_tiles_martlock).unwrap();
    solution_cabbage_tiles_martlock = *farm_tile_map_martlock.get(&cabbage_tiles_martlock).unwrap();
    solution_potato_tiles_martlock = *farm_tile_map_martlock.get(&potato_tiles_martlock).unwrap();
    solution_corn_tiles_martlock = *farm_tile_map_martlock.get(&corn_tiles_martlock).unwrap();
    solution_pumpkin_tiles_martlock = *farm_tile_map_martlock.get(&pumpkin_tiles_martlock).unwrap();

    let mut pasture_map_martlock = HashMap::new();
    let mut solution_baby_chicken_tiles_martlock = solution[baby_chicken_tiles_martlock];
    let mut solution_kid_tiles_martlock = solution[kid_tiles_martlock];
    let mut solution_gosling_tiles_martlock = solution[gosling_tiles_martlock];
    let mut solution_lamb_tiles_martlock = solution[lamb_tiles_martlock];
    let mut solution_piglet_tiles_martlock = solution[piglet_tiles_martlock];
    let mut solution_calf_tiles_martlock = solution[calf_tiles_martlock];
    let mut solution_chicken_tiles_martlock = solution[chicken_tiles_martlock];
    let mut solution_goat_tiles_martlock = solution[goat_tiles_martlock];
    let mut solution_goose_tiles_martlock = solution[goose_tiles_martlock];
    let mut solution_sheep_tiles_martlock = solution[sheep_tiles_martlock];
    let mut solution_pig_tiles_martlock = solution[pig_tiles_martlock];
    let mut solution_cow_tiles_martlock = solution[cow_tiles_martlock];

    pasture_map_martlock.insert(
        baby_chicken_tiles_martlock,
        solution_baby_chicken_tiles_martlock,
    );
    pasture_map_martlock.insert(kid_tiles_martlock, solution_kid_tiles_martlock);
    pasture_map_martlock.insert(gosling_tiles_martlock, solution_gosling_tiles_martlock);
    pasture_map_martlock.insert(lamb_tiles_martlock, solution_lamb_tiles_martlock);
    pasture_map_martlock.insert(piglet_tiles_martlock, solution_piglet_tiles_martlock);
    pasture_map_martlock.insert(calf_tiles_martlock, solution_calf_tiles_martlock);
    pasture_map_martlock.insert(chicken_tiles_martlock, solution_chicken_tiles_martlock);
    pasture_map_martlock.insert(goat_tiles_martlock, solution_goat_tiles_martlock);
    pasture_map_martlock.insert(goose_tiles_martlock, solution_goose_tiles_martlock);
    pasture_map_martlock.insert(sheep_tiles_martlock, solution_sheep_tiles_martlock);
    pasture_map_martlock.insert(pig_tiles_martlock, solution_pig_tiles_martlock);
    pasture_map_martlock.insert(cow_tiles_martlock, solution_cow_tiles_martlock);

    smart_round(&mut pasture_map_martlock);

    solution_baby_chicken_tiles_martlock = *pasture_map_martlock
        .get(&baby_chicken_tiles_martlock)
        .unwrap();
    solution_kid_tiles_martlock = *pasture_map_martlock.get(&kid_tiles_martlock).unwrap();
    solution_gosling_tiles_martlock = *pasture_map_martlock.get(&gosling_tiles_martlock).unwrap();
    solution_lamb_tiles_martlock = *pasture_map_martlock.get(&lamb_tiles_martlock).unwrap();
    solution_piglet_tiles_martlock = *pasture_map_martlock.get(&piglet_tiles_martlock).unwrap();
    solution_calf_tiles_martlock = *pasture_map_martlock.get(&calf_tiles_martlock).unwrap();
    solution_chicken_tiles_martlock = *pasture_map_martlock.get(&chicken_tiles_martlock).unwrap();
    solution_goat_tiles_martlock = *pasture_map_martlock.get(&goat_tiles_martlock).unwrap();
    solution_goose_tiles_martlock = *pasture_map_martlock.get(&goose_tiles_martlock).unwrap();
    solution_sheep_tiles_martlock = *pasture_map_martlock.get(&sheep_tiles_martlock).unwrap();
    solution_pig_tiles_martlock = *pasture_map_martlock.get(&pig_tiles_martlock).unwrap();
    solution_cow_tiles_martlock = *pasture_map_martlock.get(&cow_tiles_martlock).unwrap();

    let mut herb_garden_tile_map_thetford = HashMap::new();
    let mut solution_agaric_tiles_thetford = solution[agaric_tiles_thetford];
    let mut solution_comfrey_tiles_thetford = solution[comfrey_tiles_thetford];
    let mut solution_burdock_tiles_thetford = solution[burdock_tiles_thetford];
    let mut solution_teasel_tiles_thetford = solution[teasel_tiles_thetford];
    let mut solution_foxglove_tiles_thetford = solution[foxglove_tiles_thetford];
    let mut solution_muellin_tiles_thetford = solution[muellin_tiles_thetford];
    let mut solution_yarrow_tiles_thetford = solution[yarrow_tiles_thetford];

    herb_garden_tile_map_thetford.insert(agaric_tiles_thetford, solution_agaric_tiles_thetford);
    herb_garden_tile_map_thetford.insert(comfrey_tiles_thetford, solution_comfrey_tiles_thetford);
    herb_garden_tile_map_thetford.insert(burdock_tiles_thetford, solution_burdock_tiles_thetford);
    herb_garden_tile_map_thetford.insert(teasel_tiles_thetford, solution_teasel_tiles_thetford);
    herb_garden_tile_map_thetford.insert(foxglove_tiles_thetford, solution_foxglove_tiles_thetford);
    herb_garden_tile_map_thetford.insert(muellin_tiles_thetford, solution_muellin_tiles_thetford);
    herb_garden_tile_map_thetford.insert(yarrow_tiles_thetford, solution_yarrow_tiles_thetford);

    smart_round(&mut herb_garden_tile_map_thetford);

    solution_agaric_tiles_thetford = *herb_garden_tile_map_thetford
        .get(&agaric_tiles_thetford)
        .unwrap();
    solution_comfrey_tiles_thetford = *herb_garden_tile_map_thetford
        .get(&comfrey_tiles_thetford)
        .unwrap();
    solution_burdock_tiles_thetford = *herb_garden_tile_map_thetford
        .get(&burdock_tiles_thetford)
        .unwrap();
    solution_teasel_tiles_thetford = *herb_garden_tile_map_thetford
        .get(&teasel_tiles_thetford)
        .unwrap();
    solution_foxglove_tiles_thetford = *herb_garden_tile_map_thetford
        .get(&foxglove_tiles_thetford)
        .unwrap();
    solution_muellin_tiles_thetford = *herb_garden_tile_map_thetford
        .get(&muellin_tiles_thetford)
        .unwrap();
    solution_yarrow_tiles_thetford = *herb_garden_tile_map_thetford
        .get(&yarrow_tiles_thetford)
        .unwrap();

    let mut farm_tile_map_thetford = HashMap::new();
    let mut solution_carrot_tiles_thetford = solution[carrot_tiles_thetford];
    let mut solution_bean_tiles_thetford = solution[bean_tiles_thetford];
    let mut solution_wheat_tiles_thetford = solution[wheat_tiles_thetford];
    let mut solution_turnip_tiles_thetford = solution[turnip_tiles_thetford];
    let mut solution_cabbage_tiles_thetford = solution[cabbage_tiles_thetford];
    let mut solution_potato_tiles_thetford = solution[potato_tiles_thetford];
    let mut solution_corn_tiles_thetford = solution[corn_tiles_thetford];
    let mut solution_pumpkin_tiles_thetford = solution[pumpkin_tiles_thetford];

    farm_tile_map_thetford.insert(carrot_tiles_thetford, solution_carrot_tiles_thetford);
    farm_tile_map_thetford.insert(bean_tiles_thetford, solution_bean_tiles_thetford);
    farm_tile_map_thetford.insert(wheat_tiles_thetford, solution_wheat_tiles_thetford);
    farm_tile_map_thetford.insert(turnip_tiles_thetford, solution_turnip_tiles_thetford);
    farm_tile_map_thetford.insert(cabbage_tiles_thetford, solution_cabbage_tiles_thetford);
    farm_tile_map_thetford.insert(potato_tiles_thetford, solution_potato_tiles_thetford);
    farm_tile_map_thetford.insert(corn_tiles_thetford, solution_corn_tiles_thetford);
    farm_tile_map_thetford.insert(pumpkin_tiles_thetford, solution_pumpkin_tiles_thetford);

    smart_round(&mut farm_tile_map_thetford);

    solution_carrot_tiles_thetford = *farm_tile_map_thetford.get(&carrot_tiles_thetford).unwrap();
    solution_bean_tiles_thetford = *farm_tile_map_thetford.get(&bean_tiles_thetford).unwrap();
    solution_wheat_tiles_thetford = *farm_tile_map_thetford.get(&wheat_tiles_thetford).unwrap();
    solution_turnip_tiles_thetford = *farm_tile_map_thetford.get(&turnip_tiles_thetford).unwrap();
    solution_cabbage_tiles_thetford = *farm_tile_map_thetford.get(&cabbage_tiles_thetford).unwrap();
    solution_potato_tiles_thetford = *farm_tile_map_thetford.get(&potato_tiles_thetford).unwrap();
    solution_corn_tiles_thetford = *farm_tile_map_thetford.get(&corn_tiles_thetford).unwrap();
    solution_pumpkin_tiles_thetford = *farm_tile_map_thetford.get(&pumpkin_tiles_thetford).unwrap();

    let mut pasture_map_thetford = HashMap::new();
    let mut solution_baby_chicken_tiles_thetford = solution[baby_chicken_tiles_thetford];
    let mut solution_kid_tiles_thetford = solution[kid_tiles_thetford];
    let mut solution_gosling_tiles_thetford = solution[gosling_tiles_thetford];
    let mut solution_lamb_tiles_thetford = solution[lamb_tiles_thetford];
    let mut solution_piglet_tiles_thetford = solution[piglet_tiles_thetford];
    let mut solution_calf_tiles_thetford = solution[calf_tiles_thetford];
    let mut solution_chicken_tiles_thetford = solution[chicken_tiles_thetford];
    let mut solution_goat_tiles_thetford = solution[goat_tiles_thetford];
    let mut solution_goose_tiles_thetford = solution[goose_tiles_thetford];
    let mut solution_sheep_tiles_thetford = solution[sheep_tiles_thetford];
    let mut solution_pig_tiles_thetford = solution[pig_tiles_thetford];
    let mut solution_cow_tiles_thetford = solution[cow_tiles_thetford];

    pasture_map_thetford.insert(
        baby_chicken_tiles_thetford,
        solution_baby_chicken_tiles_thetford,
    );
    pasture_map_thetford.insert(kid_tiles_thetford, solution_kid_tiles_thetford);
    pasture_map_thetford.insert(gosling_tiles_thetford, solution_gosling_tiles_thetford);
    pasture_map_thetford.insert(lamb_tiles_thetford, solution_lamb_tiles_thetford);
    pasture_map_thetford.insert(piglet_tiles_thetford, solution_piglet_tiles_thetford);
    pasture_map_thetford.insert(calf_tiles_thetford, solution_calf_tiles_thetford);
    pasture_map_thetford.insert(chicken_tiles_thetford, solution_chicken_tiles_thetford);
    pasture_map_thetford.insert(goat_tiles_thetford, solution_goat_tiles_thetford);
    pasture_map_thetford.insert(goose_tiles_thetford, solution_goose_tiles_thetford);
    pasture_map_thetford.insert(sheep_tiles_thetford, solution_sheep_tiles_thetford);
    pasture_map_thetford.insert(pig_tiles_thetford, solution_pig_tiles_thetford);
    pasture_map_thetford.insert(cow_tiles_thetford, solution_cow_tiles_thetford);

    smart_round(&mut pasture_map_thetford);

    solution_baby_chicken_tiles_thetford = *pasture_map_thetford
        .get(&baby_chicken_tiles_thetford)
        .unwrap();
    solution_kid_tiles_thetford = *pasture_map_thetford.get(&kid_tiles_thetford).unwrap();
    solution_gosling_tiles_thetford = *pasture_map_thetford.get(&gosling_tiles_thetford).unwrap();
    solution_lamb_tiles_thetford = *pasture_map_thetford.get(&lamb_tiles_thetford).unwrap();
    solution_piglet_tiles_thetford = *pasture_map_thetford.get(&piglet_tiles_thetford).unwrap();
    solution_calf_tiles_thetford = *pasture_map_thetford.get(&calf_tiles_thetford).unwrap();
    solution_chicken_tiles_thetford = *pasture_map_thetford.get(&chicken_tiles_thetford).unwrap();
    solution_goat_tiles_thetford = *pasture_map_thetford.get(&goat_tiles_thetford).unwrap();
    solution_goose_tiles_thetford = *pasture_map_thetford.get(&goose_tiles_thetford).unwrap();
    solution_sheep_tiles_thetford = *pasture_map_thetford.get(&sheep_tiles_thetford).unwrap();
    solution_pig_tiles_thetford = *pasture_map_thetford.get(&pig_tiles_thetford).unwrap();
    solution_cow_tiles_thetford = *pasture_map_thetford.get(&cow_tiles_thetford).unwrap();

    let plot_plan = PlotPlan {
        output: solution.objective().round(),

        herb_gardens_brecilien: solution[herb_gardens_brecilien],
        farms_brecilien: solution[farms_brecilien],
        pastures_brecilien: solution[pastures_brecilien],
        herb_gardens_bridgewatch: solution[herb_gardens_bridgewatch],
        farms_bridgewatch: solution[farms_bridgewatch],
        pastures_bridgewatch: solution[pastures_bridgewatch],
        herb_gardens_caerleon: solution[herb_gardens_caerleon],
        farms_caerleon: solution[farms_caerleon],
        pastures_caerleon: solution[pastures_caerleon],
        herb_gardens_fort_sterling: solution[herb_gardens_fort_sterling],
        farms_fort_sterling: solution[farms_fort_sterling],
        pastures_fort_sterling: solution[pastures_fort_sterling],
        herb_gardens_lymhurst: solution[herb_gardens_lymhurst],
        farms_lymhurst: solution[farms_lymhurst],
        pastures_lymhurst: solution[pastures_lymhurst],
        herb_gardens_martlock: solution[herb_gardens_martlock],
        farms_martlock: solution[farms_martlock],
        pastures_martlock: solution[pastures_martlock],
        herb_gardens_thetford: solution[herb_gardens_thetford],
        farms_thetford: solution[farms_thetford],
        pastures_thetford: solution[pastures_thetford],

        agaric_tiles_brecilien: solution_agaric_tiles_brecilien,
        comfrey_tiles_brecilien: solution_comfrey_tiles_brecilien,
        burdock_tiles_brecilien: solution_burdock_tiles_brecilien,
        teasel_tiles_brecilien: solution_teasel_tiles_brecilien,
        foxglove_tiles_brecilien: solution_foxglove_tiles_brecilien,
        muellin_tiles_brecilien: solution_muellin_tiles_brecilien,
        yarrow_tiles_brecilien: solution_yarrow_tiles_brecilien,
        carrot_tiles_brecilien: solution_carrot_tiles_brecilien,
        bean_tiles_brecilien: solution_bean_tiles_brecilien,
        wheat_tiles_brecilien: solution_wheat_tiles_brecilien,
        turnip_tiles_brecilien: solution_turnip_tiles_brecilien,
        cabbage_tiles_brecilien: solution_cabbage_tiles_brecilien,
        potato_tiles_brecilien: solution_potato_tiles_brecilien,
        corn_tiles_brecilien: solution_corn_tiles_brecilien,
        pumpkin_tiles_brecilien: solution_pumpkin_tiles_brecilien,
        baby_chicken_tiles_brecilien: solution_baby_chicken_tiles_brecilien,
        kid_tiles_brecilien: solution_kid_tiles_brecilien,
        gosling_tiles_brecilien: solution_gosling_tiles_brecilien,
        lamb_tiles_brecilien: solution_lamb_tiles_brecilien,
        piglet_tiles_brecilien: solution_piglet_tiles_brecilien,
        calf_tiles_brecilien: solution_calf_tiles_brecilien,
        chicken_tiles_brecilien: solution_chicken_tiles_brecilien,
        goat_tiles_brecilien: solution_goat_tiles_brecilien,
        goose_tiles_brecilien: solution_goose_tiles_brecilien,
        sheep_tiles_brecilien: solution_sheep_tiles_brecilien,
        pig_tiles_brecilien: solution_pig_tiles_brecilien,
        cow_tiles_brecilien: solution_cow_tiles_brecilien,
        agaric_tiles_bridgewatch: solution_agaric_tiles_bridgewatch,
        comfrey_tiles_bridgewatch: solution_comfrey_tiles_bridgewatch,
        burdock_tiles_bridgewatch: solution_burdock_tiles_bridgewatch,
        teasel_tiles_bridgewatch: solution_teasel_tiles_bridgewatch,
        foxglove_tiles_bridgewatch: solution_foxglove_tiles_bridgewatch,
        muellin_tiles_bridgewatch: solution_muellin_tiles_bridgewatch,
        yarrow_tiles_bridgewatch: solution_yarrow_tiles_bridgewatch,
        carrot_tiles_bridgewatch: solution_carrot_tiles_bridgewatch,
        bean_tiles_bridgewatch: solution_bean_tiles_bridgewatch,
        wheat_tiles_bridgewatch: solution_wheat_tiles_bridgewatch,
        turnip_tiles_bridgewatch: solution_turnip_tiles_bridgewatch,
        cabbage_tiles_bridgewatch: solution_cabbage_tiles_bridgewatch,
        potato_tiles_bridgewatch: solution_potato_tiles_bridgewatch,
        corn_tiles_bridgewatch: solution_corn_tiles_bridgewatch,
        pumpkin_tiles_bridgewatch: solution_pumpkin_tiles_bridgewatch,
        baby_chicken_tiles_bridgewatch: solution_baby_chicken_tiles_bridgewatch,
        kid_tiles_bridgewatch: solution_kid_tiles_bridgewatch,
        gosling_tiles_bridgewatch: solution_gosling_tiles_bridgewatch,
        lamb_tiles_bridgewatch: solution_lamb_tiles_bridgewatch,
        piglet_tiles_bridgewatch: solution_piglet_tiles_bridgewatch,
        calf_tiles_bridgewatch: solution_calf_tiles_bridgewatch,
        chicken_tiles_bridgewatch: solution_chicken_tiles_bridgewatch,
        goat_tiles_bridgewatch: solution_goat_tiles_bridgewatch,
        goose_tiles_bridgewatch: solution_goose_tiles_bridgewatch,
        sheep_tiles_bridgewatch: solution_sheep_tiles_bridgewatch,
        pig_tiles_bridgewatch: solution_pig_tiles_bridgewatch,
        cow_tiles_bridgewatch: solution_cow_tiles_bridgewatch,
        agaric_tiles_caerleon: solution_agaric_tiles_caerleon,
        comfrey_tiles_caerleon: solution_comfrey_tiles_caerleon,
        burdock_tiles_caerleon: solution_burdock_tiles_caerleon,
        teasel_tiles_caerleon: solution_teasel_tiles_caerleon,
        foxglove_tiles_caerleon: solution_foxglove_tiles_caerleon,
        muellin_tiles_caerleon: solution_muellin_tiles_caerleon,
        yarrow_tiles_caerleon: solution_yarrow_tiles_caerleon,
        carrot_tiles_caerleon: solution_carrot_tiles_caerleon,
        bean_tiles_caerleon: solution_bean_tiles_caerleon,
        wheat_tiles_caerleon: solution_wheat_tiles_caerleon,
        turnip_tiles_caerleon: solution_turnip_tiles_caerleon,
        cabbage_tiles_caerleon: solution_cabbage_tiles_caerleon,
        potato_tiles_caerleon: solution_potato_tiles_caerleon,
        corn_tiles_caerleon: solution_corn_tiles_caerleon,
        pumpkin_tiles_caerleon: solution_pumpkin_tiles_caerleon,
        baby_chicken_tiles_caerleon: solution_baby_chicken_tiles_caerleon,
        kid_tiles_caerleon: solution_kid_tiles_caerleon,
        gosling_tiles_caerleon: solution_gosling_tiles_caerleon,
        lamb_tiles_caerleon: solution_lamb_tiles_caerleon,
        piglet_tiles_caerleon: solution_piglet_tiles_caerleon,
        calf_tiles_caerleon: solution_calf_tiles_caerleon,
        chicken_tiles_caerleon: solution_chicken_tiles_caerleon,
        goat_tiles_caerleon: solution_goat_tiles_caerleon,
        goose_tiles_caerleon: solution_goose_tiles_caerleon,
        sheep_tiles_caerleon: solution_sheep_tiles_caerleon,
        pig_tiles_caerleon: solution_pig_tiles_caerleon,
        cow_tiles_caerleon: solution_cow_tiles_caerleon,
        agaric_tiles_fort_sterling: solution_agaric_tiles_fort_sterling,
        comfrey_tiles_fort_sterling: solution_comfrey_tiles_fort_sterling,
        burdock_tiles_fort_sterling: solution_burdock_tiles_fort_sterling,
        teasel_tiles_fort_sterling: solution_teasel_tiles_fort_sterling,
        foxglove_tiles_fort_sterling: solution_foxglove_tiles_fort_sterling,
        muellin_tiles_fort_sterling: solution_muellin_tiles_fort_sterling,
        yarrow_tiles_fort_sterling: solution_yarrow_tiles_fort_sterling,
        carrot_tiles_fort_sterling: solution_carrot_tiles_fort_sterling,
        bean_tiles_fort_sterling: solution_bean_tiles_fort_sterling,
        wheat_tiles_fort_sterling: solution_wheat_tiles_fort_sterling,
        turnip_tiles_fort_sterling: solution_turnip_tiles_fort_sterling,
        cabbage_tiles_fort_sterling: solution_cabbage_tiles_fort_sterling,
        potato_tiles_fort_sterling: solution_potato_tiles_fort_sterling,
        corn_tiles_fort_sterling: solution_corn_tiles_fort_sterling,
        pumpkin_tiles_fort_sterling: solution_pumpkin_tiles_fort_sterling,
        baby_chicken_tiles_fort_sterling: solution_baby_chicken_tiles_fort_sterling,
        kid_tiles_fort_sterling: solution_kid_tiles_fort_sterling,
        gosling_tiles_fort_sterling: solution_gosling_tiles_fort_sterling,
        lamb_tiles_fort_sterling: solution_lamb_tiles_fort_sterling,
        piglet_tiles_fort_sterling: solution_piglet_tiles_fort_sterling,
        calf_tiles_fort_sterling: solution_calf_tiles_fort_sterling,
        chicken_tiles_fort_sterling: solution_chicken_tiles_fort_sterling,
        goat_tiles_fort_sterling: solution_goat_tiles_fort_sterling,
        goose_tiles_fort_sterling: solution_goose_tiles_fort_sterling,
        sheep_tiles_fort_sterling: solution_sheep_tiles_fort_sterling,
        pig_tiles_fort_sterling: solution_pig_tiles_fort_sterling,
        cow_tiles_fort_sterling: solution_cow_tiles_fort_sterling,
        agaric_tiles_lymhurst: solution_agaric_tiles_lymhurst,
        comfrey_tiles_lymhurst: solution_comfrey_tiles_lymhurst,
        burdock_tiles_lymhurst: solution_burdock_tiles_lymhurst,
        teasel_tiles_lymhurst: solution_teasel_tiles_lymhurst,
        foxglove_tiles_lymhurst: solution_foxglove_tiles_lymhurst,
        muellin_tiles_lymhurst: solution_muellin_tiles_lymhurst,
        yarrow_tiles_lymhurst: solution_yarrow_tiles_lymhurst,
        carrot_tiles_lymhurst: solution_carrot_tiles_lymhurst,
        bean_tiles_lymhurst: solution_bean_tiles_lymhurst,
        wheat_tiles_lymhurst: solution_wheat_tiles_lymhurst,
        turnip_tiles_lymhurst: solution_turnip_tiles_lymhurst,
        cabbage_tiles_lymhurst: solution_cabbage_tiles_lymhurst,
        potato_tiles_lymhurst: solution_potato_tiles_lymhurst,
        corn_tiles_lymhurst: solution_corn_tiles_lymhurst,
        pumpkin_tiles_lymhurst: solution_pumpkin_tiles_lymhurst,
        baby_chicken_tiles_lymhurst: solution_baby_chicken_tiles_lymhurst,
        kid_tiles_lymhurst: solution_kid_tiles_lymhurst,
        gosling_tiles_lymhurst: solution_gosling_tiles_lymhurst,
        lamb_tiles_lymhurst: solution_lamb_tiles_lymhurst,
        piglet_tiles_lymhurst: solution_piglet_tiles_lymhurst,
        calf_tiles_lymhurst: solution_calf_tiles_lymhurst,
        chicken_tiles_lymhurst: solution_chicken_tiles_lymhurst,
        goat_tiles_lymhurst: solution_goat_tiles_lymhurst,
        goose_tiles_lymhurst: solution_goose_tiles_lymhurst,
        sheep_tiles_lymhurst: solution_sheep_tiles_lymhurst,
        pig_tiles_lymhurst: solution_pig_tiles_lymhurst,
        cow_tiles_lymhurst: solution_cow_tiles_lymhurst,
        agaric_tiles_martlock: solution_agaric_tiles_martlock,
        comfrey_tiles_martlock: solution_comfrey_tiles_martlock,
        burdock_tiles_martlock: solution_burdock_tiles_martlock,
        teasel_tiles_martlock: solution_teasel_tiles_martlock,
        foxglove_tiles_martlock: solution_foxglove_tiles_martlock,
        muellin_tiles_martlock: solution_muellin_tiles_martlock,
        yarrow_tiles_martlock: solution_yarrow_tiles_martlock,
        carrot_tiles_martlock: solution_carrot_tiles_martlock,
        bean_tiles_martlock: solution_bean_tiles_martlock,
        wheat_tiles_martlock: solution_wheat_tiles_martlock,
        turnip_tiles_martlock: solution_turnip_tiles_martlock,
        cabbage_tiles_martlock: solution_cabbage_tiles_martlock,
        potato_tiles_martlock: solution_potato_tiles_martlock,
        corn_tiles_martlock: solution_corn_tiles_martlock,
        pumpkin_tiles_martlock: solution_pumpkin_tiles_martlock,
        baby_chicken_tiles_martlock: solution_baby_chicken_tiles_martlock,
        kid_tiles_martlock: solution_kid_tiles_martlock,
        gosling_tiles_martlock: solution_gosling_tiles_martlock,
        lamb_tiles_martlock: solution_lamb_tiles_martlock,
        piglet_tiles_martlock: solution_piglet_tiles_martlock,
        calf_tiles_martlock: solution_calf_tiles_martlock,
        chicken_tiles_martlock: solution_chicken_tiles_martlock,
        goat_tiles_martlock: solution_goat_tiles_martlock,
        goose_tiles_martlock: solution_goose_tiles_martlock,
        sheep_tiles_martlock: solution_sheep_tiles_martlock,
        pig_tiles_martlock: solution_pig_tiles_martlock,
        cow_tiles_martlock: solution_cow_tiles_martlock,
        agaric_tiles_thetford: solution_agaric_tiles_thetford,
        comfrey_tiles_thetford: solution_comfrey_tiles_thetford,
        burdock_tiles_thetford: solution_burdock_tiles_thetford,
        teasel_tiles_thetford: solution_teasel_tiles_thetford,
        foxglove_tiles_thetford: solution_foxglove_tiles_thetford,
        muellin_tiles_thetford: solution_muellin_tiles_thetford,
        yarrow_tiles_thetford: solution_yarrow_tiles_thetford,
        carrot_tiles_thetford: solution_carrot_tiles_thetford,
        bean_tiles_thetford: solution_bean_tiles_thetford,
        wheat_tiles_thetford: solution_wheat_tiles_thetford,
        turnip_tiles_thetford: solution_turnip_tiles_thetford,
        cabbage_tiles_thetford: solution_cabbage_tiles_thetford,
        potato_tiles_thetford: solution_potato_tiles_thetford,
        corn_tiles_thetford: solution_corn_tiles_thetford,
        pumpkin_tiles_thetford: solution_pumpkin_tiles_thetford,
        baby_chicken_tiles_thetford: solution_baby_chicken_tiles_thetford,
        kid_tiles_thetford: solution_kid_tiles_thetford,
        gosling_tiles_thetford: solution_gosling_tiles_thetford,
        lamb_tiles_thetford: solution_lamb_tiles_thetford,
        piglet_tiles_thetford: solution_piglet_tiles_thetford,
        calf_tiles_thetford: solution_calf_tiles_thetford,
        chicken_tiles_thetford: solution_chicken_tiles_thetford,
        goat_tiles_thetford: solution_goat_tiles_thetford,
        goose_tiles_thetford: solution_goose_tiles_thetford,
        sheep_tiles_thetford: solution_sheep_tiles_thetford,
        pig_tiles_thetford: solution_pig_tiles_thetford,
        cow_tiles_thetford: solution_cow_tiles_thetford,
    };

    Ok(plot_plan)
}

fn smart_round(plot_map: &mut HashMap<Variable, f64>) {
    clip_near_zero(plot_map);
    let final_sum: f64 = plot_map.values().sum();
    while !done_rounding(plot_map) {
        let current_sum: f64 = plot_map.values().sum();
        if current_sum > final_sum {
            smart_round_best_floor(plot_map);
        } else if current_sum < final_sum {
            smart_round_best_ceil(plot_map);
        } else {
            smart_round_best_round(plot_map);
        }
    }
    let mut current_sum: f64 = plot_map.values().sum();
    while current_sum > final_sum {
        smart_round_reduce_max(plot_map);
        current_sum = plot_map.values().sum();
    }
}

fn smart_round_reduce_max(plot_map: &mut HashMap<Variable, f64>) {
    let mut max_val = -f64::INFINITY;
    let mut max_var: Option<Variable> = None;
    for (variable, value) in plot_map.iter_mut() {
        if *value > max_val {
            max_val = *value;
            max_var = Some(*variable);
        }
    }
    plot_map.insert(max_var.unwrap(), max_val - 1.0);
}

fn smart_round_best_ceil(plot_map: &mut HashMap<Variable, f64>) {
    let mut min_error = f64::INFINITY;
    let mut min_var: Option<Variable> = None;
    for (variable, value) in plot_map.iter_mut() {
        if value.ceil() == *value {
            continue;
        }
        let e = (value.ceil() - *value).abs();
        if e < min_error {
            min_error = e;
            min_var = Some(*variable);
        }
    }
    let new_value = plot_map.get(&min_var.unwrap()).unwrap().ceil();
    plot_map.insert(min_var.unwrap(), new_value.max(1.0));
}

fn smart_round_best_floor(plot_map: &mut HashMap<Variable, f64>) {
    let mut min_error = f64::INFINITY;
    let mut min_var: Option<Variable> = None;
    for (variable, value) in plot_map.iter_mut() {
        if value.floor() == *value {
            continue;
        }
        let e = (value.floor() - *value).abs();
        if e < min_error {
            min_error = e;
            min_var = Some(*variable);
        }
    }
    let new_value = plot_map.get(&min_var.unwrap()).unwrap().floor();
    plot_map.insert(min_var.unwrap(), new_value.max(1.0));
}

fn smart_round_best_round(plot_map: &mut HashMap<Variable, f64>) {
    let mut min_error = f64::INFINITY;
    let mut min_var: Option<Variable> = None;
    for (variable, value) in plot_map.iter_mut() {
        if value.round() == *value {
            continue;
        }
        let e = (value.round() - *value).abs();
        if e < min_error {
            min_error = e;
            min_var = Some(*variable);
        }
    }
    let new_value = plot_map.get(&min_var.unwrap()).unwrap().round();
    plot_map.insert(min_var.unwrap(), new_value.max(1.0));
}

fn clip_near_zero(plot_map: &mut HashMap<Variable, f64>) {
    let mut update_map = HashMap::new();
    for (variable, value) in plot_map.iter_mut() {
        if *value <= ALMOST_ZERO {
            update_map.insert(*variable, 0.0);
        }
    }
    for (variable, value) in update_map.iter_mut() {
        plot_map.insert(*variable, *value);
    }
}

fn done_rounding(plot_map: &mut HashMap<Variable, f64>) -> bool {
    for (_, value) in plot_map.iter_mut() {
        if *value != value.round() {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minor_energy_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::MinorEnergyPotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 5751.0);
    }

    #[test]
    fn minor_healing_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::MinorHealingPotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 5751.0);
    }

    #[test]
    fn minor_gigantify_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::MinorGigantifyPotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 5751.0);
    }

    #[test]
    fn minor_resistance_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::MinorResistancePotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 5751.0);
    }

    #[test]
    fn minor_sticky_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::MinorStickyPotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 5751.0);
    }

    #[test]
    fn minor_poison_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::MinorPoisonPotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 3888.0);
    }

    #[test]
    fn energy_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::EnergyPotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 1596.0);
    }

    #[test]
    fn healing_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::HealingPotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 1596.0);
    }

    #[test]
    fn gigantify_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::GigantifyPotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 1089.0);
    }

    #[test]
    fn resistance_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::ResistancePotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 1089.0);
    }

    #[test]
    fn sticky_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::StickyPotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 1089.0);
    }

    #[test]
    fn poison_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::PoisonPotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 825.0);
    }

    #[test]
    fn major_energy_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::MajorEnergyPotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 439.0);
    }

    #[test]
    fn major_healing_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::MajorHealingPotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 436.0);
    }

    #[test]
    fn major_gigantify_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::MajorGigantifyPotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 330.0);
    }

    #[test]
    fn major_resistance_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::MajorResistancePotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 273.0);
    }

    #[test]
    fn major_sticky_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::MajorStickyPotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 271.0);
    }

    #[test]
    fn major_poison_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::MajorPoisonPotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 263.0);
    }

    #[test]
    fn invisibility_potion_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::InvisibilityPotion,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 263.0);
    }

    #[test]
    fn chicken_omelette_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::ChickenOmelette,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 7261.0);
    }

    #[test]
    fn goose_omelette_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::GooseOmelette,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 2420.0);
    }

    #[test]
    fn pork_omelette_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::PorkOmelette,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 815.0);
    }

    #[test]
    fn bean_salad_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::BeanSalad,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 5913.0);
    }

    #[test]
    fn turnip_salad_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::TurnipSalad,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 1944.0);
    }

    #[test]
    fn potato_salad_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::PotatoSalad,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 648.0);
    }

    #[test]
    fn goat_sandwich_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::GoatSandwich,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 7308.0);
    }

    #[test]
    fn mutton_sandwich_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::MuttonSandwich,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 2420.0);
    }

    #[test]
    fn beef_sandwich_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::BeefSandwich,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 807.0);
    }

    #[test]
    fn carrot_soup_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::CarrotSoup,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 5832.0);
    }

    #[test]
    fn wheat_soup_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::WheatSoup,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 1917.0);
    }

    #[test]
    fn cabbage_soup_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::CabbageSoup,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 639.0);
    }

    #[test]
    fn goat_stew_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::GoatStew,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 6210.0);
    }

    #[test]
    fn mutton_stew_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::MuttonStew,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 2070.0);
    }

    #[test]
    fn beef_stew_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::BeefStew,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 690.0);
    }

    #[test]
    fn roast_chicken_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::RoastChicken,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 6585.0);
    }

    #[test]
    fn roast_goose_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::RoastGoose,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 2197.0);
    }

    #[test]
    fn roast_pork_test() {
        let context = ModelContext {
            brecilien_plots: 16 as f64,
            bridgewatch_plots: 16 as f64,
            caerleon_plots: 16 as f64,
            fort_sterling_plots: 16 as f64,
            lymhurst_plots: 16 as f64,
            martlock_plots: 16 as f64,
            thetford_plots: 16 as f64,
            premium_factor: PREMIUM_FACTOR,
            target: Product::RoastPork,
        };
        let plot_plan = optimize_plots(context);
        assert_eq!(plot_plan.output, 732.0);
    }
}

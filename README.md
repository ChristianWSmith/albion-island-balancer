# albion-island-balancer
Tool for optimizing island crafting output in Albion Online

## Install (for non-technical users)
1. Click on "Releases"
2. Download the latest release for your platform (windows/linux x86_64/i686, sorry mac)
3. Extract the executable from the zip
4. Run the executable from your terminal / command prompt*

*You may have to edit the permissions of the file to make it executable.

## Build
Requirements: git, cargo
```
git clone https://www.github.com/ChristianWSmith/albion-island-balancer
cd albion-island-balancer
cargo build --release
cp target/release/albion-island-balancer .
./albion-island-balancer --help
```

## Example
We'll start by finding out how to set up 3 fully-upgraded islands in Lymhurst for optimal Invisibility Potion production.  We'll run:
```
albion-island-balancer --lymhurst-plots 48 --target invisibility-potion --premium-status premium
```
You should see:
```
Brecilien Plots: 0
Bridgewatch Plots: 0
Caerleon Plots: 0
Fort Sterling Plots: 0
Lymhurst Plots: 48
Martlock Plots: 0
Thetford Plots: 0
Premium Status: Premium
Target: InvisibilityPotion
PlotPlan {
     output: 100.0,
     herb_gardens_lymhurst: 39.0,
     farms_lymhurst: 6.0,
     pastures_lymhurst: 2.0,
     teasel_tiles_lymhurst: 80.0,
     muellin_tiles_lymhurst: 80.0,
     yarrow_tiles_lymhurst: 160.0,
     pumpkin_tiles_lymhurst: 53.0,
     cow_tiles_lymhurst: 18.0 
}
```
This means we should expect to produce ~100 Invisibility Potions each day by configuring our islands as described.  The algorithm prescribes 39 herb gardens, 7 farm patches, and 2 pastures.  Note that this is 47 and not 48.  In this case, the algorithm determined that it couldn't increase throughput by using all 48 plots instead of 47, therefore it decided to "refund" us one plot.

## Usage
Run:
```
albion-island-balancer --help
```
You should see:
```
Optimize your plots in Albion Online

Usage: albion-island-balancer [OPTIONS]

Options:
      --brecilien-plots <num_plots>
          Number of plots in Brecilien [default: 0]
      --bridgewatch-plots <num_plots>
          Number of plots in Bridgewatch [default: 0]
      --caerleon-plots <num_plots>
          Number of plots in Caerleon [default: 0]
      --fort-sterling-plots <num_plots>
          Number of plots in Fort Sterling [default: 0]
      --lymhurst-plots <num_plots>
          Number of plots in Lymhurst [default: 0]
      --martlock-plots <num_plots>
          Number of plots in Martlock [default: 0]
      --thetford-plots <num_plots>
          Number of plots in Thetford [default: 0]
      --premium-status <status>
          Premium status [default: premium] [possible values: premium, free]
      --target <product>
          Target product [default: major-poison-potion] [possible values: minor-energy-potion, minor-healing-potion, minor-gigantify-potion, minor-resistance-potion, minor-sticky-potion, minor-poison-potion, energy-potion, healing-potion, gigantify-potion, resistance-potion, sticky-potion, poison-potion, major-energy-potion, major-healing-potion, major-gigantify-potion, major-resistance-potion, major-sticky-potion, major-poison-potion, invisibility-potion, chicken-omelette, goose-omelette, pork-omelette, bean-salad, turnip-salad, potato-salad, goat-sandwich, mutton-sandwich, beef-sandwich, carrot-soup, wheat-soup, cabbage-soup, goat-stew, mutton-stew, beef-stew, roast-chicken, roast-goose, roast-pork]
  -h, --help
          Print help
  -V, --version
          Print version
```
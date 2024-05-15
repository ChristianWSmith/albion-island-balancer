use linprog::{
    Model,
    Objective,
    Summand,
    Operator,
    Var
};
use clap::{Arg, Command, ValueEnum};


#[derive(ValueEnum, Clone, Debug)]
enum Product {
    Option1,
    Option2,
    Option3,
}

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Optimize your plots in Albion Online")
        .arg(
            Arg::new("brecilien-plots")
                .long("brecilien-plots")
                .num_args(1)
                .value_name("num_plots")
                .help("Number of plots in Brecilien (optional)"),
        )
        .arg(
            Arg::new("bridgewatch-plots")
                .long("bridgewatch-plots")
                .num_args(1)
                .value_name("num_plots")
                .help("Number of plots in Bridgewatch (optional)"),
        )
        .arg(
            Arg::new("caerleon-plots")
                .long("caerleon-plots")
                .num_args(1)
                .value_name("num_plots")
                .help("Number of plots in Caerleon (optional)"),
        )
        .arg(
            Arg::new("fort-sterling-plots")
                .long("fort-sterling-plots")
                .num_args(1)
                .value_name("num_plots")
                .help("Number of plots in Fort Sterling (optional)"),
        )
        .arg(
            Arg::new("lymhurst-plots")
                .long("lymhurst-plots")
                .num_args(1)
                .value_name("num_plots")
                .help("Number of plots in Lymhurst (optional)"),
        )
        .arg(
            Arg::new("martlock-plots")
                .long("martlock-plots")
                .num_args(1)
                .value_name("num_plots")
                .help("Number of plots in Martlock (optional)"),
        )
        .arg(
            Arg::new("thetford-plots")
                .long("thetford-plots")
                .num_args(1)
                .value_name("num_plots")
                .help("Number of plots in Thetford (optional)"),
        )
        .arg(
            Arg::new("target")
                .long("target")
                .num_args(1)
                .value_name("product")
                .value_parser(clap::builder::EnumValueParser::<Product>::new())
                .required(true)
                .help("Target product (required)"),
        )
        .get_matches();

    // Get the values of the arguments
    let brecilien_plots = matches.get_one::<String>("brecilien-plots").map(|s| s.parse::<u32>().expect("num_plots must be a positive integer"));
    let bridgewatch_plots = matches.get_one::<String>("bridgewatch-plots").map(|s| s.parse::<u32>().expect("num_plots must be a positive integer"));
    let caerleon_plots = matches.get_one::<String>("caerleon-plots").map(|s| s.parse::<u32>().expect("num_plots must be a positive integer"));
    let fort_sterling_plots = matches.get_one::<String>("fort-sterling-plots").map(|s| s.parse::<u32>().expect("num_plots must be a positive integer"));
    let lymhurst_plots = matches.get_one::<String>("lymhurst-plots").map(|s| s.parse::<u32>().expect("num_plots must be a positive integer"));
    let martlock_plots = matches.get_one::<String>("martlock-plots").map(|s| s.parse::<u32>().expect("num_plots must be a positive integer"));
    let thetford_plots = matches.get_one::<String>("thetford-plots").map(|s| s.parse::<u32>().expect("num_plots must be a positive integer"));
    let target = matches.get_one::<Product>("target").expect("target is required");

    // Use the arguments
    println!("Brecilien Plots: {:?}", brecilien_plots);
    println!("Bridgewatch Plots: {:?}", bridgewatch_plots);
    println!("Caerleon Plots: {:?}", caerleon_plots);
    println!("Fort Sterling Plots: {:?}", fort_sterling_plots);
    println!("Lymhurst Plots: {:?}", lymhurst_plots);
    println!("Martlock Plots: {:?}", martlock_plots);
    println!("Thetford Plots: {:?}", thetford_plots);
    println!("Target: {:?}", target);

    let mut model = Model::new("My LP", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    vars.push(model.reg_var(2.0));
    model.reg_constr(vec![Summand(1.0, &vars[0])], Operator::Le, 10.0);
    model.optimize();
    println!("{}", model);
}

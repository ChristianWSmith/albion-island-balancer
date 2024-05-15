use linprog::{
    Model,
    Objective,
    Summand,
    Operator,
    Var
};
use clap::{Arg, Command};


fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Optimize your plots in Albion Online")
        .arg(
            Arg::new("plots")
                .long("plots")
                .num_args(1)
                .value_name("uint")
                .help("Number of plots (optional)"),
        )
        .arg(
            Arg::new("target")
                .long("target")
                .num_args(1)
                .value_name("str")
                .required(true)
                .help("Target recipe (required)"),
        )
        .get_matches();

    // Get the values of the arguments
    let plots = matches.get_one::<String>("plots").map(|s| s.parse::<u32>().expect("plots must be a positive integer"));
    let target = matches.get_one::<String>("target").expect("target is required");

    // Use the arguments
    println!("Plots: {:?}", plots);
    println!("Target: {}", target);

    let mut model = Model::new("My LP", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    vars.push(model.reg_var(2.0));
    model.reg_constr(vec![Summand(1.0, &vars[0])], Operator::Le, 10.0);
    model.optimize();
    println!("{}", model);
}

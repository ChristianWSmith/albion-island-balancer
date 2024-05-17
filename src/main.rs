mod constants;
mod model;
mod types;
use clap::{Arg, Command};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

use crate::{
    constants::{FREE_FACTOR, PREMIUM_FACTOR}, model::optimize_plots, types::{ModelContext, PremiumStatus, Product}
};

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
                .default_value("0")
                .help("Number of plots in Brecilien"),
        )
        .arg(
            Arg::new("bridgewatch-plots")
                .long("bridgewatch-plots")
                .num_args(1)
                .value_name("num_plots")
                .default_value("0")
                .help("Number of plots in Bridgewatch"),
        )
        .arg(
            Arg::new("caerleon-plots")
                .long("caerleon-plots")
                .num_args(1)
                .value_name("num_plots")
                .default_value("0")
                .help("Number of plots in Caerleon"),
        )
        .arg(
            Arg::new("fort-sterling-plots")
                .long("fort-sterling-plots")
                .num_args(1)
                .value_name("num_plots")
                .default_value("0")
                .help("Number of plots in Fort Sterling"),
        )
        .arg(
            Arg::new("lymhurst-plots")
                .long("lymhurst-plots")
                .num_args(1)
                .value_name("num_plots")
                .default_value("0")
                .help("Number of plots in Lymhurst"),
        )
        .arg(
            Arg::new("martlock-plots")
                .long("martlock-plots")
                .num_args(1)
                .value_name("num_plots")
                .default_value("0")
                .help("Number of plots in Martlock"),
        )
        .arg(
            Arg::new("thetford-plots")
                .long("thetford-plots")
                .num_args(1)
                .value_name("num_plots")
                .default_value("0")
                .help("Number of plots in Thetford"),
        )
        .arg(
            Arg::new("premium-status")
                .long("premium-status")
                .num_args(1)
                .value_name("status")
                .value_parser(clap::builder::EnumValueParser::<PremiumStatus>::new())
                .default_value("premium")
                .help("Premium status"),
        )
        .arg(
            Arg::new("target")
                .long("target")
                .num_args(1)
                .value_name("product")
                .value_parser(clap::builder::EnumValueParser::<Product>::new())
                .default_value("major-poison-potion")
                .help("Target product"),
        )
        .get_matches();

    // Get the values of the arguments
    let brecilien_plots = matches
        .get_one::<String>("brecilien-plots")
        .map(|s| {
            s.parse::<u32>()
                .expect("num_plots must be a positive integer")
        })
        .unwrap();
    let bridgewatch_plots = matches
        .get_one::<String>("bridgewatch-plots")
        .map(|s| {
            s.parse::<u32>()
                .expect("num_plots must be a positive integer")
        })
        .unwrap();
    let caerleon_plots = matches
        .get_one::<String>("caerleon-plots")
        .map(|s| {
            s.parse::<u32>()
                .expect("num_plots must be a positive integer")
        })
        .unwrap();
    let fort_sterling_plots = matches
        .get_one::<String>("fort-sterling-plots")
        .map(|s| {
            s.parse::<u32>()
                .expect("num_plots must be a positive integer")
        })
        .unwrap();
    let lymhurst_plots = matches
        .get_one::<String>("lymhurst-plots")
        .map(|s| {
            s.parse::<u32>()
                .expect("num_plots must be a positive integer")
        })
        .unwrap();
    let martlock_plots = matches
        .get_one::<String>("martlock-plots")
        .map(|s| {
            s.parse::<u32>()
                .expect("num_plots must be a positive integer")
        })
        .unwrap();
    let thetford_plots = matches
        .get_one::<String>("thetford-plots")
        .map(|s| {
            s.parse::<u32>()
                .expect("num_plots must be a positive integer")
        })
        .unwrap();
    let premium_status = matches
        .get_one::<PremiumStatus>("premium-status")
        .expect("premium-status is required");
    let target = matches
        .get_one::<Product>("target")
        .expect("target is required");

    // Display input conditions
    println!("Brecilien Plots: {:?}", brecilien_plots);
    println!("Bridgewatch Plots: {:?}", bridgewatch_plots);
    println!("Caerleon Plots: {:?}", caerleon_plots);
    println!("Fort Sterling Plots: {:?}", fort_sterling_plots);
    println!("Lymhurst Plots: {:?}", lymhurst_plots);
    println!("Martlock Plots: {:?}", martlock_plots);
    println!("Thetford Plots: {:?}", thetford_plots);
    println!("Premium Status: {:?}", premium_status);
    println!("Target: {:?}", target);

    let context = ModelContext {
        brecilien_plots: brecilien_plots as f64,
        bridgewatch_plots: bridgewatch_plots as f64,
        caerleon_plots: caerleon_plots as f64,
        fort_sterling_plots: fort_sterling_plots as f64,
        lymhurst_plots: lymhurst_plots as f64,
        martlock_plots: martlock_plots as f64,
        thetford_plots: thetford_plots as f64,
        premium_factor: if *premium_status == PremiumStatus::Premium {
            PREMIUM_FACTOR
        } else {
            FREE_FACTOR
        },
        target: *target,
    };

    let frames = vec!["|", "/", "-", "\\"];
    let mut frame_index = 0;
    let running = Arc::new(Mutex::new(true));
    let running_clone = Arc::clone(&running);

    let loading_handle = thread::spawn(move || {
        loop {
            print!("Thinking {}  \r", frames[frame_index]);
            io::stdout().flush().unwrap();
            frame_index = (frame_index + 1) % frames.len();
            thread::sleep(Duration::from_millis(100));
            if !*running_clone.lock().unwrap() {
                print!("            \r");
                io::stdout().flush().unwrap();
                break;
            }
        }
    });

    let plot_plan = optimize_plots(context);

    *running.lock().unwrap() = false;
    loading_handle.join().unwrap();

    println!("{}", plot_plan.display());
}

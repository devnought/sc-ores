use clap::{Parser, Subcommand, ValueEnum};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    commands: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Get common signatures for a specific type of ore
    Ore {
        /// Type of ore
        ore_type: Ore,

        /// Max cluster size
        #[arg(default_value_t = 8)]
        cluster_size: u32,
    },

    /// Lookup ore type based on signature
    Sig { signature: u32 },
}

#[derive(Debug, Clone, ValueEnum, EnumIter)]
enum Ore {
    Agricium,
    Aluminum,
    Aslarite,
    Beryl,
    Bexalite,
    Borase,
    Copper,
    Corundum,
    Gold,
    Hephaestanite,
    Iron,
    Laranite,
    Lindinium,
    Ouratite,
    Quantainium,
    Quartz,
    RawIce,
    RawSilicon,
    Riccite,
    Savrilium,
    Stileron,
    Taranite,
    Tin,
    Titanium,
    Torite,
    Tungsten,
}

impl Ore {
    fn value(&self) -> u32 {
        match self {
            Self::Agricium => 3885,
            Self::Aluminum => 4285,
            Self::Aslarite => 3840,
            Self::Beryl => 3540,
            Self::Bexalite => 3600,
            Self::Borase => 3570,
            Self::Copper => 4240,
            Self::Corundum => 4225,
            Self::Gold => 3585,
            Self::Hephaestanite => 4180,
            Self::Iron => 4270,
            Self::Laranite => 3825,
            Self::Lindinium => 3400,
            Self::Ouratite => 3370,
            Self::Quantainium => 3170,
            Self::Quartz => 4210,
            Self::RawIce => 4300,
            Self::RawSilicon => 4255,
            Self::Riccite => 3385,
            Self::Savrilium => 3200,
            Self::Stileron => 3185,
            Self::Taranite => 3555,
            Self::Tin => 4195,
            Self::Titanium => 3855,
            Self::Torite => 3900,
            Self::Tungsten => 3870,
        }
    }
}

fn main() {
    let args = Args::parse();

    match args.commands {
        Command::Ore {
            ore_type,
            cluster_size,
        } => {
            let value = ore_type.value();
            for i in 1..=cluster_size {
                println!("{}", i * value);
            }
        }
        Command::Sig { signature } => {
            for ore in Ore::iter() {
                let value = ore.value();

                if signature % value != 0 {
                    continue;
                }

                println!("{ore:?}");
            }
        }
    }
}

#![allow(clippy::new_ret_no_self)]

mod data;
mod schema;
mod static_api;
mod validate;

use crate::data::Data;
use failure::{err_msg, Error};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(structopt::StructOpt)]
#[structopt(name = "team", about = "manage the rust team members")]
enum Cli {
    #[structopt(name = "check", help = "check if the configuration is correct")]
    Check,
    #[structopt(name = "static-api", help = "generate the static API")]
    StaticApi { dest: String },
    #[structopt(name = "dump-team", help = "print the members of a team")]
    DumpTeam { name: String },
    #[structopt(name = "dump-list", help = "print all the emails in a list")]
    DumpList { name: String },
}

fn main() {
    env_logger::init();
    if let Err(e) = run() {
        eprintln!("error: {}", e);
        for e in e.iter_causes() {
            eprintln!("  cause: {}", e);
        }
        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let cli = Cli::from_args();
    let data = Data::load()?;
    match cli {
        Cli::Check => {
            crate::validate::validate(&data)?;
        }
        Cli::StaticApi { ref dest } => {
            let dest = PathBuf::from(dest);
            let generator = crate::static_api::Generator::new(&dest, &data)?;
            generator.generate()?;
        }
        Cli::DumpTeam { ref name } => {
            let team = data.team(name).ok_or_else(|| err_msg("unknown team"))?;

            let leads = team.leads();
            for member in team.members(&data)? {
                println!(
                    "{}{}",
                    member,
                    if leads.contains(member) {
                        " (lead)"
                    } else {
                        ""
                    }
                );
            }
        }
        Cli::DumpList { ref name } => {
            let list = data.list(name)?.ok_or_else(|| err_msg("unknown list"))?;
            for email in list.emails() {
                println!("{}", email);
            }
        }
    }

    Ok(())
}

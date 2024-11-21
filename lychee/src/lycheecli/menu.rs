use std::fs;
use crate::lycheecli::*;
// use aok::connection::Connection;
use clap::{Arg,Parser, Command, Subcommand};

// use tokio::runtime::Runtime;

#[derive(Parser)]
#[command(name = "lycheecli")]
#[command(about = "A command-line interface for managing databases", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    
    #[clap(about = "Create a new web App. \nExample: \n\tlycheecli new --name=lychee --version=0.1.0")]
    New {
        #[arg(short, long,default_value = "sunny-web")]
        name: String,
        #[arg(short, long,default_value = "2021")]
        version: String,
    },
    
    /// List all databases
    List,
}

pub fn new_menu(){
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name,version } => {
            println!("Creating web App: {} verson:{}", name,version);
        }
        Commands::List => {
            println!("Listing all web app");
        }
    }

}
pub fn menu() {
    let matches = Command::new("Lychee")
        .version("0.1.0")
        .author("Your Name <jinheking@163.com>")
        .about("A CLI tool for scaffold")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("new ")
                .about("Create a new web server")
                .arg(Arg::new("name").short('n').long("name").required(true)),
        )
        .subcommand(
            Command::new("list")
                .about("List this web App"),
        )
        .get_matches();
        // let runtime = Runtime::new().expect("Can not create Tokio runtime");
    match matches.subcommand() {
        Some(("create", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").expect("name is required");
            println!("Web App name: {}", name);
        }
        Some(("list", _)) => {
            println!("List this web App");
            // list_databases(&CFG.app.database_path);
        }
        _ => unreachable!(), // We have set subcommand_required(true) so this case should never happen
    }
}
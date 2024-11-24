use std::fs;
use crate::lycheecli;
use crate::lycheecli::{utils::*,*};
// use aok::connection::Connection;
use clap::{Arg,Parser, Command, Subcommand};
use std::{thread, time};
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
    
    #[clap(about = "Create a new web App. \nExample: \n\tlycheecli new --name=lychee --authors=[\"jinheking@163.com\"] --edition=2021")]
    New {
        #[arg(short, long,default_value = "sunny-web")]
        name: String,
        #[arg(short, long,default_value = "[\"jinheking@163.com\"]")]
        authors: String,
        #[arg(short, long,default_value = "2021")]
        edition: String,
    },
    #[clap(about = "Drop a  web App. \nExample: \n\tlycheecli drop --name=lychee")]
    Drop{
        #[arg(short, long,default_value = "sunny-web")]
        name:String,
    },
    /// List all databases
    List,
}

pub fn new_menu(){
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name,authors,edition} => {
            println!("Creating web App: {}\t author:{}\t edition:{}", name,authors,edition);
            // 判断edition 是不是合格
            let cfg=&crate::CFG;
            // tracing::info!("cfg:{:?}",cfg.app.edition);
            if cfg.app.edition.contains(&edition) {
                // 你的逻辑代码
                tracing::info!("Edition {} is supported", edition);
            } else {
                tracing::error!("Edition {} is not supported", edition);
                std::process::exit(0);
            }
            let millis = time::Duration::from_millis(300);
    
            let lychee_project=Cargo::new(name.to_string(),authors.to_string(),edition.to_string());
            println!("{:?}",lychee_project); 
            lychee_project.mkdir();  
            thread::sleep(millis);  
            let project_name=lychee_project.clone().name; 
            let cargo_toml_path=project_name.to_string()+"/Cargo.toml";
            // create cargo_toml_path file
            let _=lychee_project.create_cargo_toml(cargo_toml_path.clone().to_string());
            thread::sleep(millis);
            // create README.md
            let readme_path=project_name.to_string()+"/README.md";
            let _=lychee_project.create_readme(readme_path.to_string());
            thread::sleep(millis);
            
            // create directorys
            let mut dir_name=project_name.to_string()+"/src";
            lycheecli::mkdir(&dir_name);
            thread::sleep(millis);
            // create main.rs
            dir_name=project_name.to_string()+"/src/main.rs";
            let _=create_file_from_template(dir_name,"./resource/main.template".to_string());
            thread::sleep(millis);
            dir_name=project_name.to_string()+"/static/Css";
            lycheecli::mkdir(&dir_name);
            thread::sleep(millis);
            dir_name=project_name.to_string()+"/static/Images";
            lycheecli::mkdir(&dir_name);
            thread::sleep(millis);
            dir_name=project_name.to_string()+"/static/Js";
            lycheecli::mkdir(&dir_name);
            thread::sleep(millis);
            dir_name=project_name.to_string()+"/templates";
            lycheecli::mkdir(&dir_name);
            thread::sleep(millis);
            dir_name=project_name.to_string()+"/src/controller";
            lycheecli::mkdir(&dir_name);
            thread::sleep(millis);
            dir_name=project_name.to_string()+"/src/model";
            lycheecli::mkdir(&dir_name);
            thread::sleep(millis);
        }
        Commands::Drop { name } => {
            let dir_name=name.to_string()+"/";
            if dir_exists(&dir_name){
                utils::remove_dir(&dir_name).expect("remove dir failed");
            }
            println!("Drop a web app {}",name);
        }
        Commands::List => {
            println!("Listing all web app");
        }
    }

}
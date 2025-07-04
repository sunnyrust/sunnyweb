use crate::lycheecli;
use crate::lycheecli::{utils::*,*};
// use aok::connection::Connection;
use chrono::Local;
use clap::{Parser,  Subcommand};
use std::{thread, time};
use crate::lycheecli::db::sqlite;


#[derive(Parser)]
#[command(name = "lycheecli")]
#[command(about = "A command-line specification for web app", long_about = None)]
#[command(author="Sunny Region", version= crate::GIT_VERSION,  long_about = None)]

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
        #[arg(short, long,default_value = "2024")]
        edition: String,
    },
    #[clap(about = "Drop a  web App. \nExample: \n\tlycheecli drop --name=lychee")]
    Drop{
        #[arg(short, long,default_value = "sunny-web")]
        name:String,
    },
    /// Manage databases for a web App
    #[clap(alias = "db")]
    #[clap(about = "Manage databases for a web App. Short name is 'db'.\nExample: \n\tlycheecli databases --name=auth.db\n\tlycheecli db--name=auth.db")]
    Databases {
        #[arg(short, long,default_value = "sunny-web")]
        name: String,
        #[arg(short, long,default_value = "auth.db")]
        database: String,
    },
    #[clap(about = "List all web App.Short name is 'ls'. \nExample: \n\tlycheecli list\n\tlycheecli ls")]
    #[clap(alias = "ls")]
    /// List all databases
    List,
}

pub fn new_menu(){
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name,authors,edition} => {
            println!("Creating web App: {}\t author:{}\t edition:{}", name,authors,edition);
            // Determine if the edition is in the right format
            let cfg=&crate::CFG;
            if cfg.app.edition.contains(&edition) {
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
            let mut content: String = std::fs::read_to_string(&readme_path)
                    .expect("Failed to read README.md");
            append_content(&readme_path,&mut content, "Project creation date:", format!("\nProject creation date: {}", Local::now().format("%Y-%m-%d")).as_str());
            // create directorys
            let mut dir_name=project_name.to_string()+"/src";
            lycheecli::mkdir(&dir_name);
            thread::sleep(millis);
            // create main.rs
            dir_name=project_name.to_string()+"/src/main.rs";
            let mut params_vec:Vec<TemplateParams> = Vec::new();// Create a vector to hold TemplateParams
            let mut target=project_name.to_string();
            target=target.replace("-", "_");
            let mut tp_target=format!("{}=debug",target);
            let mut tp=TemplateParams::new(true,"lychee=debug".to_string(),tp_target);
            params_vec.push(tp.clone());
            tp_target=format!{"let cfg={}::new(\"{}\");",target,target};
            tp=TemplateParams::new(true,"let cfg=lychee::new(\"lychee\");".to_string(),tp_target);
            params_vec.push(tp.clone());
            tp_target=format!{"let website_name=\"{}\".to_string();",target};
            tp=TemplateParams::new(true,"let website_name=\"lychee\".to_string();".to_string(),tp_target);
            params_vec.push(tp.clone());
            
            tp_target=format!("{}::handle_error",target);
            tp=TemplateParams::new(true,"webhotel::handle_error".to_string(),tp_target);
            params_vec.push(tp.clone());
            if check_file_exists("./resource/main.rs.template"){ 
                match create_file_from_template(dir_name.clone(),"./resource/main.rs.template".to_string(),params_vec){
                    Ok(_)=>println!("Create  main.rs successfully.👌"),
                    Err(e)=>println!("{}",e)
                }
            }else{
                let app = get_main_default().unwrap();
                let content = std::str::from_utf8(app.data.as_ref()).unwrap();
                let s_content=content.to_string();
                let content=s_content.replace(&tp.clone().source,&tp.clone().target);
                let _=create_file_from_str(dir_name.clone(),content.as_bytes(),"Create  main.rs successfully.👌".to_string());
            }
            thread::sleep(millis);
            let use_project_name=lychee_project.clone().name.replace("-", "_");
            let text_to_insert=format!("use {}::{{config,router,dbstate::DbState,utils,AppState,base_controller_middleware}};",use_project_name);
            let _=insert_text_at_beginning(dir_name.clone(),&text_to_insert);
            // // Modify main.rs
            // dir_name=project_name.to_string()+"/src/main.rs";
            // create static directory
            dir_name=project_name.to_string()+"/static/css";
            lycheecli::mkdir(&dir_name);
            thread::sleep(millis);
            dir_name=project_name.to_string()+"/static/images";
            lycheecli::mkdir(&dir_name);
            thread::sleep(millis);
            dir_name=project_name.to_string()+"/static/js";
            lycheecli::mkdir(&dir_name);
            thread::sleep(millis);
            dir_name=project_name.to_string()+"/templates";
            lycheecli::mkdir(&dir_name);
            thread::sleep(millis);
            // create configs directory
            dir_name=project_name.to_string()+"/configs";
            lycheecli::mkdir(&dir_name);
            thread::sleep(millis);
            // create app.toml
            dir_name=project_name.to_string()+"/configs/app.toml";
            if check_file_exists("./resource/app.toml.template"){
                let tp=TemplateParams::default();
                let mut params_vec:Vec<TemplateParams> = Vec::new();// Create a vector to hold TemplateParams
                params_vec.push(tp.clone());
                let _=create_file_from_template(dir_name,"./resource/app.toml.template".to_string(),params_vec);
            }else{
                let app = get_app_default().unwrap();
                let content = std::str::from_utf8(app.data.as_ref()).unwrap();
                let _=create_file_from_str(dir_name,content.as_bytes(),"Create  app.toml successfully.👌".to_string());
            }
            
            thread::sleep(millis);
            //create config.rs
            dir_name="./resource/config.rs.template".to_string();
            if check_file_exists(&dir_name){
                match copy_file(&dir_name,project_name.to_string()+"/src/config.rs"){
                    Ok(_) => println!("Create  config.rs successfully.👌"),
                    Err(e) => {
                        eprintln!("Error copying file: {}❌️", e);
                        std::process::exit(0);
                    },
                }
            }else{
                let app = get_config_default().unwrap();
                let content = std::str::from_utf8(app.data.as_ref()).unwrap();
                let _=create_file_from_str(project_name.to_string()+"/src/config.rs",content.as_bytes(),"Create  config.rs successfully.👌".to_string());
            }
            // copy run.sh
            dir_name="./resource/run.sh".to_string();
            match copy_file(&dir_name,project_name.to_string()+"/run.sh"){
                Ok(_) => println!("Create  run.sh successfully.👌"),
                Err(e) => {
                    eprintln!("Error copying file: {}❌️", e);
                    std::process::exit(0);
                },
            }
            thread::sleep(millis);
            // create lib.rs
            dir_name="./resource/main_lib.rs.template".to_string();
            if check_file_exists(&dir_name){
                match copy_file(&dir_name,project_name.to_string()+"/src/lib.rs"){
                    Ok(_) => println!("Create  lib.rs  successfully.👌"),
                    Err(e) => {
                        eprintln!("Error copying file: {}", e);
                        std::process::exit(0);
                    },
                }
            }else{
                let app = get_main_lib_default().unwrap();
                let content = std::str::from_utf8(app.data.as_ref()).unwrap();
                let _=create_file_from_str(project_name.to_string()+"/src/lib.rs",content.as_bytes(),"Create  lib.rs successfully.👌".to_string());
            }
            thread::sleep(millis);
            // create err.rs
            dir_name="./resource/err.rs.template".to_string();
            if check_file_exists(&dir_name){
                match copy_file(&dir_name,project_name.to_string()+"/src/err.rs"){
                    Ok(_) => println!("Create  err.rs  successfully.👌"),
                    Err(e) => {
                        eprintln!("Error copying file: {}", e);
                        std::process::exit(0);
                    },
                }
            }else{
                let app = get_err_default().unwrap();
                let content = std::str::from_utf8(app.data.as_ref()).unwrap();
                let _=create_file_from_str(project_name.to_string()+"/src/err.rs",content.as_bytes(),"Create  err.rs successfully.👌".to_string());
            }
            thread::sleep(millis);
            // crate router.rs
            dir_name="./resource/router.rs.template".to_string();
            if check_file_exists(&dir_name){
                match copy_file(&dir_name,project_name.to_string()+"/src/router.rs"){
                    Ok(_) => println!("Create  router.rs  successfully.👌"),
                    Err(e) => {
                        eprintln!("Error copying file: {}", e);
                        std::process::exit(0);
                    },
                }
            }else{
                let app = get_router_default().unwrap();
                let content = std::str::from_utf8(app.data.as_ref()).unwrap();
                let _=create_file_from_str(project_name.to_string()+"/src/router.rs",content.as_bytes(),"Create  router.rs successfully.👌".to_string());
            }

            // create controller directory
            // dir_name=project_name.to_string()+"/src/controller";
            // lycheecli::mkdir(&dir_name);
            // thread::sleep(millis);
            dir_name=project_name.to_string()+"/src/model";
            lycheecli::mkdir(&dir_name);
            thread::sleep(millis);

            //copy static to ststic
            let static_path="./resource/static";
            let target_static_path=project_name.to_string()+"/static";
            if check_file_exists(&static_path) {
                match copy_dir(static_path, &target_static_path) {
                    Ok(_) => println!("Copy static files successfully.👌"),
                    Err(e) => {
                        eprintln!("Error copying static files: {}", e);
                        std::process::exit(0);
                    },
                }
            } else {
                println!("Static directory does not exist at {}", static_path);
            }

            // copy templates to templates
            let templates_path="./resource/templates";
            let target_templates_path=project_name.to_string()+"/templates";
            if check_file_exists(&templates_path) {
                match copy_dir(templates_path, &target_templates_path) {
                    Ok(_) => println!("Copy templates files successfully.👌"),
                    Err(e) => {
                        eprintln!("Error copying templates files: {}", e);
                        std::process::exit(0);
                    },
                }
            } else {
                println!("Templates directory does not exist at {}", templates_path);
            }

            //copy locales to locales
            let locales_path="./resource/locales";
            let target_locales_path=project_name.to_string()+"/configs/locales";
            if check_file_exists(&locales_path) {
                match copy_dir(locales_path, &target_locales_path) {
                    Ok(_) => println!("Copy locales files successfully.👌"),
                    Err(e) => {
                        eprintln!("Error copying locales files: {}", e);
                        std::process::exit(0);
                    },
                }
            } else {
                println!("Locales directory does not exist at {}", locales_path);
            }
            //copy src to src
            let src_path="./resource/src";
            let target_src_path=project_name.to_string()+"/src";
            if check_file_exists(&src_path) {
                match copy_dir(src_path, &target_src_path) {
                    Ok(_) => println!("Copy src files successfully.👌"),
                    Err(e) => {
                        eprintln!("Error copying src files: {}", e);
                        std::process::exit(0);
                    },
                }
            } else {
                println!("Src directory does not exist at {}", src_path);
            }
            // copy sunny-derive-trait to ./
            let src_path="./sunny-derive-trait";
            let target_src_path=project_name.to_string()+"/sunny-derive-trait";
            if check_file_exists(&src_path) {
                match copy_dir(src_path, &target_src_path) {
                    Ok(_) => println!("Copy sunny-derive-trait files successfully.👌"),
                    Err(e) => {
                        eprintln!("Error copying sunny-derive-trait files: {}", e);
                        std::process::exit(0);
                    },
                }
            } else {
                println!("sunny-derive-trait directory does not exist at {}", src_path);
            }
            // create Dockerfile
            dir_name=project_name.to_string()+"/Dockerfile";
            if check_file_exists("./resource/Dockerfile.template") {
                let tp=TemplateParams::default();
                let mut params_vec:Vec<TemplateParams> = Vec::new();// Create a vector to hold TemplateParams
                params_vec.push(tp.clone());
                let _=create_file_from_template(dir_name,"./resource/Dockerfile.template".to_string(),params_vec);
            } else {
                let app = get_dockerfile_default().unwrap();
                let content = std::str::from_utf8(app.data.as_ref()).unwrap();
                let _=create_file_from_str(dir_name,content.as_bytes(),"Create  Dockerfile successfully.👌".to_string());
            }
        }
        Commands::Drop { name } => {
            let dir_name=name.to_string()+"/";
            if dir_exists(&dir_name){
                utils::remove_dir(&dir_name).expect("remove dir failed");
            }
            println!("Drop a web app {}",name);
        }
        Commands::Databases { name,database } => {
            println!("Manage databases for web app {}, database: {}", name, database);
            let path=name.to_string();
            if !dir_exists(&path) {
                tracing::error!("Directory is not exists: {}.You must create it first.Example: \n\tlycheecli new --name={} --authors=[\"jinheking@163.com\"] --edition=2021", path,path);
                std::process::exit(1);
            } 
            let db_path = format!("./{}/{}", path, database);
            if check_file_exists(&db_path) {
                println!("Database {} already exists.\nIf you want to initialize it, you could drop it first.", database);
                std::process::exit(0);
            }
            let millis = time::Duration::from_millis(300);
            thread::sleep(millis);
            println!("Database path: {}", db_path);
            //sqlite::init(db_path).await.unwrap();
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    sqlite::init(db_path).await.unwrap();
            });
            println!("Database {} initialized successfully.", database);

            // append to Cargo.toml
            let cargo_toml_path: String = format!("{}/Cargo.toml", path);
            if check_file_exists(&cargo_toml_path) {
                let mut content: String = std::fs::read_to_string(&cargo_toml_path)
                    .expect("Failed to read Cargo.toml");
                
                append_content(&cargo_toml_path,&mut content, "sqlx", "\nsqlx = { version = \"0.8.6\", features = [\"sqlite\", \"runtime-tokio-rustls\"] }");
                append_content(&cargo_toml_path,&mut content, "tokio", "\ntokio = { version = \"1.0\", features = [\"full\"] }");
                append_content(&cargo_toml_path,&mut content, "anyhow", "\nanyhow = \"1.0\"");
                append_content(&cargo_toml_path,&mut content, "rust-argon2", "\nrust-argon2  = \"2.1.0\"");

            } else {
                println!("Cargo.toml does not exist at {}", cargo_toml_path);
            }

        }
        Commands::List => {
            println!("Listing all web app");
        }
    }

}
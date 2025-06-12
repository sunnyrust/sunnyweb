pub mod menu;
pub mod utils;
pub mod config;
pub mod db;
use std::fs;
use serde::{Deserialize, Serialize};
use std::fs::{ OpenOptions};
use std::io::{Write};
use crate::{lycheecli::utils::*,resources::embed_resources::*};

#[derive(Clone,Debug,Serialize, Deserialize)]
/// Cargo.toml necessary element
pub struct Cargo { 
     pub name : String,
     pub version :String,
     pub authors : String,
     pub edition : String,
}

impl Cargo {
  pub fn new(name:String, authors:String, edition:String) -> Self {
    Cargo { name, authors,version:"0.1.0".to_string(), edition }
  }

  pub fn mkdir(&self){
      mkdir(self.name.as_str());
  }
  pub fn create_cargo_toml(&self,path:String)-> std::io::Result<()>{
      Ok(create_cargo_toml(self.clone(),path)?)
  }
  pub fn create_readme(&self,path:String)-> std::io::Result<()>{
      Ok(create_readme(self.clone(),path)?)
  }

} 
/// Check if the specified path is an existing directory
pub(crate) fn dir_exists(path: &str) -> bool {
  fs::metadata(path).map_or(false, |m| m.is_dir())
}
/// create dir
pub(crate)  fn mkdir(path:&str){
    if dir_exists(path) {
      tracing::error!("Directory is exists: {}", path);
      std::process::exit(1);
    } 
    let r = fs::create_dir_all(path);
    match r {
        Err(e) => {
            println!("error creating {}: {}", path, e);
            std::process::exit(1);
        }
        Ok(_) => println!("Project folder created {}: 👌", path),
    }
}



/// create  README.md
pub(crate) fn create_readme(cargo: crate::lycheecli::Cargo,path:String)-> std::io::Result<()>{
  let mut buffer = OpenOptions::new()
  .read(true)
  .write(true)
  .create(true)
  .open(&path).unwrap();
  let mut str_tmp=r#"#  "#;
  buffer.write(str_tmp.as_bytes())?;
  buffer.write(cargo.name.trim().as_bytes())?;
  buffer.write(b"\n")?;
  str_tmp=r#"## Introduction"#;
  buffer.write(str_tmp.as_bytes())?;
  Ok(())
}

/// create Cargo.toml
pub(crate) fn create_cargo_toml(cargo: crate::lycheecli::Cargo,path:String)-> std::io::Result<()>{
    //println!("{}---{:#?}",path,cargo);
    let mut buffer = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(&path).unwrap();

    buffer.write(b"[package]")?;
    buffer.write(b"\n")?;
    let mut str_package=r#"name = ""#;
    buffer.write(str_package.as_bytes())?;
    buffer.write(cargo.name.trim().as_bytes())?;
    buffer.write(b"\"\n")?;
    str_package=r#"version = ""#;
    buffer.write(str_package.as_bytes())?;
    buffer.write(cargo.version.trim().as_bytes())?;
    buffer.write(b"\"\n")?;
    str_package=r#"authors = "#;
    buffer.write(str_package.as_bytes())?;
    buffer.write(cargo.authors.trim().as_bytes())?;
    buffer.write(b"\n")?;
    str_package=r#"edition = ""#;
    buffer.write(str_package.as_bytes())?;
    buffer.write(cargo.edition.trim().as_bytes())?;
    buffer.write(b"\"\n\n")?;

    buffer.write(r"# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html".as_bytes())?;
    buffer.write(b"\n\n")?;
    // let content = include_str!("../../resource/Cargo.template");
    //if  std::path::Path::new("./resource/Cargo.template").exists() {
    if    check_file_exists("./resource/Cargo.template"){
        let content = include_str!("../../resource/Cargo.template");
        buffer.write(content.as_bytes())?;
    }else{
        let app = get_app_cargo_default().unwrap();
        let content = std::str::from_utf8(app.data.as_ref()).unwrap();
        buffer.write(content.as_bytes())?;
    }
    println!("Create  Cargo.toml created.👌");
    buffer.flush()?;

    Ok(())
}

#[allow(dead_code)]
/// ceate index controller
pub(crate) fn create_index_controller(path:String)-> std::io::Result<()>{
    let body=br#"use std::collections::HashMap;
use actix_web::{error, web, Error, HttpResponse, Result};
use vulcan_salute::util::*;
pub struct Index {}
impl Index {
    pub async fn index(
        tmpl: web::Data<tera::Tera>,
        query: web::Query<HashMap<String, String>>,
        // item: impl super::Controller,
    ) -> Result<HttpResponse, Error> {
        let s = if let Some(name) = query.get("username") {
            let mut ctx = tera::Context::new();
            ctx.insert("name", &name.to_owned());
            let welcome = "Welcome!".to_string();
            ctx.insert("text", &welcome);
            tmpl.render("index.html", &ctx)
                .map_err(|_| error::ErrorInternalServerError("Template error"))?
        } else {
            let mut ctx = tera::Context::new();
            ctx.insert("os", &get_os_info());
            tmpl.render("index.html", &ctx)
                .map_err(|_| error::ErrorInternalServerError("Template error"))?
        };
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    }
}
"#;
    let msg=String::from("Index controller created.👌");
    let _result=create_file_from_str(path, body,msg);
    Ok(())
}
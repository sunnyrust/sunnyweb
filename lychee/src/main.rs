use lychee::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::lycheecli::menu;
// mod git_version;
fn logo(){
//     eprintln!(r#"
//     __                         __                              _          __     
//    /\ \                       /\ \                           /' \       /'__`\   
//    \ \ \       __  __     ___ \ \ \___       __      __     /\_, \     /\ \/\ \  
//     \ \ \  __ /\ \/\ \   /'___\\ \  _ `\   /'__`\  /'__`\   \/_/\ \    \ \ \ \ \ 
//      \ \ \L\ \\ \ \_\ \ /\ \__/ \ \ \ \ \ /\  __/ /\  __/      \ \ \  __\ \ \_\ \
//       \ \____/ \/`____ \\ \____\ \ \_\ \_\\ \____\\ \____\      \ \_\/\_\\ \____/
//        \/___/   `/___/> \\/____/  \/_/\/_/ \/____/ \/____/       \/_/\/_/ \/___/ 
//                    /\___/                                                        
//                    \/__/                                                         "#);
   
//    eprintln!(
//                r#"
//        ╔══╗
//        ╚╗╔╝
//        ╔╝(¯`v´¯)
//        ╚══`.¸.[🅻 🅨 🅒 🅗 🅔 🅔 🌐🌱]
//        "#);
    eprintln!(r#"
 _                   _                          
| |     _   _   ___ | |__    ___   ___          
| |    | | | | / __|| '_ \  / _ \ / _ \ 
| |___ | |_| || (__ | | | ||  __/|  __/ 
|_____| \__, | \___||_| |_| \___| \___|  {} 
        |___/                                        

   "#,"1.0");
}
fn main() {
    if let Some(timestamp) = option_env!("VERGEN_BUILD_TIMESTAMP") {
         println!("Build Timestamp: {timestamp}");
    }
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "lychee=debug".into()),  //,tower_http=debug
    ).with(tracing_subscriber::fmt::layer())
    .init();
    // tracing::info!("Starting lychee");
    logo();

    menu::new_menu();
}

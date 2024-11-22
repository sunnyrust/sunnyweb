pub mod lycheecli;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::lycheecli::menu;
fn logo(){
    eprintln!(r#"
    __                         __                              _          __     
   /\ \                       /\ \                           /' \       /'__`\   
   \ \ \       __  __     ___ \ \ \___       __      __     /\_, \     /\ \/\ \  
    \ \ \  __ /\ \/\ \   /'___\\ \  _ `\   /'__`\  /'__`\   \/_/\ \    \ \ \ \ \ 
     \ \ \L\ \\ \ \_\ \ /\ \__/ \ \ \ \ \ /\  __/ /\  __/      \ \ \  __\ \ \_\ \
      \ \____/ \/`____ \\ \____\ \ \_\ \_\\ \____\\ \____\      \ \_\/\_\\ \____/
       \/___/   `/___/> \\/____/  \/_/\/_/ \/____/ \/____/       \/_/\/_/ \/___/ 
                   /\___/                                                        
                   \/__/                                                         "#);
   
   eprintln!(
               r#"
       ╔══╗
       ╚╗╔╝
       ╔╝(¯`v´¯)
       ╚══`.¸.[🅻 🅨 🅒 🅗 🅔 🅔 🌐🌱]
       "#);
}
fn main() {
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "lychee=debug".into()),  //,tower_http=debug
    ).with(tracing_subscriber::fmt::layer())
    .init();


    logo();
    menu::new_menu();
}

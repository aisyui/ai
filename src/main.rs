pub mod refresh;
pub mod token;
pub mod ascii;

use seahorse::{App, Command, Context, Flag, FlagType};
use std::env;
use crate::ascii::c_ascii;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .command(
            Command::new("ai")
            .alias("a")
            .action(c_ascii_art)
            .flag(
                Flag::new("type", FlagType::Bool)
                .description("type flag")
                .alias("t"),
            )
        )
        .command(
            Command::new("refresh")
            .alias("r")
            .action(c_refresh),
            )
        .command(
            Command::new("token")
            .alias("t")
            .description("handle\n\t\t\t$ ai t syui.bsky.social -p password")
            .action(c_token)
            .flag(
                Flag::new("password", FlagType::String)
                .description("password flag")
                .alias("p"),
                )
            )
        ;
    app.run(args);
}

fn c_ascii_art(c: &Context) {
    c_ascii(c.bool_flag("type"));
}

fn refresh(c: &Context) {
    let m = c.args[0].to_string();
    let h = async {
        let str = refresh::post_request(m.to_string()).await;
        println!("{}",str);
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res
}

fn c_refresh(c: &Context) {
    refresh(c);
}

fn token(c: &Context) {
    let m = c.args[0].to_string();
    let h = async {
        if let Ok(p) = c.string_flag("password") {
            let str = token::post_request(m.to_string(), p.to_string()).await;
            println!("{}",str);
        }
    };
    let res = tokio::runtime::Runtime::new().unwrap().block_on(h);
    return res
}

fn c_token(c: &Context) {
    token(c);
}

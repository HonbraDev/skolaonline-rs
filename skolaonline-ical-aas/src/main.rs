// Ignored since it's a Rocket problem
#![allow(clippy::needless_late_init)]

#[macro_use]
extern crate rocket;

use std::net::IpAddr;

use clap::Parser;
use rocket::config::Ident;

mod calendar_response;
mod catcher;
mod endpoint;
mod error;

/// A REST API for the `skolaonline-ical` crate
#[derive(Debug, clap::Parser)]
#[command(version, author)]
struct LaunchArgs {
    /// Base path of the endpoints
    #[clap(short, long, env, default_value = "/")]
    base: String,

    /// Address to listen on
    #[clap(short, long, env, default_value = "0.0.0.0")]
    address: IpAddr,

    /// Port to listen on
    #[clap(short, long, env, default_value = "8000")]
    port: u16,

    /// How to identify the server via the `Server` header.
    /// Must be header-safe. Specify "NONE" to disable
    #[clap(long, env, default_value = "Samsung Smart Fridge")]
    ident: String,
}

#[launch]
fn rocket() -> _ {
    let args = LaunchArgs::parse();

    let ident = if args.ident == "NONE" {
        Ident::none()
    } else {
        Ident::try_new(args.ident).expect("Ident should be header-safe")
    };

    let config = rocket::Config {
        address: args.address,
        port: args.port,
        ident,
        ..Default::default()
    };

    rocket::build()
        .configure(config)
        .mount(args.base, endpoint::routes())
        .register("/", catcher::catchers())
}

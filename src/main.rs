#![feature(plugin, io)]
extern crate iron;
extern crate router;
extern crate mime;
extern crate hyper;
extern crate "rustc-serialize" as rustc_serialize;
extern crate "handlebars-iron" as handlebars_iron;

use hyper::{ Client, Url };
use iron::headers::ContentType;
use iron::prelude::*;
use router::Router;
use rustc_serialize::json::Json;
use std::io::Read;
use handlebars_iron::{ HandlebarsEngine, Template };

fn label(req: &mut Request) -> IronResult<Response> {
  let ref krate = req.extensions.get::<Router>()
    .unwrap().find("crate").unwrap_or("/");
  let uri = Url::parse(
    &*format!("https://crates.io/api/v1/crates/{}", krate))
    .ok().expect("invalid url");
  match Client::new().get(uri).send() {
    Ok(mut res) => {
      let mut s = String::new();
      match res.read_to_string(&mut s)
        .map(|_| s).ok()
        .and_then(|s| Json::from_str(&s).ok()) {
        Some(data) => {
          let mut res = Response::new();
          res.set_mut(iron::status::Ok);
          res.headers.set(ContentType("image/svg+xml".parse().unwrap()));
          res.set_mut(Template::new("label", data));
          Ok(res)
        },
                 _ =>
        Ok(Response::with((iron::status::Ok, "???")))
      }
    },
    Err(e) => {
      let err = format!("Err: {:?}", e);
      Ok(Response::with((iron::status::Ok, err)))
    }
  }
}

fn main() {  
  let mut router = Router::new();
  router.get("/labels/:crate", label);
  let mut chain = Chain::new(router);
  chain.link_after(HandlebarsEngine::new("./src/templates/", ".hbs"));
  Iron::new(chain).http("localhost:3000").unwrap();
}

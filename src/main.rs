#![feature(plugin, io)]
extern crate iron;
extern crate router;
extern crate mime;
extern crate hyper;
extern crate "rustc-serialize" as rustc_serialize;

use rustc_serialize::{ Decoder, Decodable, json };
use std::result;
use std::io::Read;
use hyper::{ Client, Url };
use iron::headers::ContentType;
use iron::prelude::*;
use router::Router;

#[derive(RustcDecodable)]
#[derive(Debug)]
pub struct Crate {
  pub max_version: String
}

// RustcDecodable be derived because the key used in json is `crate`,
// a reserved word
struct CrateReq {
  krate: Crate
}

impl Decodable for CrateReq {
  fn decode<D: Decoder>(d: &mut D) -> result::Result<CrateReq, D::Error> {
    d.read_struct("CrateReq", 1usize, |_d| {
      Ok(CrateReq {
        krate: try!(_d.read_struct_field("crate", 0usize, |_d| Decodable::decode(_d)))
      })
    })
  }
}

fn labeled(req: &mut Request) -> IronResult<Response> {
    let ref krate = req.extensions.get::<Router>()
            .unwrap().find("crate").unwrap_or("/");
    let uri = Url::parse(
        &*format!("https://crates.io/api/v1/crates/{}", krate))
        .ok().expect("invalid url");
    match Client::new().get(uri).send() {
      Ok(mut res) => {
          let mut s = String::new();
          let max_version = res.read_to_string(&mut s)
             .map(|_| s).ok()
             .and_then(|s| json::decode::<CrateReq>(&s).ok())
             .map(|r| r.krate.max_version)
             .unwrap_or("unknown".to_string());
          //_res.content_type(MediaType::Svg);
          let svg = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"112\" height=\"20\"><linearGradient id=\"b\" x2=\"0\" y2=\"100%\"><stop offset=\"0\" stop-color=\"#bbb\" stop-opacity=\".1\"/><stop offset=\"1\" stop-opacity=\".1\"/></linearGradient><mask id=\"a\"><rect width=\"112\" height=\"20\" rx=\"3\" fill=\"#fff\"/></mask><g mask=\"url(#a)\"><path fill=\"#555\" d=\"M0 0h59v20H0z\"/><path fill=\"#fe7d37\" d=\"M59 0h53v20H59z\"/><path fill=\"url(#b)\" d=\"M0 0h112v20H0z\"/></g><g fill=\"#fff\" text-anchor=\"middle\" font-family=\"DejaVu Sans,Verdana,Geneva,sans-serif\" font-size=\"11\"><text x=\"30.5\" y=\"15\" fill=\"#010101\" fill-opacity=\".3\">crates.io</text><text x=\"30.5\" y=\"14\">crates.io</text><text x=\"84.5\" y=\"15\" fill=\"#010101\" fill-opacity=\".3\">v{version}</text><text x=\"84.5\" y=\"14\">v{version}</text></g></svg>",
          version = max_version);
          let content_type = ContentType("image/svg+xml".parse().unwrap());
          let mut res = Response::with((iron::status::Ok, svg));
          res.headers.set(content_type);
          Ok(res)
        },
        Err(e) => {
          let err = format!("Err: {:?}", e);
          Ok(Response::with((iron::status::Ok, err)))
        }
      }
}

fn main() {
  let mut router = Router::new();
  router.get("/labels/:crate", labeled);
  Iron::new(router).http("localhost:3000").unwrap();
}

#![feature(plugin, io, core)]
#[macro_use] extern crate nickel_macros;
extern crate nickel;
extern crate hyper;
extern crate "rustc-serialize" as serialize;

use serialize::json::Json;
use std::old_io::net::ip::Ipv4Addr;
use nickel::{ Nickel, Request, Response, HttpRouter };
use nickel::mimes::MediaType;
use hyper::{ Client, Url };

fn main() {
  let mut server = Nickel::new();
  server.utilize(router! {
    get "/labels/:crate" => |_req, _res| {
      let uri = Url::parse(
          &*format!("https://crates.io/api/v1/crates/{}", _req.param("crate")))
        .ok().expect("invalid url");
      match Client::new().get(uri).send() {
        Ok(mut res) => {
           let payload = res.read_to_string().ok()
               .and_then(|s| Json::from_str(&s).ok());
           let max_version = match payload {
             Some(ref j) => j.find("crate"),
             _ => None
           }.and_then(|j| match j.find("max_version") {
             Some(&Json::String(ref max)) => Some(&max[]),
             _ => None
           }).unwrap_or("unknown");
          _res.content_type(MediaType::Svg);
          _res.send(format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"112\" height=\"20\"><linearGradient id=\"b\" x2=\"0\" y2=\"100%\"><stop offset=\"0\" stop-color=\"#bbb\" stop-opacity=\".1\"/><stop offset=\"1\" stop-opacity=\".1\"/></linearGradient><mask id=\"a\"><rect width=\"112\" height=\"20\" rx=\"3\" fill=\"#fff\"/></mask><g mask=\"url(#a)\"><path fill=\"#555\" d=\"M0 0h59v20H0z\"/><path fill=\"#fe7d37\" d=\"M59 0h53v20H59z\"/><path fill=\"url(#b)\" d=\"M0 0h112v20H0z\"/></g><g fill=\"#fff\" text-anchor=\"middle\" font-family=\"DejaVu Sans,Verdana,Geneva,sans-serif\" font-size=\"11\"><text x=\"30.5\" y=\"15\" fill=\"#010101\" fill-opacity=\".3\">crates.io</text><text x=\"30.5\" y=\"14\">crates.io</text><text x=\"84.5\" y=\"15\" fill=\"#010101\" fill-opacity=\".3\">v{version}</text><text x=\"84.5\" y=\"14\">v0.2.13</text></g></svg>",
          version = max_version))
        },
        Err(e) =>
          _res.send(format!("Err: {:?}", e))
      }

      
    }
  });
  server.listen(Ipv4Addr(0, 0, 0, 0), 6767);
}
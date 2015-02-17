#![feature(plugin, io, core)]
#[macro_use] extern crate nickel_macros;
extern crate nickel;

use std::old_io::net::ip::Ipv4Addr;
use nickel::{ Nickel, Request, Response, HttpRouter };
use nickel::mimes::MediaType;

fn main() {
  let mut server = Nickel::new();
  server.utilize(router! {
    get "/labels/:crate" => |_req, _res| {
      _res.content_type(MediaType::Svg);
      _res.send(format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"112\" height=\"20\"><linearGradient id=\"b\" x2=\"0\" y2=\"100%\"><stop offset=\"0\" stop-color=\"#bbb\" stop-opacity=\".1\"/><stop offset=\"1\" stop-opacity=\".1\"/></linearGradient><mask id=\"a\"><rect width=\"112\" height=\"20\" rx=\"3\" fill=\"#fff\"/></mask><g mask=\"url(#a)\"><path fill=\"#555\" d=\"M0 0h59v20H0z\"/><path fill=\"#fe7d37\" d=\"M59 0h53v20H59z\"/><path fill=\"url(#b)\" d=\"M0 0h112v20H0z\"/></g><g fill=\"#fff\" text-anchor=\"middle\" font-family=\"DejaVu Sans,Verdana,Geneva,sans-serif\" font-size=\"11\"><text x=\"30.5\" y=\"15\" fill=\"#010101\" fill-opacity=\".3\">{crate}</text><text x=\"30.5\" y=\"14\">{crate}</text><text x=\"84.5\" y=\"15\" fill=\"#010101\" fill-opacity=\".3\">v0.2.13</text><text x=\"84.5\" y=\"14\">v0.2.13</text></g></svg>", crate = _req.param("crate")));
    }
  });
  server.listen(Ipv4Addr(0, 0, 0, 0), 6767);
}
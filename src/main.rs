#[macro_use]
extern crate nickel;

use std::collections::HashMap;
use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, MediaType,
             StaticFilesHandler};


fn content_type<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut data = HashMap::<&str, String>::new();
    // println!("pId:	{}", req.param("pId").is_some());
    let mut id = "index.html";

    if (req.param("pId").is_some()) {
        id = req.param("pId").unwrap().trim();
    }
    return res.render("webContent/".to_string() + id, &data);
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("src/webContent/"));

    server.get("/",
               middleware! { |_, response|
            let mut data = HashMap::<&str, String>::new();
        //data.insert("name", "user");
        return response.render("webContent/index.html", &data);
    });

    server.get("/getPage", content_type);

    server.listen("127.0.0.1:6767");
}

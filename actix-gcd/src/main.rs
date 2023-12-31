use actix_web::{web, App, HttpResponse, HttpServer}; // Similar to python: from x import y, where using actix_web by itself is import x
use serde::Deserialize;     // Declarations can be in any order/location, but recommend grouping near top for maintainability and review

#[derive(Deserialize)]      // attribute: tells serde to generate code to parse from data in from HTML POST requests (or JSON, YAML, TOML, etc)
struct GcdParameters {
    n: u64,                 // Notable difference from C or C++: commas instead of semicolon
    m: u64,
}

fn main() {
    let server = HttpServer::new(|| {  // This closure is a callback function with no arguments, each service request will create new App
        App::new()
            .route("/",web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http:://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run().expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post"/>
                <input type="text" name="n">
                <input type="text" name="m">
                <button type="submit">Compute GCD</button>
                </form>
            "#,
        ) // no semi-colon makes this a returnable object
}

fn post_gcd(form : web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response = 
        format!("The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
                 form.n,
                 form.m,
                 gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15), 1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}
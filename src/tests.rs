use super::rocket;

use rocket::http::{RawStr, Status, Method::*};
use rocket::local::blocking::Client;
use rocket_dyn_templates::{Template, context};

fn test_root() {
    // Check that the redirect works.
    let client = Client::tracked(rocket()).unwrap();
    for method in &[Get, Head] {
        let response = client.req(*method, format!("/")).dispatch();
        assert_eq!(response.status(), Status::SeeOther);
        assert!(response.body().is_none());

        let location = response.headers().get_one("Location").unwrap();
        assert_eq!(location, format!("/hello/Your%20Name"));
    }

    // Check that other request methods are not accepted (and instead caught).
    for method in &[Post, Put, Delete, Options, Trace, Connect, Patch] {
        let context = context! { uri: format!("/") };
        let expected = Template::show(client.rocket(), format!("/error/404"), &context);

        let response = client.req(*method, format!("/")).dispatch();
        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(response.into_string(), expected);
    }
}

fn test_name() {
    // Check that the /hello/<name> route works.
    let client = Client::tracked(rocket()).unwrap();
    let response = client.get(format!("/hello/Jack%20Daniels")).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.into_string().unwrap().contains("Hi Jack Daniels!"));
}

fn test_404() {
    // Check that the error catcher works.
    let client = Client::tracked(rocket()).unwrap();
    for bad_path in &["/hello", "/foo/bar", "/404"] {
        let path = format!("/{}", bad_path);
        let escaped_path = RawStr::new(&path).html_escape().to_lowercase();

        let response = client.get(&path).dispatch();
        assert_eq!(response.status(), Status::NotFound);
        let response = response.into_string().unwrap().to_lowercase();

        assert! {
            response.contains(&format!("{} does not exist", path))
                || response.contains(&format!("{} does not exist", escaped_path))
        };
    }
}

fn test_about() {
    let client = Client::tracked(rocket()).unwrap();
    let response = client.get(format!("/about")).dispatch();
    assert!(response.into_string().unwrap().contains("About - Here's another page!"));
}

#[test]
fn hbs() {
    test_root();
    test_name();
    test_404();
    test_about();
}
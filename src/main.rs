#[macro_use]
extern crate rocket;

mod hbs;

#[cfg(test)]
mod tests;

use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;

#[get("/")]
#[allow(dead_code)]
fn index() -> RawHtml<&'static str> {
    RawHtml(
        r#"See <a href="tera">Tera</a>,
        <a href="hbs">Handlebars</a>,
        or <a href="minijinja">MiniJinja</a>."#,
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hbs::index, hbs::hello, hbs::about])
        .register("/", catchers![hbs::not_found])
        .attach(Template::custom(|engines| {
            hbs::customize(&mut engines.handlebars);
        }))
}

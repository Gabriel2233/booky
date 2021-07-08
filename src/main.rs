use rocket::response::Redirect;
mod utils;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    if cmd == "" {
        panic!("no cmd")
    }

    let command = if cmd.contains(" ") {
        let space_index = cmd.find(" ").unwrap_or(0);

        &cmd[..space_index]
    } else {
        &cmd
    };

    let redirect = match command {
        "gh" => utils::handle_github_search(&cmd),
        "tw" => utils::handle_twitch_search(&cmd),
        _ => utils::handle_google_search(&cmd),
    };

    Redirect::to(redirect)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, search])
}

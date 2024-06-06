mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_request;

fn main() {
    let user_req: String = get_user_request("What web-server are we building today?");

    dbg!(user_req);
}

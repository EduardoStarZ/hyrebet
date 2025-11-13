pub mod routes;
pub mod schema;
pub mod database;

pub fn post_route_to_str(user : String, id : i32) -> String {
    return format!("{user}////:////{id}");
}

pub fn str_to_post_route(path : String) -> Option<(String, i32)> {
    return match path.split_once("////:////") {
        Some(tuple) => return Some((tuple.0.to_string(), tuple.1.parse::<i32>().unwrap())),
        None => None
    };
}

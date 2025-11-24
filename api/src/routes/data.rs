use crate::{database::{self, NewPost, Post}, Base};
use chrono::NaiveDateTime;
use common::session::{get_user_from_token, LoginToken};
use ntex::web;
use ntex::http::HttpMessage;
use serde::{Deserialize, Serialize};
use askama::Template;
use crate::{post_route_to_str, str_to_post_route};

#[derive(Deserialize)]
struct PostPath {
    pub user : String,
    pub id : i32
}

#[derive(Template)]
#[template(path = "data/post.html")]
pub struct PostTemplate {
    pub posts : Vec<PostWrapper>
}

#[derive(Serialize)]
pub struct PostWrapper {
    pub id : i32,
    pub owner : String,
    pub contents : String,
    pub time : NaiveDateTime,
    pub repost : Option<Post>,
    pub total_likes : i32
}

impl PostWrapper {
    pub fn new(post : &Post) -> PostWrapper { 
        return PostWrapper {
            id: post.id,
            contents: post.contents.clone(),
            owner: post.owner.clone(),
            time: post.time,
            total_likes: post.total_likes,
            repost : {
                match &post.repost {

                    Some(value) =>{
                        let (name , id ) = str_to_post_route(value.to_string()).unwrap();  

                        match database::get_post(&name, id) {
                            Some(value) => Some(value),
                            None => None
                        }
                    },
                    None => None
                }
            }
        
        };
    }
}


#[derive(Deserialize)]
struct PostQuery {
    pub fullview : Option<String>
}

#[derive(Template)]
#[template(path = "data/full_post.html")]
struct FullPostTemplate {
    pub post : PostWrapper,
    pub replies : Vec<PostWrapper>
}

#[web::get("/{user}/{id}")]
pub async fn get_post(path : web::types::Path<PostPath>, query : web::types::Query<PostQuery>) -> web::HttpResponse {
    let sel_post : Post = match database::get_post(&path.user, path.id) {
        Some(value) => value,
        None => return web::HttpResponse::NotFound().finish()
    };

    if query.fullview.clone().is_some_and(|value| value == "true") {
        let untreated_replies : Vec<Post> = match database::get_replies(&sel_post) {
            Some(value) => value,
            None => Vec::new()
        };

        let treated_replies = untreated_replies.iter().map(|reply| PostWrapper::new(&reply)).collect::<Vec<PostWrapper>>();

        let body : String = FullPostTemplate{post : PostWrapper::new(&sel_post), replies : treated_replies}.render().unwrap();

        let template : String = Base{title : format!("{}'s post", &path.user), scripts: None, body: body}.render().unwrap();

        return web::HttpResponse::Ok().body(template);
    }

    let template : String = PostTemplate{posts: vec![PostWrapper::new(&sel_post)]}.render().unwrap();
    
    return web::HttpResponse::Ok().body(template); 
}

#[web::get("/{user}/{id}")]
pub async fn get_post_json(path : web::types::Path<PostPath>) -> web::HttpResponse {
    let sel_post : Post = match database::get_post(&path.user, path.id) {
        Some(value) => value,
        None => return web::HttpResponse::NotFound().finish()
    };
    
    return web::HttpResponse::Ok().json(&sel_post); 
}




#[derive(Deserialize)]
struct RepostForm {
    pub contents : String
}

#[derive(Deserialize)]
struct RepostPath {
    pub op_type : String,
    pub user : String,
    pub post : i32
}

#[web::post("/{op_type}/{user}/{post}")]
pub async fn create_post(path : web::types::Path<RepostPath>, form : web::types::Form<RepostForm>, request : web::HttpRequest) -> web::HttpResponse {
    let auth_cookie : String = request.cookie("Auth").unwrap().value().to_string();

    let user = match get_user_from_token(&LoginToken::Value(auth_cookie)) {
        Some(value) => value,
        None => return web::HttpResponse::Unauthorized().finish()
    };

    let mut new_post : NewPost = NewPost::new(user, form.0.contents.clone(), None, None);

    match path.op_type.as_str() {
        "repost" => new_post.repost = Some(post_route_to_str(path.user.clone(), path.post)),
        "reply" => new_post.reply = Some(post_route_to_str(path.user.clone(), path.post)),
        "post" => {},
        _ => return web::HttpResponse::BadRequest().finish()
    }

    return match database::create_post(new_post) {
        Some(post) => {
            let template : String = PostTemplate { posts: vec![PostWrapper::new(&post)]}.render().unwrap();
        
            web::HttpResponse::Ok().body(template)
        },
        None => web::HttpResponse::NotAcceptable().finish()
    }
}


#[web::post("/{op_type}/{user}/{post}")]
pub async fn create_post_json(path : web::types::Path<RepostPath>, form : web::types::Json<RepostForm>, request : web::HttpRequest) -> web::HttpResponse {
    let auth_cookie : String = request.cookie("Auth").unwrap().value().to_string();

    let user = match get_user_from_token(&LoginToken::Value(auth_cookie)) {
        Some(value) => value,
        None => return web::HttpResponse::Unauthorized().finish()
    };

    let mut new_post : NewPost = NewPost::new(user, form.0.contents.clone(), None, None);

    match path.op_type.as_str() {
        "repost" => new_post.repost = Some(post_route_to_str(path.user.clone(), path.post)),
        "reply" => new_post.reply = Some(post_route_to_str(path.user.clone(), path.post)),
        "post" => {},
        _ => return web::HttpResponse::BadRequest().finish()
    }

    return match database::create_post(new_post) {
        Some(post) => {
            web::HttpResponse::Ok().json(&post)
        },
        None => web::HttpResponse::NotAcceptable().finish()
    }
}


#[derive(Serialize)]
struct LikeJson {
    pub did_like: bool,
    pub post : Post
}

#[derive(Template)]
#[template(path = "data/like_button.html")]
struct LikeButton {
    pub did_like: bool,
    pub post: Post
}

#[web::get("/like/{user}/{id}")]
pub async fn get_like(path : web::types::Path<PostPath>, query : web::types::Query<LikeQuery>, request : web::HttpRequest) -> web::HttpResponse {
    let auth_cookie : String = request.cookie("Auth").unwrap().value().to_string();

    let user = match get_user_from_token(&LoginToken::Value(auth_cookie)) {
        Some(value) => value,
        None => return web::HttpResponse::Unauthorized().finish()
    };

    let route : String = post_route_to_str(path.user.clone(), path.id);

    let liked : bool = database::liked(route.clone(), user);

    return { 
            let sel_post : Post = database::get_post(&path.user, path.id).unwrap();
            let template = LikeButton{did_like: liked, post: sel_post.clone()}.render().unwrap();
            
            if query.number.clone().is_some_and(|num| num == "true") {
                return web::HttpResponse::Ok().body(format!("{}", sel_post.total_likes));
            } 

            web::HttpResponse::Ok().body(template)
    }
}



#[web::get("/like/{user}/{id}")]
pub async fn get_like_json(path : web::types::Path<PostPath>, request : web::HttpRequest) -> web::HttpResponse {
    let auth_cookie : String = request.cookie("Auth").unwrap().value().to_string();

    let user = match get_user_from_token(&LoginToken::Value(auth_cookie)) {
        Some(value) => value,
        None => return web::HttpResponse::Unauthorized().finish()
    };

    let route : String = post_route_to_str(path.user.clone(), path.id);

    let liked : bool = database::liked(route.clone(), user);

    return { 
            let sel_post : Post = database::get_post(&path.user, path.id).unwrap();
            let json = LikeJson{did_like: liked, post: sel_post.clone()};
            
            web::HttpResponse::Ok().json(&json)
    }
}


#[derive(Deserialize)]
struct LikeQuery {
    pub number : Option<String>
}

#[web::put("/like/{user}/{id}")]
pub async fn like(path : web::types::Path<PostPath>, request : web::HttpRequest) -> web::HttpResponse {
    let auth_cookie : String = request.cookie("Auth").unwrap().value().to_string();

    let user = match get_user_from_token(&LoginToken::Value(auth_cookie)) {
        Some(value) => value,
        None => return web::HttpResponse::Unauthorized().finish()
    };

    let route : String = post_route_to_str(path.user.clone(), path.id);

    return match database::like(user, route.clone()) {
        true => { 
            let sel_post : Post = database::get_post(&path.user, path.id).unwrap();
            let template = LikeButton{did_like: true, post: sel_post}.render().unwrap();
            
            web::HttpResponse::Ok().body(template)
        },
        false => {
            let sel_post : Post = database::get_post(&path.user, path.id).unwrap();
            let template = LikeButton{did_like: false, post: sel_post}.render().unwrap();
            web::HttpResponse::Ok().body(template)
        }
    }
}


#[web::put("/like/{user}/{id}")]
pub async fn like_json(path : web::types::Path<PostPath>, request : web::HttpRequest) -> web::HttpResponse {
    let auth_cookie : String = request.cookie("Auth").unwrap().value().to_string();

    let user = match get_user_from_token(&LoginToken::Value(auth_cookie)) {
        Some(value) => value,
        None => return web::HttpResponse::Unauthorized().finish()
    };

    let route : String = post_route_to_str(path.user.clone(), path.id);

    return match database::like(user, route.clone()) {
        true => { 
            let sel_post : Post = database::get_post(&path.user, path.id).unwrap();
            let template = LikeJson{did_like: true, post: sel_post};
            
            web::HttpResponse::Ok().json(&template)
        },
        false => {
            let sel_post : Post = database::get_post(&path.user, path.id).unwrap();
            let template = LikeJson{did_like: false, post: sel_post};
            web::HttpResponse::Ok().json(&template)
        }
    }
}



#[derive(Template)]
#[template(path = "data/profile.html")]
struct ProfileTemplate {
    username : String,
    total_posts : i32,
    posts : String
}

#[web::get("/{user}")]
pub async fn profile(path : web::types::Path<String>) -> web::HttpResponse {
    let all_posts : Vec<Post> = match database::get_all_posts_from_user(&path) {
        Some(value) => value,
        None => return web::HttpResponse::NoContent().finish()
    };

    let treated_posts : Vec<PostWrapper> = all_posts.iter().map(|post| PostWrapper::new(&post)).collect::<Vec<PostWrapper>>();

    let body : String = ProfileTemplate{username: path.clone(), total_posts: treated_posts.len() as i32, posts: PostTemplate { posts: treated_posts }.render().unwrap()}.render().unwrap();

    let template : String = Base {title: format!("Profile - {}", &path), scripts: None, body}.render().unwrap();

    return web::HttpResponse::Ok().body(template);
}

#[web::get("/{user}")]
pub async fn profile_json(path : web::types::Path<String>) -> web::HttpResponse {
    #[derive(Serialize)]
    struct Response {
        username : String,
        total_posts: i32,
        posts: Vec<PostWrapper>
    }

    let all_posts : Vec<Post> = match database::get_all_posts_from_user(&path) {
        Some(value) => value,
        None => return web::HttpResponse::NoContent().finish()
    };

    let treated_posts: Vec<PostWrapper> = all_posts.iter().map(|post| PostWrapper::new(&post)).collect::<Vec<PostWrapper>>();
    
    let total_posts : i32 = all_posts.len() as i32;

    let response : Response = Response {posts: treated_posts, username: path.to_string(), total_posts};

    return web::HttpResponse::Ok().json(&response);
}




#[derive(Template)]
#[template(path = "data/home.html")]
struct HomeTemplate {
    username : String,
    total_posts : i32,
    posts : String
}

#[web::get("/")]
pub async fn home(request : web::HttpRequest) -> web::HttpResponse {
    let auth_cookie : String = request.cookie("Auth").unwrap().value().to_string();

    let user = match get_user_from_token(&LoginToken::Value(auth_cookie)) {
        Some(value) => value,
        None => return web::HttpResponse::Unauthorized().finish()
    };

    let all_posts : i32 = database::get_all().unwrap().len() as i32; 

    let random_posts : Vec<Post> = match database::get_random_posts() {
        Some(value) => value,
        None => return web::HttpResponse::NoContent().finish()
    };

    let treated_posts = random_posts.iter().map(|post| PostWrapper::new(&post)).collect::<Vec<PostWrapper>>();

    let body : String = HomeTemplate{posts: PostTemplate { posts: treated_posts }.render().unwrap(), username : user, total_posts: all_posts}.render().unwrap();

    let template : String = Base {title: String::from("Home"), scripts: None, body}.render().unwrap();

    return web::HttpResponse::Ok().body(template);
}

#[derive(Serialize)]
struct HomeJson {
    username : String,
    total_posts : i32,
    posts : Vec<PostWrapper>
}

#[web::get("/")]
pub async fn home_json(request : web::HttpRequest) -> web::HttpResponse {
    let auth_cookie : String = request.cookie("Auth").unwrap().value().to_string();

    let user = match get_user_from_token(&LoginToken::Value(auth_cookie)) {
        Some(value) => value,
        None => return web::HttpResponse::Unauthorized().finish()
    };

    let all_posts : i32 = database::get_all().unwrap().len() as i32; 

    let random_posts : Vec<Post> = match database::get_random_posts() {
        Some(value) => value,
        None => return web::HttpResponse::NoContent().finish()
    };

    let treated_posts : Vec<PostWrapper> = random_posts.iter().map(|post| PostWrapper::new(post)).collect::<Vec<PostWrapper>>();

    let json = HomeJson{posts: treated_posts, username : user, total_posts: all_posts};

    return web::HttpResponse::Ok().json(&json);
}

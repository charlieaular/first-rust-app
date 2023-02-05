use actix_web::{get, post, web, Responder};

use super::super::domain::Playlist;
use super::dtos::{Info, CreatePlaylist};

#[get("/playlists")]
async fn playlists() -> impl Responder {
    let mut playlists: Vec<Playlist> = vec![];

    let p1: Playlist = Playlist {
        name: "Charlie2022".to_string(),
        songs: vec![],
    };

    playlists.push(p1);

    web::Json(playlists)
}

#[get("/playlists/{id}")]
async fn get_playlists(info: web::Path<Info>) -> impl Responder {
    let all_playlists: Vec<Playlist> = vec![Playlist {
        name: "Charlie2022".to_string(),
        songs: vec![],
    }];

    let p1: Playlist = all_playlists[info.id].clone();

    web::Json(p1)
}

#[post("/playlists")]
async fn create_playlists(dto: web::Json<CreatePlaylist>) -> impl Responder {
    let mut all_playlists: Vec<Playlist> = vec![Playlist {
        name: "Charlie2022".to_string(),
        songs: vec![],
    }];

    let new_playlist: Playlist = Playlist { 
        name: dto.name.clone(),
        songs: vec![],
    };

    all_playlists.push(new_playlist);


    web::Json(all_playlists)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(playlists);
    cfg.service(get_playlists);
    cfg.service(create_playlists);
}

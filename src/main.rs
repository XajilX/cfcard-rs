mod themes;

use axum::{
    Router,
    routing::get,
    extract::{ Query, rejection::QueryRejection },
    http::StatusCode, response::IntoResponse
};
use hyper_tls::HttpsConnector;
use hyper::{Client, body::{self, Buf}};
use image::{codecs::jpeg::JpegEncoder, RgbaImage};
use serde::Deserialize;
use serde_json::Value;
use themes::ThemeGetter;
use lazy_static::lazy_static;

lazy_static! {
    static ref GETTER: ThemeGetter = ThemeGetter::new((themes::clear::gene_clear, themes::clear::fail_clear))
        .add("clear", (themes::clear::gene_clear, themes::clear::fail_clear))
        .add("dark", (themes::dark::gene_dark, themes::dark::fail_dark));
}
#[tokio::main]
async fn main() {
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(index))
}

#[derive(Deserialize)]
struct Params {
    user: String,
    theme: Option<String>
}

async fn index(rparams: Result<Query<Params>, QueryRejection>) -> impl IntoResponse {
    match rparams {
        Ok(Query(params)) => {
            let theme_gene = match &params.theme {
                Some(str) => GETTER.get_gene(str.as_str()),
                None => GETTER.get_default_gene()
            };
            let theme_fail = match &params.theme {
                Some(str) => GETTER.get_fail(str.as_str()),
                None => GETTER.get_default_fail()
            };

            let https = HttpsConnector::new();
            let client = Client::builder().build::<_, hyper::Body>(https);
            let uri = format!("https://codeforces.com/api/user.info?handles={}", params.user)
                .parse().unwrap();
            let rres: Result<Value, &str> = match client.get(uri).await {
                Ok(resp) => match body::to_bytes(resp).await {
                    Ok(body) => Ok(serde_json::from_reader(body.reader()).unwrap()),
                    Err(_) => Err("Source Error")
                }
                Err(_) => Err("Source Error")
            };

            match rres {
                Ok(res) => match res.get("status") {
                    Some(Value::String(x)) if x == "OK" => {
                        let info = &res["result"][0];
                        let rank = match info.get("rank") {
                            Some(x) => x.as_str().unwrap(),
                            None => "unranked"
                        };
                        let rating = match info.get("rating") {
                            Some(x) => x.as_i64().unwrap(),
                            None => -1
                        };
                        let tlink = info["titlePhoto"].as_str().unwrap();
                        let link_avatar = if tlink.get(0..1).unwrap() == "\\" {
                            format!("https:{}", tlink)
                        } else { tlink.to_string() }.parse().unwrap();
                        
                        let ravatar = match client.get(link_avatar).await {
                            Ok(resp) => match body::to_bytes(resp).await {
                                Ok(body) => match image::load_from_memory(&body.to_vec()) {
                                    Ok(img) => Ok(img.to_rgba8()),
                                    Err(_) => Err("Source Error")
                                },
                                Err(_) => Err("Source Error")
                            },
                            Err(_) => Err("Source Error")
                        };
                        
                        match ravatar {
                            Ok(img) => img_to_response(&theme_gene(&params.user, rank, rating, &img)),
                            Err(msg) => img_to_response(&theme_fail(msg))
                        }
                    }
                    Some(_) | None => {
                        img_to_response(&theme_fail("User Not Found"))
                    }
                },
                Err(msg) => {
                    img_to_response(&theme_fail(msg))
                }
            }
        }
        Err(_) => {
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

fn img_to_response(img: &RgbaImage) -> Result<impl IntoResponse, StatusCode> {
    let mut buf: Vec<u8> = Vec::new();
    let mut enc = JpegEncoder::new(&mut buf);
    match enc.encode_image(img) {
        Ok(_) => {
            Ok(([("Content-Type", "image/jpeg")], buf))
        },
        Err(_) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
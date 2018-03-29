extern crate arrayvec;
extern crate cage;
extern crate glium;
extern crate patchgllib;
extern crate rusttype;
extern crate xml;
extern crate yaml_rust;

use patchgllib::{Anchor, Block, Color, start_screen, RemoteScreen, Sigil, WebColor};
use std::thread;
use std::time::Duration;
use yaml_rust::{Yaml, YamlLoader};

static STR_YAML: &'static str = include_str!("example.yaml");

enum Message {
    AddBlock { id: u64, block: Block },
    Close,
}

fn main() {
    start_screen(320, 480, |screen: &RemoteScreen| {
        init_screen_with_yaml(screen, STR_YAML);
        thread::sleep(Duration::from_secs(40));
        screen.close()
    });
}

fn init_screen_with_yaml(screen: &RemoteScreen, str_yaml: &str) {
    let docs = YamlLoader::load_from_str(str_yaml).unwrap();
    for doc in &docs {
        if let Some(message) = message_from_yaml(doc) {
            match message {
                Message::AddBlock { id, block } => screen.add_block(id, block),
                Message::Close => {
                    thread::sleep(Duration::from_secs(3));
                    screen.close();
                    return;
                }
            }
        } else {
            println!("Invalid message {:?}", doc)
        }
    }
}

fn sigil_from_yaml(doc: &Yaml) -> Sigil {
    match doc["type"].as_str().unwrap() {
        "filled-rectangle" => {
            let web_color = WebColor::from_name(doc["color"].as_str().unwrap());
            Sigil::FilledRectangle(Color::from_web(web_color))
        }
        "paragraph" => {
            Sigil::Paragraph {
                line_height: doc["line-height"].as_f64().unwrap() as f32,
                text: doc["text"].as_str().unwrap().to_string(),
            }
        }
        _ => Sigil::FilledRectangle(Color::from_web(WebColor::DeepPink))
    }
}

fn message_from_yaml(doc: &Yaml) -> Option<Message> {
    match doc["message"].as_str().unwrap() {
        "close" => Option::Some(Message::Close),
        "add-block" => Option::Some(Message::AddBlock {
            id: doc["block-id"].as_i64().unwrap() as u64,
            block: Block {
                sigil: sigil_from_yaml(&doc["sigil"]),
                width: doc["width"].as_f64().unwrap() as f32,
                height: doc["height"].as_f64().unwrap() as f32,
                approach: doc["approach"].as_f64().unwrap() as f32,
                anchor: match doc["anchor"].as_str().unwrap() {
                    "top-left" => Anchor::top_left(),
                    _ => Anchor::top_left()
                },
            },
        }),
        _ => Option::None
    }
}

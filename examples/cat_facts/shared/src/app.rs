pub use crux_core::*;
use serde::{Deserialize, Serialize};

pub mod platform;

const CAT_LOADING_URL: &str = "https://c.tenor.com/qACzaJ1EBVYAAAAd/tenor.gif";
const FACT_API_URL: &str = "https://catfact.ninja/fact";
const IMAGE_API_URL: &str = "https://aws.random.cat/meow";

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct CatFact {
    fact: String,
    length: i32,
}

impl CatFact {
    fn format(&self) -> String {
        format!("{} ({} bytes)", self.fact, self.length)
    }
}

#[derive(Default)]
pub struct CatFacts {
    platform: platform::Platform,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Model {
    cat_fact: Option<CatFact>,
    cat_image: Option<CatImage>,
    platform: platform::Model,
    time: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CatImage {
    pub file: String,
}

impl Default for CatImage {
    fn default() -> Self {
        Self {
            file: CAT_LOADING_URL.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ViewModel {
    pub fact: String,
    pub image: Option<CatImage>,
    pub platform: String,
}

#[derive(Serialize, Deserialize)]
pub enum Msg {
    None,
    GetPlatform,
    Platform(platform::PlatformMsg),
    Clear,
    Get,
    Fetch,
    Restore,                   // restore state
    SetState(Option<Vec<u8>>), // receive the data to restore state with
    SetFact(Vec<u8>),
    SetImage(Vec<u8>),
    CurrentTime(String),
}

impl App for CatFacts {
    type Message = Msg;
    type Model = Model;
    type ViewModel = ViewModel;

    fn update(&self, msg: Msg, model: &mut Model) -> Vec<Command<Msg>> {
        match msg {
            Msg::GetPlatform => Command::lift(
                self.platform
                    .update(platform::PlatformMsg::Get, &mut model.platform),
                Msg::Platform,
            ),
            Msg::Platform(msg) => Command::lift(
                self.platform.update(msg, &mut model.platform),
                Msg::Platform,
            ),
            Msg::Clear => {
                model.cat_fact = None;
                model.cat_image = None;
                let bytes = serde_json::to_vec(&model).unwrap();

                vec![
                    key_value::write("state".to_string(), bytes, |_| Msg::None),
                    Command::render(),
                ]
            }
            Msg::Get => {
                if let Some(_fact) = &model.cat_fact {
                    vec![Command::render()]
                } else {
                    model.cat_image = Some(CatImage::default());

                    vec![
                        http::get(FACT_API_URL.to_owned(), Msg::SetFact),
                        http::get(IMAGE_API_URL.to_string(), Msg::SetImage),
                        Command::render(),
                    ]
                }
            }
            Msg::Fetch => {
                model.cat_image = Some(CatImage::default());

                vec![
                    http::get(FACT_API_URL.to_owned(), Msg::SetFact),
                    http::get(IMAGE_API_URL.to_string(), Msg::SetImage),
                    Command::render(),
                ]
            }
            Msg::SetFact(bytes) => {
                let fact = serde_json::from_slice::<CatFact>(&bytes).unwrap();
                model.cat_fact = Some(fact);

                let bytes = serde_json::to_vec(&model).unwrap();

                vec![
                    key_value::write("state".to_string(), bytes, |_| Msg::None),
                    time::get(Msg::CurrentTime),
                ]
            }
            Msg::CurrentTime(iso_time) => {
                model.time = Some(iso_time);
                let bytes = serde_json::to_vec(&model).unwrap();

                vec![
                    key_value::write("state".to_string(), bytes, |_| Msg::None),
                    Command::render(),
                ]
            }
            Msg::SetImage(bytes) => {
                let image = serde_json::from_slice::<CatImage>(&bytes).unwrap();
                model.cat_image = Some(image);

                let bytes = serde_json::to_vec(&model).unwrap();

                vec![
                    key_value::write("state".to_string(), bytes, |_| Msg::None),
                    Command::render(),
                ]
            }
            Msg::Restore => {
                vec![key_value::read("state".to_string(), Msg::SetState)]
            }
            Msg::SetState(bytes) => {
                if let Some(bytes) = bytes {
                    if let Ok(m) = serde_json::from_slice::<Model>(&bytes) {
                        *model = m
                    };
                }

                vec![Command::render()]
            }
            Msg::None => vec![],
        }
    }

    fn view(&self, model: &Model) -> ViewModel {
        let fact = match (&model.cat_fact, &model.time) {
            (Some(fact), Some(time)) => format!("Fact from {}: {}", time, fact.format()),
            (Some(fact), _) => fact.format(),
            _ => "No fact".to_string(),
        };

        let platform = self.platform.view(&model.platform).platform;

        ViewModel {
            platform: format!("Hello {}", platform),
            fact,
            image: model.cat_image.clone(),
        }
    }
}

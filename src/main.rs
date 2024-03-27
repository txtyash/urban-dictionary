use reqwest::get;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use slint::ModelNotify;
use slint::ModelRc;
use slint::ModelTracker;
use slint::PhysicalSize;
use slint::SharedString;
use slint::{Model, VecModel};
use std::error::Error;
use std::rc::Rc;
slint::include_modules!();

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Definitions {
    pub list: Vec<Definition>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Definition {
    pub word: String,
    pub definition: String,
    pub example: String,
}

#[tokio::main]
async fn main() {
    let query = String::from("word");
    let ui = MainWindow::new().unwrap();
    let weak = ui.as_weak();
    ui.window().set_size(PhysicalSize::new(400, 500));

    ui.run().unwrap();
    ui.on_search(move |query| {
        let ui = weak.unwrap();
        slint::spawn_local(async move {
            let defs: Definitions = search(query.into()).await.unwrap();
            // expects ModelRc<(SharedString, SharedString, SharedString)> which I do below
            // ui.set_definitions(defs);
            let defs: Vec<(SharedString, SharedString, SharedString)> = defs
                .list
                .into_iter()
                .map(|def| {
                    (
                        SharedString::from(def.word),
                        SharedString::from(def.definition),
                        SharedString::from(def.example),
                    )
                })
                .collect();
            let model = slint::ModelRc::new(VecModel::from(defs));
            // Now it expects ModelRc<slint_generatedMainWindow::Definition>
            ui.set_definitions(model);
        })
        .unwrap();
    });
}

async fn search(query: String) -> Result<Definitions> {
    let body = get(format!(
        "https://api.urbandictionary.com/v0/define?term={query}"
    ))
    .await?
    .text()
    .await?;
    Ok(serde_json::from_str::<Definitions>(&body)?)
}

/*
    let ui = MainWindow::new().unwrap();
    let weak = ui.as_weak();
    ui.window().set_size(PhysicalSize::new(400, 500));
    ui.run().unwrap();
    ui.on_search(move |text| {
        let ui = weak.unwrap();
        let query = text.to_string();
        slint::spawn_local(async move {
            let response = match search(query).await {
                Ok(defs) => dbg!(defs),
                Err(_) => todo!("Failed to search!"),
            };
            ui.set_response(response);
        })
        .unwrap();
    });
*/

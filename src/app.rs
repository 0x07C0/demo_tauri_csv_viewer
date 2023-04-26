use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use yew::{platform::spawn_local, prelude::*};

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
  async fn invoke(cmd: &str, args: JsValue) -> JsValue;
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
  async fn open(options: JsValue) -> JsValue;
  #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "fs"])]
  async fn readTextFile(path: &str) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
  name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct Filter {
  name: String,
  extensions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct OpenFileOptions {
  filters: Vec<Filter>,
}

impl Default for OpenFileOptions {
  fn default() -> Self {
    Self {
      filters: vec![Filter {
        name: "CSV".to_string(),
        extensions: vec!["csv".to_string()],
      }],
    }
  }
}

#[function_component(App)]
pub fn app() -> Html {
  let open_trigger = use_state(|| false);
  let table_headers = use_state(|| vec![]);
  let table_data = use_state(|| vec![vec![]]);
  {
    let table_headers = table_headers.clone();
    let table_data = table_data.clone();
    let open_trigger = open_trigger.clone();
    use_effect(move || {
      spawn_local(async move {
        if !*open_trigger {
          return;
        }

        let options = to_value(&OpenFileOptions::default()).unwrap();
        let file = open(options).await;
        open_trigger.set(false);
        match file.as_string() {
          Some(path) => {
            let data = readTextFile(&path).await.as_string().unwrap();
            let mut reader = csv::Reader::from_reader(data.as_bytes());
            let header = reader.headers().unwrap();
            table_headers.set(header.iter().map(|s| s.to_string()).collect());
            table_data.set(
              reader.records()
              .map(|record| record.unwrap().iter().map(|s| s.to_string()).collect())
              .collect()
            );
          }
          None => {}
        }
      });

      || {}
    });
  }

  let open_file = {
    let open_trigger = open_trigger.clone();
    Callback::from(move |e: MouseEvent| {
      e.prevent_default();
      open_trigger.set(true);
    })
  };

  html! {
    <main class="container">
      <button onclick={open_file} type="submit">{"Open"}</button>
      <table>
        { table_headers.iter().map(|header| 
          html! {
            <th>{ header }</th>
          }).collect::<Html>()
        }
        { table_data.iter().map(|row| 
          html! {
            <tr>
              { row.iter().map(|cell| {
                html! {
                  <td>{ cell }</td>
                }
              }).collect::<Html>() }
            </tr>
          }).collect::<Html>()
        }
      </table>
    </main>
  }
}

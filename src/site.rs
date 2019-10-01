use std::collections::HashMap;
use stdweb::*;
use yew::{html, prelude::*, services::ConsoleService};

pub enum Msg {
    UpperInput(InputData),
    LowerInput(InputData),
}

pub struct Model {
    symbols: HashMap<String, String>,
    upper_word: String,
    lower_word: String,
    console: ConsoleService,
}

impl Model {
    fn generate(&self) -> String {
        let mut output = String::new();

        let mut upper_chars = self.upper_word.chars();
        for lower_c in self.lower_word.chars() {
            output.push(lower_c);

            if lower_c != ' ' {
                if let Some(upper_c) = upper_chars.next() {
                    if let Some(symbol) = self.symbols.get(&*upper_c.to_string()) {
                        output.push_str(symbol);
                    }
                }
            }
        }

        output
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut symbols = HashMap::new();
        for line in include_str!("symbols.csv").lines() {
            let mut split = line.split(',');
            symbols.insert(split.next().unwrap().into(), split.next().unwrap().into());
        }

        // get the url hash (#) and parse the words from it
        let hash = js! {
            return location.hash;
        };
        let (upper_word, lower_word) = parse_hash(&hash.into_string().unwrap());
        let title = format!("Multiline Text Generator: {}, {}", upper_word, lower_word);
        js! { @(no_return)
            document.title = @{title};
        }

        Model {
            symbols,
            upper_word,
            lower_word,
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpperInput(data) => {
                let value = data.value.to_lowercase();

                for c in value.chars() {
                    if !self.symbols.contains_key(&c.to_string()) {
                        self.console.warn(&format!("Unsupported char: \"{}\"", c));
                    }
                }

                self.upper_word = value;
            }
            Msg::LowerInput(data) => {
                self.lower_word = data.value;
            }
        }

        // write new words if both have a sufficient length
        if self.upper_word.len() > 1 && self.lower_word.len() > 1 {
            let new_hash = format!("#{}#{}", self.upper_word, self.lower_word);
            let new_title = format!("Multiline Text Generator: {}, {}", self.upper_word, self.lower_word);
            js! { @(no_return)
                history.pushState(null, @{new_title}, @{new_hash});
                document.title = @{new_title};
            };

            true
        } else {
            false
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div id="container",>
              <h1 id="heading",>{ "Multiline Text Generator" }</h1>
              <div id="wrapper",>
                <input type="text", value=&self.upper_word, placeholder="Upper Text", class="line input", oninput=|event| Msg::UpperInput(event), />
                <input type="text", value=&self.lower_word, placeholder="Lower Text", class="line input", oninput=|event| Msg::LowerInput(event), />
                <div id="output", class="line",>{ self.generate() }</div>
              </div>
            </div>
        }
    }
}

fn parse_hash(hash: &str) -> (String, String) {
    let mut split = hash.split('#').skip(1);

    let upper_word: String = split.next().unwrap_or_default().into();
    let upper_word_decoded =
        percent_encoding::percent_decode(upper_word.as_bytes()).decode_utf8_lossy();

    let lower_word: String = split.next().unwrap_or_default().into();
    let lower_word_decoded =
        percent_encoding::percent_decode(lower_word.as_bytes()).decode_utf8_lossy();

    (upper_word_decoded.into(), lower_word_decoded.into())
}

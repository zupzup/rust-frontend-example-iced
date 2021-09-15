use iced::{
    button, executor, futures, Align, Application, Button, Clipboard, Column, Command, Element,
    Settings, Text,
};
use serde::Deserialize;

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    PokemonFound(Result<Pokemon, String>),
}

// TODO: make web request using reqwest and render result

impl Application for Counter {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (Counter, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message, _c: &mut Clipboard) -> Command<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
                Command::perform(Pokemon::bla(), Message::PokemonFound)
            }
            Message::DecrementPressed => {
                self.value -= 1;
                Command::none()
            }
            Message::PokemonFound(_) => {
                self.value = 555;
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("blaaah"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Pokemon {
    number: u16,
    name: String,
    description: String,
}

impl Pokemon {
    async fn bla() -> Result<Pokemon, String> {
        #[derive(Debug, Deserialize)]
        struct Entry {
            id: u32,
            name: String,
        }
        let id = 1;

        let url = format!("https://pokeapi.co/api/v2/pokemon-species/{}", id);
        reqwest::get(&url)
            .await
            .map_err(|_| String::new())?
            .json()
            .await
            .map_err(|_| String::new())
    }
}

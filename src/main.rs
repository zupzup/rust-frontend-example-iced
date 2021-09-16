use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Element, Settings,
    Text,
};
use serde::Deserialize;
use wasm_bindgen::UnwrapThrowExt;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Clone, Debug)]
enum Route {
    List,
    Detail(i32),
}

// TODO: posts and comments from http://jsonplaceholder.typicode.com/
// TODO: list of posts, when clicking on post, go to detail showing posts
// TODO: ultra basic routing between list and detail, no web stuff

struct App {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
    list_button: button::State,
    detail_button: button::State,
    route: Route,
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    PokemonFound(Result<Pokemon, String>),
    GoToList,
    GoToDetail(i32),
}
// TODO: basic router like https://github.com/fitzgen/dodrio/blob/master/examples/todomvc/src/router.rs
// TODO: don't implement, just mention in post

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (App, Command<Message>) {
        (
            App {
                value: 0,
                increment_button: button::State::new(),
                decrement_button: button::State::new(),
                list_button: button::State::new(),
                detail_button: button::State::new(),
                route: Route::List,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("App - Iced")
    }

    fn update(&mut self, message: Message, _c: &mut Clipboard) -> Command<Message> {
        match message {
            Message::GoToList => {
                self.route = Route::List;
                let win = web_sys::window().unwrap_throw();
                win.location().set_hash("/list").unwrap_throw();
                Command::none()
            }
            Message::GoToDetail(id) => {
                let win = web_sys::window().unwrap_throw();
                win.location()
                    .set_hash(&format!("/detail/{}", id))
                    .unwrap_throw();
                self.route = Route::Detail(id);
                Command::perform(Pokemon::bla(id), Message::PokemonFound)
            }
            Message::IncrementPressed => {
                self.value += 1;
                Command::none()
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
        let col = Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Button::new(&mut self.list_button, Text::new("Home")).on_press(Message::GoToList))
            .push(
                Button::new(&mut self.detail_button, Text::new("Detail"))
                    .on_press(Message::GoToDetail(1)),
            )
            .push(
                Button::new(&mut self.increment_button, Text::new("blaaah"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            );
        match self.route {
            Route::List => col.push(Text::new("List page".to_owned()).size(50)).into(),
            Route::Detail(id) => col
                .push(Text::new(format!("Detail page: {}", id)).size(50))
                .into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Pokemon {
    number: u16,
    name: String,
    description: String,
}

impl Pokemon {
    async fn bla(id: i32) -> Result<Pokemon, String> {
        #[derive(Debug, Deserialize)]
        struct Entry {
            id: u32,
            name: String,
        }

        let url = format!("https://pokeapi.co/api/v2/pokemon-species/{}", id);
        reqwest::get(&url)
            .await
            .map_err(|_| String::new())?
            .json()
            .await
            .map_err(|_| String::new())
    }
}

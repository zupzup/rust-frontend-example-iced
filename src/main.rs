use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Element, Settings,
    Text,
};
// use wasm_bindgen::UnwrapThrowExt;

mod data;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Clone, Debug)]
enum Route {
    List,
    Detail(i32),
}

struct App {
    list_button: button::State,
    route: Route,
    posts: Option<Vec<Post>>,
    post: Option<Post>,
    comments: Option<Vec<Comment>>,
}

#[derive(Debug, Clone)]
enum Message {
    PostsFound(Result<Vec<data::Post>, String>),
    PostFound(Result<data::Post, String>),
    CommentsFound(Result<Vec<data::Comment>, String>),
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
                list_button: button::State::new(),
                route: Route::List,
                posts: None,
                post: None,
                comments: None,
            },
            Command::perform(data::Post::fetch_all(), Message::PostsFound),
        )
    }

    fn title(&self) -> String {
        String::from("App - Iced")
    }

    fn update(&mut self, message: Message, _c: &mut Clipboard) -> Command<Message> {
        match message {
            Message::GoToList => {
                self.post = None;
                self.comments = None;
                self.route = Route::List;
                // let win = web_sys::window().unwrap_throw();
                // win.location().set_hash("/list").unwrap_throw();
                Command::perform(data::Post::fetch_all(), Message::PostsFound)
            }
            Message::GoToDetail(id) => {
                // let win = web_sys::window().unwrap_throw();
                // win.location()
                //     .set_hash(&format!("/detail/{}", id))
                //     .unwrap_throw();
                self.route = Route::Detail(id);
                self.posts = None;
                Command::batch(vec![
                    Command::perform(data::Post::fetch(id), Message::PostFound),
                    Command::perform(data::Comment::fetch_for_post(id), Message::CommentsFound),
                ])
            }
            Message::PostsFound(posts) => {
                match posts {
                    Err(_) => (),
                    Ok(data) => {
                        self.posts = Some(
                            data.into_iter()
                                .map(|post| Post {
                                    detail_button: button::State::new(),
                                    post,
                                })
                                .collect(),
                        );
                    }
                };
                Command::none()
            }
            Message::PostFound(post) => {
                match post {
                    Err(_) => (),
                    Ok(data) => {
                        self.post = Some(Post {
                            detail_button: button::State::new(),
                            post: data,
                        });
                    }
                }
                Command::none()
            }
            Message::CommentsFound(comments) => {
                match comments {
                    Err(_) => (),
                    Ok(data) => {
                        self.comments = Some(
                            data.into_iter()
                                .map(|comment| Comment { comment })
                                .collect(),
                        );
                    }
                };
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let col = Column::new().padding(20).align_items(Align::Center).push(
            Button::new(&mut self.list_button, Text::new("Home")).on_press(Message::GoToList),
        );
        match self.route {
            Route::List => {
                let posts: Element<_> = match self.posts {
                    None => Column::new()
                        .push(Text::new("loading...".to_owned()).size(20))
                        .into(),
                    Some(ref mut p) => App::render_posts(p),
                };
                col.push(Text::new("List page".to_owned()).size(50))
                    .push(posts)
                    .into()
            }
            Route::Detail(id) => {
                let post: Element<_> = match self.post {
                    None => Column::new()
                        .push(Text::new("loading...".to_owned()).size(20))
                        .into(),
                    Some(ref mut p) => p.view(),
                };
                let comments: Element<_> = match self.comments {
                    None => Column::new()
                        .push(Text::new("loading...".to_owned()).size(20))
                        .into(),
                    Some(ref mut c) => App::render_comments(c),
                };

                col.push(Text::new(format!("Detail page: {}", id)).size(50))
                    .push(post)
                    .push(comments)
                    .into()
            }
        }
    }
}

impl App {
    fn render_posts(posts: &mut Vec<Post>) -> Element<Message> {
        let c = Column::new();
        let posts: Element<_> = posts
            .iter_mut()
            .fold(Column::new().spacing(10), |col, p| {
                col.push(p.view_in_list())
            })
            .into();
        c.push(posts).into()
    }

    fn render_comments(comments: &Vec<Comment>) -> Element<Message> {
        let c = Column::new();
        let comments: Element<_> = comments
            .iter()
            .fold(Column::new().spacing(10), |col, c| col.push(c.view()))
            .into();
        c.push(Text::new(String::from("Comments:")).size(20))
            .push(comments)
            .into()
    }
}

struct Post {
    detail_button: button::State,
    post: data::Post,
}

impl Post {
    fn view(&mut self) -> Element<Message> {
        let c = Column::new();
        c.push(
            Text::new(format!(
                "{} | {} | {} | {}",
                self.post.id, self.post.user_id, self.post.title, self.post.body
            ))
            .size(12),
        )
        .into()
    }

    fn view_in_list(&mut self) -> Element<Message> {
        let c = Column::new();
        c.push(
            Text::new(format!(
                "{} | {} | {} | {}",
                self.post.id, self.post.user_id, self.post.title, self.post.body
            ))
            .size(12),
        )
        .push(
            Button::new(&mut self.detail_button, Text::new("Detail"))
                .on_press(Message::GoToDetail(self.post.id)),
        )
        .into()
    }
}

struct Comment {
    comment: data::Comment,
}

impl Comment {
    fn view(&self) -> Element<Message> {
        let c = Column::new();
        c.push(
            Text::new(format!(
                "{} | {} | {} | {} | {}",
                self.comment.post_id,
                self.comment.id,
                self.comment.name,
                self.comment.email,
                self.comment.body
            ))
            .size(12),
        )
        .into()
    }
}

use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Element, Row,
    Settings, Text, VerticalAlignment,
};

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
                Command::perform(data::Post::fetch_all(), Message::PostsFound)
            }
            Message::GoToDetail(id) => {
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
        let col = Column::new()
            .max_width(600)
            .spacing(10)
            .padding(10)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.list_button, Text::new("Home")).on_press(Message::GoToList),
            );
        match self.route {
            Route::List => {
                let posts: Element<_> = match self.posts {
                    None => Column::new()
                        .push(Text::new("loading...".to_owned()).size(15))
                        .into(),
                    Some(ref mut p) => App::render_posts(p),
                };
                col.push(Text::new("Home".to_owned()).size(20))
                    .push(posts)
                    .into()
            }
            Route::Detail(id) => {
                let post: Element<_> = match self.post {
                    None => Column::new()
                        .push(Text::new("loading...".to_owned()).size(15))
                        .into(),
                    Some(ref mut p) => p.view(),
                };
                let comments: Element<_> = match self.comments {
                    None => Column::new()
                        .push(Text::new("loading...".to_owned()).size(15))
                        .into(),
                    Some(ref mut c) => App::render_comments(c),
                };

                col.push(Text::new(format!("Post: {}", id)).size(20))
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
        c.push(Text::new(String::from("Comments:")).size(15))
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
        Column::new()
            .push(Text::new(format!("id: {}", self.post.id)).size(12))
            .push(Text::new(format!("user_id: {}", self.post.user_id)).size(12))
            .push(Text::new(format!("title: {}", self.post.title)).size(12))
            .push(Text::new(self.post.body.to_owned()).size(12))
            .into()
    }

    fn view_in_list(&mut self) -> Element<Message> {
        let r = Row::new().padding(5).spacing(5);
        r.push(
            Column::new().spacing(5).push(
                Text::new(self.post.title.to_owned())
                    .size(12)
                    .vertical_alignment(VerticalAlignment::Center),
            ),
        )
        .push(
            Button::new(&mut self.detail_button, Text::new("Detail").size(12))
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
        Column::new()
            .push(Text::new(format!("name: {}", self.comment.name)).size(12))
            .push(Text::new(format!("email: {}", self.comment.email)).size(12))
            .push(Text::new(self.comment.body.to_owned()).size(12))
            .into()
    }
}

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub post_id: i32,
    pub id: i32,
    pub name: String,
    pub email: String,
    pub body: String,
}

impl Post {
    pub async fn fetch_all() -> Result<Vec<Post>, String> {
        let url = String::from("https://jsonplaceholder.typicode.com/posts/");
        reqwest::get(&url)
            .await
            .map_err(|_| String::new())?
            .json()
            .await
            .map_err(|_| String::new())
    }

    pub async fn fetch(id: i32) -> Result<Post, String> {
        let url = format!("https://jsonplaceholder.typicode.com/posts/{}", id);
        reqwest::get(&url)
            .await
            .map_err(|_| String::new())?
            .json()
            .await
            .map_err(|_| String::new())
    }
}

impl Comment {
    pub async fn fetch_for_post(id: i32) -> Result<Vec<Comment>, String> {
        let url = format!(
            "https://jsonplaceholder.typicode.com/posts/{}/comments/",
            id
        );
        reqwest::get(&url)
            .await
            .map_err(|_| String::new())?
            .json()
            .await
            .map_err(|_| String::new())
    }
}

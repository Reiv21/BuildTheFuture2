use std::cell::RefCell;

mod blog;
use crate::blog::Blog;
thread_local! {
    static BLOGS: RefCell<Vec<Blog>> = RefCell::new(Vec::new());
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::update]
fn add_blog(title: String, content: String, tags: Vec<String>) -> Result<String, String> {
    if title.len() > 250
    {
        return Err("Title is too long".to_string())
    }
    if content.len() > 2000
    {
        return Err("Content is too long".to_string())
    }
    if tags.len() > 3
    {
        return Err("Too many tags".to_string())
    }
    let blog = Blog::new(title, content, tags);
    BLOGS.with(|blogs| blogs.borrow_mut().push(blog));
    Ok("Added new blog".to_string())
}

#[ic_cdk::query]
fn get_blogs() -> Vec<Blog> {
    BLOGS.with(|blogs| blogs.borrow().clone())
}
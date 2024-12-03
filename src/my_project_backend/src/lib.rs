use std::cell::RefCell;

mod blog;
thread_local! {
    static BLOGS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn add_blogs(new_blog: String) {
    BLOGS.with(|blogs| blogs.borrow_mut().push(new_blog));
}

#[ic_cdk::query]
fn get_blogs() -> Vec<String> {
    BLOGS.with(|blogs| blogs.borrow().clone())
}
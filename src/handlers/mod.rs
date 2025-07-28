mod blog_handler;
mod demo_handler;
mod home_handler;
mod post_handler;
mod publications_handler;
mod privacy_handler;

pub use blog_handler::blog;
pub use demo_handler::demo;
pub use home_handler::index;
pub use post_handler::post;
pub use publications_handler::publications;
pub use privacy_handler::privacy;

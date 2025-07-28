mod blog_handler;
mod demo_handler;
mod home_handler;
mod post_handler;
mod privacy_handler;
mod publications_handler;

pub use blog_handler::blog;
pub use demo_handler::demo;
pub use demo_handler::demos;
pub use home_handler::index;
pub use post_handler::post;
pub use privacy_handler::privacy;
pub use publications_handler::publications;

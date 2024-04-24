
mod key;
mod handler;
mod char_sender;

pub use key::Key;
pub use handler::KeypressHandler;
pub use char_sender::CharSender;

pub(crate) use handler::MAGIC_CONST;

pub type Mutex<T> = spin::mutex::Mutex<T, spin::Spin>;
pub(crate) type Lazy<T> = spin::lazy::Lazy<T, fn() -> T, spin::Spin>;

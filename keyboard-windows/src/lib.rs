mod char_sender;
mod handler;
mod key;

pub use char_sender::CharSender;
pub use handler::KeypressHandler;
pub use key::Key;

pub(crate) use handler::MAGIC_CONST;

pub type Mutex<T> = spin::mutex::Mutex<T, spin::Spin>;
pub(crate) type Lazy<T> = spin::lazy::Lazy<T, fn() -> T, spin::Spin>;

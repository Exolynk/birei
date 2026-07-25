// QR-code rendering stays in a small dedicated module so consumers receive a
// single public component without exposing encoder details.
mod view;

pub use view::QrCode;

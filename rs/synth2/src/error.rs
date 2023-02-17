// use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GlicolSynthError {
    ChannelMismatchingError,
}

#[derive(Debug)]
struct ChannelMismatchingError {}

impl fmt::Display for ChannelMismatchingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Each node channel number must match the context channel number.")
    }
}

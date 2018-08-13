use actix;

pub mod console;
#[cfg(feature = "s3-backend")]
pub mod s3;
#[cfg(feature = "statsd-backend")]
pub mod statsd;

use metrics::Measurement;

pub type BackendHandler = actix::Recipient<Message>;

#[derive(Debug, Clone, Message)]
pub enum Message {
    Single(Measurement),
    List(Vec<Measurement>),
}

#[derive(Message)]
pub struct Flush;

impl actix::Message for Measurement {
    type Result = ();
}

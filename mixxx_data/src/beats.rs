#[derive(Clone, PartialEq, ::prost::Message, Copy)]
pub struct Beat {
    #[prost(int32, optional, tag="1")]
    pub frame_position: ::std::option::Option<i32>,
    #[prost(bool, optional, tag="2", default="true")]
    pub enabled: ::std::option::Option<bool>,
    #[prost(enumeration="Source", optional, tag="3", default="Analyzer")]
    pub source: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message, Copy)]
pub struct Bpm {
    #[prost(double, optional, tag="1")]
    pub bpm: ::std::option::Option<f64>,
    #[prost(enumeration="Source", optional, tag="2", default="Analyzer")]
    pub source: ::std::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeatMap {
    #[prost(message, repeated, tag="1")]
    pub beat: ::std::vec::Vec<Beat>,
}
#[derive(Clone, PartialEq, ::prost::Message, Copy)]
pub struct BeatGrid {
    #[prost(message, optional, tag="1")]
    pub bpm: ::std::option::Option<Bpm>,
    #[prost(message, optional, tag="2")]
    pub first_beat: ::std::option::Option<Beat>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Source {
    Analyzer = 0,
    FileMetadata = 1,
    User = 2,
}

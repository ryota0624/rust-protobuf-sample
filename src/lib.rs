pub mod entity;
pub mod repository;

extern crate prost;
#[macro_use]

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/book_club.model.book.rs"));
    include!(concat!(env!("OUT_DIR"), "/book_club.model.club.rs"));
    include!(concat!(env!("OUT_DIR"), "/book_club.model.meeting.rs"));
    include!(concat!(env!("OUT_DIR"), "/book_club.service.user.rs"));
    include!(concat!(env!("OUT_DIR"), "/book_club.service.club.rs"));
    include!(concat!(env!("OUT_DIR"), "/book_club.service.meeting.rs"));
    pub fn out_dir() {
        println!(env!("OUT_DIR"));
    }
}

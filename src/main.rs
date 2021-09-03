use book_club::event_publisher::{
    ConsoleEventPublisher, EventPublisher, InmemoryEventPublisher, ProstEventConverter,
};
use book_club::event_subscriber::{EventSubscriber, ProstMessageEventSubscriber};

fn main() {
    use book_club::entity::user::{User, UserId};
    use book_club::repository::Repository;
    use prost::Message;

    let book = book_club::proto::Book::default();

    let mut buf: Vec<u8> = Vec::with_capacity(18);
    let book_buf = book.encode(&mut buf).unwrap();
    println!("{:?}, {:?}", buf, book_buf);

    let mut buf = &*buf;
    let recovered_book = book_club::proto::Book::decode(&mut buf).unwrap();
    println!("{:?} {:?}", recovered_book, book);

    ConsoleEventPublisher.publish(&book, &ProstEventConverter);

    let i_pub = InmemoryEventPublisher::new();
    let log_for_inmemry: &dyn Fn(&Vec<u8>) = &|bytes: &Vec<u8>| {
        println!("inmemry event publisher published {:?}", bytes);
    };

    let ms = ProstMessageEventSubscriber::new(
        Box::new(|evt: &book_club::proto::Book| {
            println!("ProstMessageEventSubscriber {:?}", evt)
        }));

    let ms_handle: &dyn Fn(&Vec<u8>) = &|bytes: &Vec<u8>| {
        ms.handle(bytes);
    };
    i_pub
        .add_listener(Box::new(log_for_inmemry))
        .add_listener(Box::new(ms_handle))
        .publisher()
        .publish(&book, &ProstEventConverter);

    let repo = book_club::entity::user::UserRepositoryOnMemory::default();
    let run = move || {
        repo.store(User::new(UserId("1".to_string())))?
            .store(User::new(UserId("2".to_string())))
    };

    let repo_r = run();
    match repo_r {
        Ok(repo) => {
            let user1 = repo.find_by_id(&UserId("1".to_string()));
            let user2 = repo.find_by_id(&UserId("2".to_string()));

            println!("{:?}, {:?}", user1, user2)
        }
        Err(reason) => println!("{:?}", reason),
    }
    book_club::proto::out_dir();
}

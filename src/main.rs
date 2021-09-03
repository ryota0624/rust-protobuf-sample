fn main() {
    use book_club::entity::user::{User, UserId};
    use book_club::repository::Repository;
    let b = book_club::proto::Book::default();
    let repo = book_club::entity::user::UserRepositoryOnMemory::new();
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
    println!("Hello, world! {:?}", b);
}

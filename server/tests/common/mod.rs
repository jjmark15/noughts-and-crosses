use lazy_static::lazy_static;

lazy_static! {
    pub static ref GAME_SERVER_HOST: String = {
        match std::env::var("GAME_SERVER_HOST") {
            Ok(val) => val,
            Err(_) => "localhost:3030".to_string(),
        }
    };
}

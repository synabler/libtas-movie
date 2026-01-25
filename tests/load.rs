use libtas_movie::{LoadError, load_movie};

#[test]
fn test_load() -> Result<(), LoadError> {
    load_movie("tests/movies/221769_Trapped_5.ltm")
}

#[test]
fn test_load_not_exist() {
    match load_movie("tests/movies/nope.ltm") {
        Ok(_) => panic!("should have failed to load"),
        Err(LoadError::FileError(err)) => {
            assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
        }
    }
}

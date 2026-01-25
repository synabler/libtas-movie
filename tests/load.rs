use libtas_movie::{LoadError, load_movie};

#[test]
fn test_load() -> Result<(), LoadError> {
    load_movie("tests/movies/221769_Trapped_5.ltm")
}

/// If a file doesn't exist, it should fail with `NotFound`.
#[test]
fn test_load_not_exist() {
    match load_movie("tests/movies/nope.ltm") {
        Err(LoadError::FileError(err)) => {
            assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
        }
        _ => panic!("should have failed to load"),
    }
}

/// If a file is not a .tar.gz file, it should fail with `InvalidArchive`.
#[test]
fn test_load_not_movie() {
    match load_movie("tests/invalid_movies/not_movie.txt") {
        Err(LoadError::InvalidArchive) => {}
        _ => panic!("should have failed to load"),
    }
}

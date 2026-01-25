use std::fs::read_to_string;

use libtas_movie::load::{LoadError, load_movie};

#[test]
fn test_config() {
    let movie = load_movie("tests/movies/221769_Trapped_5.ltm").unwrap();

    // check config
    let general = &movie.config.general;
    assert_eq!(general.authors, "synabler");
    assert_eq!(general.auto_restart, false);
    assert_eq!(general.frame_count, 456);
    assert_eq!(general.framerate_den, 1);
    assert_eq!(general.framerate_num, 20);
    assert_eq!(general.game_name, "ruffle");
    assert_eq!(general.initial_monotonic_time_nsec, 0);
    assert_eq!(general.initial_monotonic_time_sec, 1);
    assert_eq!(general.initial_time_nsec, 0);
    assert_eq!(general.initial_time_sec, 1);
    assert_eq!(general.length_nsec, 800000000);
    assert_eq!(general.length_sec, 22);
    assert_eq!(general.libtas_major_version, 1);
    assert_eq!(general.libtas_minor_version, 4);
    assert_eq!(general.libtas_patch_version, 7);
    assert_eq!(general.md5, "c9b4f1b544725cb0d9d784c35232a52d");
    assert_eq!(general.mouse_support, true);
    assert_eq!(general.nb_controllers, 0);
    assert_eq!(general.rerecord_count, 101);
    assert_eq!(general.savestate_frame_count, 456);
    assert_eq!(general.variable_framerate, false);

    let timetrack = &movie.config.mainthread_timetrack;
    assert_eq!(timetrack.get_tick_count, -1);
    assert_eq!(timetrack.get_tick_count64, -1);
    assert_eq!(timetrack.query_performance_counter, -1);
    assert_eq!(timetrack.clock, -1);
    assert_eq!(timetrack.clock_gettime_monotonic, -1);
    assert_eq!(timetrack.clock_gettime_real, -1);
    assert_eq!(timetrack.gettimeofday, -1);
    assert_eq!(timetrack.sdl_getperformancecounter, -1);
    assert_eq!(timetrack.sdl_getticks, -1);
    assert_eq!(timetrack.time, -1);

    // check Display
    let config_str = read_to_string("tests/movies/221769_Trapped_5_config.ini").unwrap();
    assert_eq!(movie.config.to_string(), config_str);
}

#[test]
fn test_inputs() {
    let movie = load_movie("tests/movies/221769_Trapped_5.ltm").unwrap();

    // check Display
    let config_str = read_to_string("tests/movies/221769_Trapped_5_inputs").unwrap();
    assert_eq!(movie.inputs.to_string(), config_str);
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

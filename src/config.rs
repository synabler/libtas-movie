//! Module that defines a config of a movie file.

use core::{fmt::Display, str::FromStr};

/// An error while parsing a config, containing the string that caused the error.
#[derive(Debug)]
#[expect(dead_code)]
pub struct InvalidConfigError(String);

macro_rules! impl_str_io {
    (
        $struct:ident,
        $group_marker:literal,
        $($key:literal => $field:ident: $type:ty),*
    ) => {
        impl FromStr for $struct {
            type Err = InvalidConfigError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if !s.starts_with($group_marker) {
                    return Err(InvalidConfigError($group_marker.to_owned()));
                }

                let mut config = Self::default();
                for line in s.lines().skip(1) {
                    let Some((key, value)) = line.split_once('=') else {
                        return Err(InvalidConfigError(line.to_owned()));
                    };
                    match key {
                        $(
                            $key => config.$field = value.parse::<$type>().map_err(
                                |_| InvalidConfigError(key.to_owned())
                            )?,
                        )*
                        _ => {}
                    }
                }
                Ok(config)
            }
        }

        impl Display for $struct {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                writeln!(f, $group_marker)?;
                $(
                    writeln!(f, "{}={}", $key, self.$field)?;
                )*
                Ok(())
            }
        }
    };
}

/// `General` config.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GeneralConfig {
    /// Author(s) of the movie.
    pub authors: String,
    /// Whether or not the game automatically restarts after being closed.
    pub auto_restart: bool,
    /// The number of frames in the movie.
    pub frame_count: u64,
    /// Denominator of the framerate.
    /// That is, the framerate is `framerate_num/framerate_den`.
    pub framerate_den: u64,
    /// Numerator of the framerate.
    /// That is, the framerate is `framerate_num/framerate_den`.
    pub framerate_num: u64,
    /// The name of the game executable.
    pub game_name: String,
    /// The fractional part of the initial system time in seconds (monotonic),
    /// expressed as nanoseconds.
    pub initial_monotonic_time_nsec: u64,
    /// The integer part of the initial system time in seconds (monotonic).
    pub initial_monotonic_time_sec: u64,
    /// The fractional part of the initial system time in seconds,
    /// expressed as nanoseconds.
    pub initial_time_nsec: u64,
    /// The integer part of the initial system time in seconds.
    pub initial_time_sec: u64,
    /// The fractional part of the movie length in seconds,
    /// expressed as nanoseconds.
    pub length_nsec: u64,
    /// The integer part of the movie length in seconds.
    pub length_sec: u64,
    /// The major version of libTAS (`a` in `a.b.c`).
    pub libtas_major_version: u32,
    /// The minor version of libTAS (`b` in `a.b.c`).
    pub libtas_minor_version: u32,
    /// The patch version of libTAS (`c` in `a.b.c`).
    pub libtas_patch_version: u32,
    /// The [MD5 hash](https://en.wikipedia.org/wiki/MD5) of the game executable.
    pub md5: String,
    /// Whether or not mouse inputs are sent to the game.
    pub mouse_support: bool,
    /// The number of controllers (up to 4).
    pub nb_controllers: u32,
    /// The number of rerecords.
    pub rerecord_count: u64,
    /// TODO: what does this mean?
    pub savestate_frame_count: u64,
    /// Whether or not the framerate can change in the middle of the movie.
    pub variable_framerate: bool,
}

impl_str_io!(
    GeneralConfig,
    "[General]",
    "authors" => authors: String,
    "auto_restart" => auto_restart: bool,
    "frame_count" => frame_count: u64,
    "framerate_den" => framerate_den: u64,
    "framerate_num" => framerate_num: u64,
    "game_name" => game_name: String,
    "initial_monotonic_time_nsec" => initial_monotonic_time_nsec: u64,
    "initial_monotonic_time_sec" => initial_monotonic_time_sec: u64,
    "initial_time_nsec" => initial_time_nsec: u64,
    "initial_time_sec" => initial_time_sec: u64,
    "length_nsec" => length_nsec: u64,
    "length_sec" => length_sec: u64,
    "libtas_major_version" => libtas_major_version: u32,
    "libtas_minor_version" => libtas_minor_version: u32,
    "libtas_patch_version" => libtas_patch_version: u32,
    "md5" => md5: String,
    "mouse_support" => mouse_support: bool,
    "nb_controllers" => nb_controllers: u32,
    "rerecord_count" => rerecord_count: u64,
    "savestate_frame_count" => savestate_frame_count: u64,
    "variable_framerate" => variable_framerate: bool
);

/// `mainthread_timetrack` config.
/// Each field denotes how many times each function is called
/// before advancing the deterministic timer, with `-1` meaning disabled.
///
/// (TODO) `Default` is wrong, it should be all -1.
/// Or better yet, use `Option<u64>`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TimetrackConfig {
    /// `GetTickCount`
    pub get_tick_count: i64,
    /// `GetTickCount64`
    pub get_tick_count64: i64,
    /// `QueryPerformanceCounter`
    pub query_performance_counter: i64,
    /// `clock`
    pub clock: i64,
    /// `clock_gettime_monotonic`
    pub clock_gettime_monotonic: i64,
    /// `clock_gettime_real`
    pub clock_gettime_real: i64,
    /// `gettimeofday`
    pub gettimeofday: i64,
    /// `sdl_getperformancecounter`
    pub sdl_getperformancecounter: i64,
    /// `sdl_getticks`
    pub sdl_getticks: i64,
    /// `time`
    pub time: i64,
}

impl_str_io!(
    TimetrackConfig,
    "[mainthread_timetrack]",
    "GetTickCount" => get_tick_count: i64,
    "GetTickCount64" => get_tick_count64: i64,
    "QueryPerformanceCounter" => query_performance_counter: i64,
    "clock" => clock: i64,
    "clock_gettime_monotonic" => clock_gettime_monotonic: i64,
    "clock_gettime_real" => clock_gettime_real: i64,
    "gettimeofday" => gettimeofday: i64,
    "sdl_getperformancecounter" => sdl_getperformancecounter: i64,
    "sdl_getticks" => sdl_getticks: i64,
    "time" => time: i64
);

/// Config of a movie.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Config {
    pub general: GeneralConfig,
    pub mainthread_timetrack: TimetrackConfig,
}

impl Display for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "{}", self.general)?;
        write!(f, "{}", self.mainthread_timetrack)
    }
}

impl FromStr for Config {
    type Err = InvalidConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((general, timetrack)) = s.split_once("\n\n") else {
            return Err(InvalidConfigError("not two groups".to_owned()));
        };
        Ok(Self {
            general: general.parse()?,
            mainthread_timetrack: timetrack.parse()?,
        })
    }
}

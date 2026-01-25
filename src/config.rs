use std::str::FromStr;

#[derive(Debug)]
#[expect(dead_code)]
pub struct InvalidConfig(String);

macro_rules! impl_from_str {
    (
        $struct:ident,
        $group_marker:literal,
        $($key:literal => $field:ident: $type:ty),*
        $(, strings: $($str_key:literal => $str_field:ident),*)?
    ) => {
        impl FromStr for $struct {
            type Err = InvalidConfig;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if !s.starts_with($group_marker) {
                    return Err(InvalidConfig($group_marker.to_owned()));
                }

                let mut config = Self::default();
                for line in s.lines().skip(1) {
                    let Some((key, value)) = line.split_once('=') else {
                        continue;
                    };
                    match key {
                        $(
                            $key => config.$field = value.parse::<$type>().map_err(|_| InvalidConfig(key.to_owned()))?,
                        )*
                        $(
                            $(
                                $str_key => config.$str_field = value.to_string(),
                            )*
                        )?
                        _ => {}
                    }
                }
                Ok(config)
            }
        }
    };
}

#[derive(Clone, Debug, Default)]
pub struct GeneralConfig {
    pub authors: String,
    pub auto_restart: bool,
    pub frame_count: u64,
    pub framerate_den: u64,
    pub framerate_num: u64,
    pub game_name: String,
    pub initial_monotonic_time_nsec: u64,
    pub initial_monotonic_time_sec: u64,
    pub initial_time_nsec: u64,
    pub initial_time_sec: u64,
    pub length_nsec: u64,
    pub length_sec: u64,
    pub libtas_major_version: u32,
    pub libtas_minor_version: u32,
    pub libtas_patch_version: u32,
    pub md5: String,
    pub mouse_support: bool,
    pub nb_controllers: u32,
    pub rerecord_count: u64,
    pub savestate_frame_count: u64,
    pub variable_framerate: bool,
}

impl_from_str!(
    GeneralConfig,
    "[General]",
    "auto_restart" => auto_restart: bool,
    "frame_count" => frame_count: u64,
    "framerate_den" => framerate_den: u64,
    "framerate_num" => framerate_num: u64,
    "initial_monotonic_time_nsec" => initial_monotonic_time_nsec: u64,
    "initial_monotonic_time_sec" => initial_monotonic_time_sec: u64,
    "initial_time_nsec" => initial_time_nsec: u64,
    "initial_time_sec" => initial_time_sec: u64,
    "length_nsec" => length_nsec: u64,
    "length_sec" => length_sec: u64,
    "libtas_major_version" => libtas_major_version: u32,
    "libtas_minor_version" => libtas_minor_version: u32,
    "libtas_patch_version" => libtas_patch_version: u32,
    "mouse_support" => mouse_support: bool,
    "nb_controllers" => nb_controllers: u32,
    "rerecord_count" => rerecord_count: u64,
    "savestate_frame_count" => savestate_frame_count: u64,
    "variable_framerate" => variable_framerate: bool,
    strings:
    "authors" => authors,
    "game_name" => game_name,
    "md5" => md5
);

#[derive(Clone, Debug, Default)]
pub struct TimetrackConfig {
    pub get_tick_count: i64,
    pub get_tick_count64: i64,
    pub query_performance_counter: i64,
    pub clock: i64,
    pub clock_gettime_monotonic: i64,
    pub clock_gettime_real: i64,
    pub gettimeofday: i64,
    pub sdl_getperformancecounter: i64,
    pub sdl_getticks: i64,
    pub time: i64,
}

impl_from_str!(
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

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub general: GeneralConfig,
    pub mainthread_timetrack: TimetrackConfig,
}

impl FromStr for Config {
    type Err = InvalidConfig;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((general, timetrack)) = s.split_once("\n\n") else {
            return Err(InvalidConfig("not two groups".to_owned()));
        };
        Ok(Self {
            general: general.parse()?,
            mainthread_timetrack: timetrack.parse()?,
        })
    }
}

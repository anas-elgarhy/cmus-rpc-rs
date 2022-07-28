use crate::args;
use crate::config::ConfigFile;

pub struct Config {
    pub debug: bool,
    pub link: bool,
    pub config_path: String,
    pub interval: u32,
    pub sleep: u32,
    pub part_one_format: String,
    pub part_two_format: String,
}

impl Config {
    pub fn new() -> Config {
        let args_matches = args::get_args_app().get_matches();
        let parsed_config = match ConfigFile::load() {
            Ok(config) => config,
            Err(e) => {
                println!("Error loading config_path file: {}", e);
                ConfigFile::create_default()
            }
        };
        let mut configs = Config {
            debug: args_matches.is_present("debug") || parsed_config.debug,
            link: args_matches.is_present("link") || parsed_config.link,
            config_path: if args_matches.is_present("config_path") {
                args_matches
                    .value_of("config_path")
                    .unwrap()
                    .to_string()
            } else {
                parsed_config.config_path
            },
            interval: if args_matches.is_present("interval") {
                args_matches
                    .value_of("interval")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
            } else {
                parsed_config.interval
            },
            sleep: if args_matches.is_present("sleep") {
                args_matches
                    .value_of("sleep")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
            } else {
                parsed_config.sleep
            },
            part_one_format: if args_matches.is_present("part_one_format") {
                args_matches
                    .value_of("part_one_format")
                    .unwrap()
                    .to_string()
            } else {
                parsed_config.part_one_format
            },
            part_two_format: if args_matches.is_present("part_two_format") {
                args_matches
                    .value_of("part_two_format")
                    .unwrap()
                    .to_string()
            } else {
                parsed_config.part_two_format
            },
        };

        configs
    }

    pub fn default() -> Config {
        let config_path = format!("{}/{}", dirs::config_dir()
            .unwrap_or_default()
            .to_str()
            .unwrap(), "cmus-rps-rs/config.conf");
        Config {
            debug: false,
            link: false,
            config_path: config_path.to_string(),
            interval: 1000,
            sleep: 5000,
            part_one_format: "{artist} - {title}".to_string(),
            part_two_format: "{album}".to_string(),
        }
    }
}
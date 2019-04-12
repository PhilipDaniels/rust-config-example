use std::path::{Path, PathBuf};
use std::{io, fs, env};

use clap::{Arg, App, ArgMatches};
use dirs;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use serde_json;

lazy_static! {
    // Global configuration variable.
    // Lazy-static works by creating one-off types that wrap your value and provide
    // thread-safe single initialization guarantees:
    //     For a given static ref NAME: TYPE = EXPR;,
    // the macro generates a unique type that implements Deref<TYPE> and stores it
    // in a static with name NAME. This wrapper type does not implement your trait,
    // (e.g. Debug) only the wrapped type does. Hence to print using the Debug
    // format, use `*CONFIGURATION`.
    pub static ref CONFIGURATION: Configuration = Configuration::initialize();
}



/// Terminology
///   * Options - command line options
///   * FileConfiguration - configuration read from files
///   * Configuration - the final, app-level configuration object


/// Represents the final, global configuration of the program.
/// Mainly immutable, uses interior immutability where necessary.
/// This struct is the combination of
/// - default configuration
/// - plus overrides read from configuration file(s)
/// - plus command-line options
/// In that order.
#[derive(Debug)]
pub struct Configuration {
    pub verbose: bool,
    pub directory: PathBuf
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            verbose: false,
            directory: "".into()
        }
    }
}

impl Configuration {
    fn initialize() -> Self {
        // Get the default configuration, this is what will apply if nothing
        // else is specified.
        let mut config = Configuration::default();

        // First get the command line options. If these are wrong we don't even need to
        // bother loading the configuration file. Clap will exit automatically and the
        // program will terminate.
        // ALTS: Create our own `Options` type and/or use StructOpt.
        let arg_matches = Self::get_command_line_options();
        if arg_matches.is_present("verbose") {
            config.verbose = true;
        }

        // Code after this is basically merging in other settings.
        // It is only an example, adjust to taste.

        // From some arbitrary directory, e.g. as specified in the matches.
        config.merge_file_config("/home/phil/somewhere");

        // Look for a config file in the same directory as the exe.
        if let Ok(exe_path) = env::current_exe() {
            config.merge_file_config(exe_path.parent().unwrap());
        }

        // Look for a config file in the home dir.
        if let Some(home_dir) = dirs::home_dir() {
            config.merge_file_config(home_dir.parent().unwrap());
        }

        config
    }

    fn merge_file_config<D>(&mut self, directory: D)
    where D: Into<PathBuf>
    {
        const CONFIG_FILE: &str = ".myprog.json";

        let mut config_file_path = directory.into();
        config_file_path.push(CONFIG_FILE);
        if let Some(file_config) = FileConfiguration::load_from_file(&config_file_path) {
            self.merge(&file_config);
        }

    }

    fn merge(&mut self, _file_config: &FileConfiguration) {
        // Apply elements from file_config over the top of self.
    }

    fn get_command_line_options() -> ArgMatches<'static> {
        let matches = App::new("My Super Program")
                            .version("1.0")
                            .author("philip.daniels1971@gmail.com>")
                            .about("Does awesome things")
                            .arg(Arg::with_name("config")
                                .short("c")
                                .long("config")
                                .value_name("FILE")
                                .help("Sets a custom config file")
                                .takes_value(true))
                            .arg(Arg::with_name("INPUT")
                                .help("Sets the input file to use")
                                .required(true)
                                .index(1))
                            .arg(Arg::with_name("v")
                                .short("v")
                                .multiple(true)
                                .help("Sets the level of verbosity"))
                            .get_matches();

        matches
    }
}



/// Configuration as stored in a config file. Note the serde attributes.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct FileConfiguration {
    verbose: bool,
    names: Vec<String>,
}

impl FileConfiguration {
    /// Either successfully loads a file, or returns a None if it does not exist.
    /// Panics in the case of a bad file. Adjust as you see fit.
    fn load_from_file<P>(path: P) -> Option<Self>
    where P: AsRef<Path>
    {
        match fs::File::open(path) {
            Ok(f) => match serde_json::from_reader(f) {
                Ok(r) => Some(r),
                Err(e) => { eprintln!("Could not parse JSON {:?}", e); None },
            },
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => None,
            Err(e) => panic!("Error opening config file {:?}", e)
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    /// Writes the default settings to stdout. Useful for creating an initial config file.
    /// Normally you would implement Default yourself or convert between Configuration
    /// and FileConfiguration.
    #[allow(unused)]
    pub fn dump_defaults() {
        use std::io::Write;

        let serialized = Self::default().to_string();
        println!("{}", serialized);
        io::stdout().flush().unwrap();
    }
}


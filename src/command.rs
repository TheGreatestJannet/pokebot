use structopt::clap::AppSettings::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    rename_all = "kebab-case",
    template = "{subcommands}",
    raw(global_settings = "&[VersionlessSubcommands,
                            DisableHelpFlags,
                            DisableVersion,
                            ColorNever,
                            NoBinaryName]",)
)]
pub enum Command {
    /// Adds url to playlist
    Add { url: String },
    /// Starts audio playback
    Play,
    /// Pauses audio playback
    Pause,
    /// Stops audio playback
    Stop,
    /// Switches to the next queue entry
    #[structopt(alias = "skip")]
    Next,
    /// Clears the playback queue
    Clear,
    /// Changes the volume to the specified value
    Volume { percent: f64 },
}

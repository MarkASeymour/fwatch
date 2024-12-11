
use argh::FromArgs;

#[derive(FromArgs)]
/// FWatch file watching tool for batch orchestration
pub struct Args {
    /// full directory path where to look for file
    #[argh(positional)]
    pub path: String,
    /// the file name to look for, can include wildcards
    #[argh(positional)]
    pub pattern: String,
    /// the number of days to run for
    #[argh(positional)]
    pub rundays: i32,
    /// the time to end at in 24HR in follow format "15:00:00" for 3 pm in host timezone
    #[argh(positional)]
    pub endtime: String,
    /// String time zone. If not set then UTC is used. IANA format. for example-> America/New_York
    #[argh(option, short = 'z', default = "Etc/UTC")]
    pub timezone: String
}

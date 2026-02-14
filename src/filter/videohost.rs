use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum VideoHost {
    Streamwo,
    Streamja,
    Streamye,
    Streamable,
    Imgtc,
    Clippituser,
    Vimeo,
    Streamvi,
    Juststream,
    Streamff,
    Streamgg,
    Streamin,
    Streamain,
    Dubz,
    Streambug,
}
#[derive(Debug, PartialEq, Eq)]
pub struct UnkownVideoHostError;

impl FromStr for VideoHost {
    type Err = UnkownVideoHostError;
    fn from_str(input: &str) -> Result<VideoHost, Self::Err> {
        let mut owned_string = input.to_owned().to_lowercase();
        owned_string = owned_string.replace("http://", "");
        owned_string = owned_string.replace("https://", "");
        owned_string = owned_string.replace("www.", "");
        let domain = owned_string.split(".").collect::<Vec<&str>>()[0];

        match domain {
            "streamwo" => Ok(VideoHost::Streamwo),
            "streamja" => Ok(VideoHost::Streamja),
            "streamye" => Ok(VideoHost::Streamye),
            "streamable" => Ok(VideoHost::Streamable),
            "imgtc" => Ok(VideoHost::Imgtc),
            "clippituser" => Ok(VideoHost::Clippituser),
            "vimeo" => Ok(VideoHost::Vimeo),
            "streamvi" => Ok(VideoHost::Streamvi),
            "juststream" => Ok(VideoHost::Juststream),
            "streamff" => Ok(VideoHost::Streamff),
            "streamgg" => Ok(VideoHost::Streamgg),
            "streamin" => Ok(VideoHost::Streamin),
            "dubz" => Ok(VideoHost::Dubz),
            "streambug" => Ok(VideoHost::Streambug),
            "streamain" => Ok(VideoHost::Streamain),
            _ => Err(UnkownVideoHostError),
        }
    }
}

impl fmt::Display for VideoHost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VideoHost::Streamwo => write!(f, "streamwo"),
            VideoHost::Streamja => write!(f, "streamja"),
            VideoHost::Streamye => write!(f, "streamye"),
            VideoHost::Streamable => write!(f, "streamable"),
            VideoHost::Imgtc => write!(f, "imgtc"),
            VideoHost::Clippituser => write!(f, "clippituser"),
            VideoHost::Vimeo => write!(f, "vimeo"),
            VideoHost::Streamvi => write!(f, "streamvi"),
            VideoHost::Juststream => write!(f, "juststream"),
            VideoHost::Streamff => write!(f, "streamff"),
            VideoHost::Streamgg => write!(f, "streamgg"),
            VideoHost::Streamin => write!(f, "streamin"),
            VideoHost::Streamain => write!(f, "streamain"),
            VideoHost::Dubz => write!(f, "dubz"),
            VideoHost::Streambug => write!(f, "streambug"),
        }
    }
}

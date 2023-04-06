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
    Dubz,
}

impl FromStr for VideoHost {
    type Err = ();
    fn from_str(input: &str) -> Result<VideoHost, Self::Err> {
        match input {
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
            _ => Err(()),
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
            VideoHost::Dubz => write!(f, "dubz"),
        }
    }
}

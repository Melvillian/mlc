extern crate url;

use self::url::Url;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum LinkType {
    HTTP,
    FTP,
    Mail,
    FileSystem,
    UnknownUrlSchema,
}

pub fn get_link_type(link: &str) -> Option<LinkType> {
    lazy_static! {
        static ref FILE_SYSTEM_REGEX: Regex =
            Regex::new(r"^(([[:alpha:]]:(\\|/))|(..?(\\|/))|((\\\\?|//?))).*").unwrap();
    }

    if FILE_SYSTEM_REGEX.is_match(link) || !link.contains(':') {
        if link.contains('@') {
            return Some(LinkType::Mail);
        } else {
            return Some(LinkType::FileSystem);
        }
    }

    if let Ok(url) = Url::parse(&link) {
        let scheme = url.scheme();
        debug!("Link {} is a URL type with scheme {}", link, scheme);
        match scheme {
            "http" => return Some(LinkType::HTTP),
            "https" => return Some(LinkType::HTTP),
            "ftp" => return Some(LinkType::FTP),
            "ftps" => return Some(LinkType::FTP),
            "mailto" => return Some(LinkType::Mail),
            "file" => return Some(LinkType::FileSystem),
            _ => return Some(LinkType::UnknownUrlSchema),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use ntest::test_case;

    fn test_link(link: &str, expected_type: &LinkType) {
        let link_type =
            get_link_type(link).expect(format!("Unknown link type for link {}", link).as_str());
        assert_eq!(link_type, *expected_type);
    }

    #[test_case("https://doc.rust-lang.org.html")]
    #[test_case("http://www.website.php")]
    fn http_link_types(link: &str) {
        test_link(link, &LinkType::HTTP);
    }

    #[test_case("ftp://mueller:12345@ftp.downloading.ch")]
    fn ftp_link_types(ftp: &str) {
        test_link(ftp, &LinkType::FTP);
    }

    #[test_case("F:/fake/windows/paths")]
    #[test_case("\\\\smb}\\paths")]
    #[test_case("C:\\traditional\\paths")]
    #[test_case("\\file.ext")]
    #[test_case("file:///some/path/")]
    #[test_case("path")]
    #[test_case("./file.ext")]
    #[test_case(".\\file.md")]
    #[test_case("../upper_dir.md")]
    #[test_case("..\\upper_dir.mdc")]
    #[test_case("D:\\Program Files(x86)\\file.log")]
    #[test_case("D:\\Program Files(x86)\\folder\\file.log")]
    fn test_file_system_link_types(link: &str) {
        test_link(link, &LinkType::FileSystem);
    }
}

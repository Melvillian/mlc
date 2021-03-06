use std::str::FromStr;

#[derive(Debug)]
pub struct MarkupFile {
    pub markup_type: MarkupType,
    pub path: String,
}

#[derive(Debug, Clone)]
pub enum MarkupType {
    Markdown,
    HTML,
}

impl FromStr for MarkupType {
    type Err = ();

    fn from_str(s: &str) -> Result<MarkupType, ()> {
        match s {
            "md" => Ok(MarkupType::Markdown),
            "html" => Ok(MarkupType::HTML),
            _ => Err(()),
        }
    }
}

impl MarkupType {
    pub fn file_extensions(&self) -> Vec<String> {
        match self {
            MarkupType::Markdown => vec![
                "md".to_string(),
                "markdown".to_string(),
                "mkdown".to_string(),
                "mkdn".to_string(),
                "mkd".to_string(),
                "mdwn".to_string(),
                "mdtxt".to_string(),
                "mdtext".to_string(),
                "text".to_string(),
                "rmd".to_string(),
            ],
            MarkupType::HTML => vec!["html".to_string(), "xhtml".to_string()],
        }
    }
}

use crate::Fields;
use std::{collections::BTreeMap, str::FromStr};

macro_rules! field {
    ($map:expr, $field:expr, $enum:expr) => {{
        if let Some(v) = $field {
            $map.insert($enum, v);
        }
    }};
}

#[derive(Hash, Debug, Clone, Copy, Ord, PartialEq, PartialOrd, Eq)]
pub enum FlacTags {
    Title,
    Version,
    Album,
    Tracknumber,
    Artist,
    Performer,
    Copyright,
    License,
    Organization,
    Description,
    Genre,
    Date,
    Location,
    Contact,
    Isrc,
}
impl FlacTags {
    pub fn from_args(f: Fields) -> BTreeMap<FlacTags, Vec<String>> {
        let mut map = BTreeMap::new();
        use FlacTags::*;

        field!(map, f.title, Title);
        field!(map, f.version, Version);
        field!(map, f.album, Album);
        field!(map, f.tracknumber, Tracknumber);
        field!(map, f.artist, Artist);
        field!(map, f.performer, Performer);
        field!(map, f.copyright, Copyright);
        field!(map, f.license, License);
        field!(map, f.organization, Organization);
        field!(map, f.description, Description);
        field!(map, f.genre, Genre);
        field!(map, f.date, Date);
        field!(map, f.location, Location);
        field!(map, f.contact, Contact);
        field!(map, f.isrc, Isrc);

        map
    }

    pub fn as_str(&self) -> &str {
        use FlacTags::*;
        match self {
            Title => "TITLE",
            Version => "VERSION",
            Album => "ALBUM",
            Tracknumber => "TRACKNUMBER",
            Artist => "ARTIST",
            Performer => "PERFORMER",
            Copyright => "COPYRIGHT",
            License => "LICENSE",
            Organization => "ORGANIZATION",
            Description => "DESCRIPTION",
            Genre => "GENRE",
            Date => "DATE",
            Location => "LOCATION",
            Contact => "CONTACT",
            Isrc => "ISRC",
        }
    }
}
impl FromStr for FlacTags {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FlacTags::*;
        let s = s.to_uppercase();

        // check for illegal characters
        [':'].into_iter().try_for_each(|c| {
            if s.find(c).is_some() {
                Err(format!(
                    "Attempted to use tag `{}` that contained illegal character `{}`",
                    s, c
                ))
            } else {
                Ok(())
            }
        })?;

        // map to enum variant
        Ok(match s.as_str() {
            "TITLE" => Title,
            "VERSION" => Version,
            "ALBUM" => Album,
            "TRACKNUMBER" => Tracknumber,
            "ARTIST" => Artist,
            "PERFORMER" => Performer,
            "COPYRIGHT" => Copyright,
            "LICENSE" => License,
            "ORGANIZATION" => Organization,
            "DESCRIPTION" => Description,
            "GENRE" => Genre,
            "DATE" => Date,
            "LOCATION" => Location,
            "CONTACT" => Contact,
            "ISRC" => Isrc,
            _ => Err(s)?,
        })
    }
}

use std::str::FromStr;

use crate::Fields;

#[derive(Hash, Debug, Clone, Copy)]
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
    pub fn from_args(f: Fields) -> Vec<(FlacTags, Vec<String>)> {
        let mut vec = Vec::new();
        use FlacTags::*;

        field!(vec, f.title, Title);
        field!(vec, f.version, Version);
        field!(vec, f.album, Album);
        field!(vec, f.tracknumber, Tracknumber);
        field!(vec, f.artist, Artist);
        field!(vec, f.performer, Performer);
        field!(vec, f.copyright, Copyright);
        field!(vec, f.license, License);
        field!(vec, f.organization, Organization);
        field!(vec, f.description, Description);
        field!(vec, f.genre, Genre);
        field!(vec, f.date, Date);
        field!(vec, f.location, Location);
        field!(vec, f.contact, Contact);
        field!(vec, f.isrc, Isrc);

        vec
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

// impl FlacTags {
//     fn info(&self) -> &str {
//         use FlacTags::*;
//         match self {
//             Title => "Track/Work name",
//             Version => "The version field may be used to differentiate multiple versions of the same track title in a single collection. (e.g. remix info)",
//             Album => "The collection name to which this track belongs",
//             Tracknumber => "The track number of this piece if part of a specific larger collection or album",
//             Artist => "The artist generally considered responsible for the work. In popular music this is usually the performing band or singer. For classical music it would be the composer. For an audio book it would be the author of the original text.",
//             Performer => "The artist(s) who performed the work. In classical music this would be the conductor, orchestra, soloists. In an audio book it would be the actor who did the reading. In popular music this is typically the same as the ARTIST and is omitted.",
//             Copyright => "Copyright attribution, e.g., '2001 Nobody's Band' or '1999 Jack Moffitt'",
//             License => "License information, for example, 'All Rights Reserved', 'Any Use Permitted', a URL to a license such as a Creative Commons license (e.g. \"creativecommons.org/licenses/by/4.0/\"), or similar.",
//             Organization => "Name of the organization producing the track (i.e. the 'record label')",
//             Description => "A short text description of the contents",
//             Genre => "A short text indication of music genre",
//             Date => "Date the track was recorded",
//             Location => "Location where track was recorded",
//             Contact => "Contact information for the creators or distributors of the track. This could be a URL, an email address, the physical address of the producing label.",
//             Isrc => "ISRC number for the track; see the ISRC intro page for more information on ISRC numbers.",
//             _ => "Any other custom tags.",
//         }
//     }
// }

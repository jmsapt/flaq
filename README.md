# Flaq
- [Installation](#Installation)
- [Usage](#Usage)
- [Queries](#Queries)
- [The Standard](#The-Standard)


## Installation

## Usage

## Queries

## The Standard
The standard, listed below, is not super rigid with room for ambiguity. This program follow this standard
completely with the following exception;
- the fields `Date` and `Tracknumber` will support any number of arguments, however the 0th tag must be match the following 
formats
  - `Date` must be given as `YYYY`, `YYYY-MM`, or `YYYY-MM-DD`. For the purpose of comparisons the more specific dates is 
  truncated to match the less specific dates (i.e. comparing `YYYY-MM-DD` and `YYYY` will truncate the first date so that the
  result of the operation is just comparing the years)
  - `Tracknumber` must be able to be parsed as an `u32` (unsigned 32-bit integer). It is up to the user to decide whether to 
  index tracks from `1` or `0` (however it suggested to index from `1` for consistency with the real track numbers).
- Only the 0th `Date`/`Tracknumber` will be used for comparison (and hence must match the above requirements). Any other
tags can optionally be included to give greater context.


[Vorbis Comment Standard (Flac Metadata Tags)](https://www.xiph.org/vorbis/doc/v-comment.html)

> ### Field names
> 
> Below is a proposed, minimal list of standard field names with a description of intended use. No single or group of field names is mandatory; a comment header may contain one, all or none of the names in this list.
> 
> - TITLE
>   - Track/Work name
> - VERSION
>   - The version field may be used to differentiate multiple versions of the same track title in a single collection. (e.g. remix info)
> - ALBUM
>   - The collection name to which this track belongs
> - TRACKNUMBER
>   - The track number of this piece if part of a specific larger collection or album
> - ARTIST
>   - The artist generally considered responsible for the work. In popular music this is usually the performing band or singer. For classical music it would be the composer. For an audio book it would be the author of the original text.
> - PERFORMER
>   - The artist(s) who performed the work. In classical music this would be the conductor, orchestra, soloists. In an audio book it would be the actor who did the reading. In popular music this is typically the same as the ARTIST and is omitted.
> - COPYRIGHT
>   - Copyright attribution, e.g., '2001 Nobody's Band' or '1999 Jack Moffitt'
> - LICENSE
>   - License information, for example, 'All Rights Reserved', 'Any Use Permitted', a URL to a license such as a Creative Commons license (e.g. "creativecommons.org/licenses/by/4.0/"), or similar.
> - ORGANIZATION
>   - Name of the organization producing the track (i.e. the 'record label')
> - DESCRIPTION
>   - A short text description of the contents
> - GENRE
>   - A short text indication of music genre
> - DATE
>   - Date the track was recorded
> - LOCATION
>   - Location where track was recorded
> - CONTACT
>   - Contact information for the creators or distributors of the track. This could be a URL, an email address, the physical address of the producing label.
> - ISRC
>   - ISRC number for the track; see the ISRC intro page for more information on ISRC numbers.
> 
> ### Implications
> Field names should not be 'internationalized'; this is a concession to simplicity not an attempt to exclude the majority of the world that doesn't speak English. Field contents, however, use the UTF-8 character encoding to allow easy representation of any language.
> We have the length of the entirety of the field and restrictions on the field name so that the field name is bounded in a known way. Thus we also have the length of the field contents.
> Individual 'vendors' may use non-standard field names within reason. The proper use of comment fields should be clear through context at this point. Abuse will be discouraged.
> There is no vendor-specific prefix to 'nonstandard' field names. Vendors should make some effort to avoid arbitrarily polluting the common namespace. We will generally collect the more useful tags here to help with standardization.
> Field names are not required to be unique (occur once) within a comment header. As an example, assume a track was recorded by three well know artists; the following is permissible, and encouraged:


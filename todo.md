# To-Do List
## Modifying Tags
- clean duplicated tags (-c/--clean)
- modify tags (replacement and appending)
- replace all instances of a specific tags

## Query Tags
- query (return list of files matching the query) (lower case `q` for case sensitive, and uppercase `Q` for case insensitive)
  - support operations like AND, OR, NOT, XOR, GR, LT, GE, LE, EQ, IN, CON (Same as in, but matches substrings)
  - dates interpreted numerically (YYYY, YYYY-MM-DD)
  - can operate of over the following env variables; title etc. (Tag enum), filename, path
  - all string inputs must be captured in either ("") or in ('').
  - formulated as a finite state machine
  - any queries where a requried env is not set are skipped, silently unless `-e` flag is set
- verbose mode (include list tags with matching files)
- print collection of all tags (recursively)

## Comments on the standard
- standard allows for n-many values for each field (i.e. 3 artists, 2 albums,). Many field likely don't make sense to have multiple
  entries (like location)
- non-standard behaviour only allowing 1 entry per numeric field (track number, and date).
- genre is derived from the genre enum (which includes `other` for non-standard genres).
- all other fields are strings (uft8-encoded)
- track number is to be assumed as only positive integers

## Problems for later
- handling ISRC numbers

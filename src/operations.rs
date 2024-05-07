use std::collections::HashSet;

use metaflac::Tag;

fn clean(tag: &mut Tag) {}
fn delete(tag: &mut Tag, tags: HashSet<Tag>) {}

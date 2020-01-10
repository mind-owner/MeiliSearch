#[cfg(test)]
#[macro_use]
extern crate assert_matches;

mod automaton;
mod bucket_sort;
mod database;
mod distinct_map;
mod error;
mod levenshtein;
mod number;
mod query_builder;
mod query_tree;
mod query_words_mapper;
mod ranked_map;
mod raw_document;
mod reordered_attrs;
mod update;
// mod fields_map;
pub mod settings;
pub mod criterion;
pub mod raw_indexer;
pub mod serde;
pub mod store;

pub use self::database::{BoxUpdateFn, Database, MainT, UpdateT};
pub use self::error::{Error, MResult};
pub use self::number::{Number, ParseNumberError};
pub use self::ranked_map::RankedMap;
// pub use self::fields_map::FieldsMap;
pub use self::raw_document::RawDocument;
pub use self::store::Index;
pub use self::update::{EnqueuedUpdateResult, ProcessedUpdateResult, UpdateStatus, UpdateType};
pub use meilisearch_types::{DocIndex, DocumentId, Highlight};
pub use query_words_mapper::QueryWordsMapper;

use std::convert::TryFrom;
use std::collections::HashMap;
use compact_arena::SmallArena;

use crate::bucket_sort::PostingsListView;
use crate::levenshtein::prefix_damerau_levenshtein;
use crate::query_tree::{QueryId, QueryKind};
use crate::reordered_attrs::ReorderedAttrs;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Document {
    pub id: DocumentId,
    pub highlights: Vec<Highlight>,

    #[cfg(test)]
    pub matches: Vec<crate::bucket_sort::SimpleMatch>,
}

fn highlights_from_raw_document<'a, 'tag, 'txn>(
    raw_document: &RawDocument<'a, 'tag>,
    queries_kinds: &HashMap<QueryId, &QueryKind>,
    arena: &SmallArena<'tag, PostingsListView<'txn>>,
    searchable_attrs: Option<&ReorderedAttrs>,
) -> Vec<Highlight>
{
    let mut highlights = Vec::new();

    for bm in raw_document.bare_matches.iter() {
        let postings_list = &arena[bm.postings_list];
        let input = postings_list.input();
        let kind = &queries_kinds.get(&bm.query_index);

        for di in postings_list.iter() {
            let covered_area = match kind {
                Some(QueryKind::NonTolerant(query)) | Some(QueryKind::Tolerant(query)) => {
                    let len = if query.len() > input.len() {
                        input.len()
                    } else {
                        prefix_damerau_levenshtein(query.as_bytes(), input).1
                    };
                    u16::try_from(len).unwrap_or(u16::max_value())
                },
                _ => di.char_length,
            };

            let attribute = searchable_attrs
                .and_then(|sa| sa.reverse(di.attribute))
                .unwrap_or(di.attribute);

            let highlight = Highlight {
                attribute: attribute,
                char_index: di.char_index,
                char_length: covered_area,
            };

            highlights.push(highlight);
        }
    }

    highlights
}

impl Document {
    #[cfg(not(test))]
    pub fn from_highlights(id: DocumentId, highlights: &[Highlight]) -> Document {
        Document { id, highlights: highlights.to_owned() }
    }

    #[cfg(test)]
    pub fn from_highlights(id: DocumentId, highlights: &[Highlight]) -> Document {
        Document { id, highlights: highlights.to_owned(), matches: Vec::new() }
    }

    #[cfg(not(test))]
    pub fn from_raw<'a, 'tag, 'txn>(
        raw_document: RawDocument<'a, 'tag>,
        queries_kinds: &HashMap<QueryId, &QueryKind>,
        arena: &SmallArena<'tag, PostingsListView<'txn>>,
        searchable_attrs: Option<&ReorderedAttrs>,
    ) -> Document
    {
        let highlights = highlights_from_raw_document(
            &raw_document,
            queries_kinds,
            arena,
            searchable_attrs,
        );

        Document { id: raw_document.id, highlights }
    }

    #[cfg(test)]
    pub fn from_raw<'a, 'tag, 'txn>(
        raw_document: RawDocument<'a, 'tag>,
        queries_kinds: &HashMap<QueryId, &QueryKind>,
        arena: &SmallArena<'tag, PostingsListView<'txn>>,
        searchable_attrs: Option<&ReorderedAttrs>,
    ) -> Document
    {
        use crate::bucket_sort::SimpleMatch;

        let highlights = highlights_from_raw_document(
            &raw_document,
            queries_kinds,
            arena,
            searchable_attrs,
        );

        let mut matches = Vec::new();
        for sm in raw_document.processed_matches {
            let attribute = searchable_attrs
                .and_then(|sa| sa.reverse(sm.attribute))
                .unwrap_or(sm.attribute);

            matches.push(SimpleMatch { attribute, ..sm });
        }
        matches.sort_unstable();

        Document { id: raw_document.id, highlights, matches }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn docindex_mem_size() {
        assert_eq!(mem::size_of::<DocIndex>(), 16);
    }
}

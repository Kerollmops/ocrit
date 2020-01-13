use std::cmp::Reverse;
use std::mem;
use compact_arena::{mk_arena, SmallArena, Idx32};
use rand::{Rng, SeedableRng, rngs::StdRng};
use sdset::{Set, SetBuf};
use slice_group_by::{GroupBy, GroupByMut};

use crate::typo::Typo;
use crate::words::Words;
use crate::proximity::Proximity;
use crate::attribute::Attribute;

mod typo;
mod words;
mod proximity;
mod attribute;

trait Criterion {
    fn name(&self) -> &str;
    fn order(&self) -> Order;

    fn evaluate<'a, 'tag>(
        &self,
        postings_lists: &SmallArena<'tag, SetBuf<DocIndex>>,
        document: &RawDocument<'a, 'tag>,
    ) -> usize;
}

#[derive(Debug, Clone, Copy)]
enum Order {
    /// Lower is better
    Asc,
    /// Bigger is better
    Dsc,
}

type DocumentId = u32;

struct BareMatch<'tag> {
    document_id: DocumentId,
    query_index: u16,
    distance: u8,
    is_exact: bool,
    postings_list: Idx32<'tag>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
struct DocIndex {
    document_id: DocumentId,
    attribute: u16,
    word_index: u16,
}

struct RawDocument<'a, 'tag> {
    id: DocumentId,
    bare_matches: &'a [BareMatch<'tag>],
}

fn main() {
    mk_arena!(arena);

    // new york
    let mut bare_matches = vec![
        // new
        BareMatch { document_id: 0, query_index: 0, distance: 0, is_exact: true,
            postings_list: arena.add(SetBuf::from_dirty(vec![
                DocIndex { document_id: 0, attribute: 0, word_index: 0  },
                DocIndex { document_id: 0, attribute: 0, word_index: 5  },
                DocIndex { document_id: 0, attribute: 1, word_index: 9  },
                DocIndex { document_id: 0, attribute: 1, word_index: 10 },
            ]))
        },
        // york
        BareMatch { document_id: 0, query_index: 1, distance: 0, is_exact: true,
            postings_list: arena.add(SetBuf::from_dirty(vec![
                DocIndex { document_id: 0, attribute: 0, word_index: 6 },
                DocIndex { document_id: 0, attribute: 2, word_index: 0 },
            ]))
        },

        // new
        BareMatch { document_id: 1, query_index: 0, distance: 0, is_exact: true,
            postings_list: arena.add(SetBuf::from_dirty(vec![
                DocIndex { document_id: 1, attribute: 0, word_index: 0 },
                DocIndex { document_id: 1, attribute: 0, word_index: 5 },
            ]))
        },
        // york
        BareMatch { document_id: 1, query_index: 1, distance: 0, is_exact: true,
            postings_list: arena.add(SetBuf::from_dirty(vec![
                DocIndex { document_id: 1, attribute: 0, word_index: 1 },
                DocIndex { document_id: 1, attribute: 0, word_index: 4 },
            ]))
        },

        // new
        BareMatch { document_id: 2, query_index: 0, distance: 0, is_exact: true,
            postings_list: arena.add(SetBuf::from_dirty(vec![
                DocIndex { document_id: 2, attribute: 1, word_index: 6 },
        DocIndex { document_id: 2, attribute: 2, word_index: 4 },
            ]))
         },
    ];

    bare_matches.sort_unstable_by_key(|bm| bm.document_id);

    let mut raw_documents = Vec::new();
    for bare_matches in bare_matches.linear_group_by_key(|bm| bm.document_id) {
        let id = bare_matches[0].document_id;
        raw_documents.push(RawDocument { id, bare_matches });
    }

    // ---------------------

    let criteria: &[&dyn Criterion] = &[
        &Typo,
        &Words,
        &Proximity,
        &Attribute,
    ];

    let mut groups = vec![raw_documents.as_mut_slice()];

    for criterion in criteria {

        println!("{:?}", criterion.name());
        let prev_groups = mem::take(&mut groups);

        for group in prev_groups {
            match criterion.order() {
                Order::Asc => group.sort_unstable_by_key(|d| criterion.evaluate(&arena, d)),
                Order::Dsc => group.sort_unstable_by_key(|d| Reverse(criterion.evaluate(&arena, d))),
            }

            for group in group.linear_group_by_key_mut(|d| criterion.evaluate(&arena, d)) {
                groups.push(group);
            }
        }
    }

    for raw_document in raw_documents {
        println!("{:?}", raw_document.id);
    }
}

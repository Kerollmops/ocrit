use std::cmp::Reverse;
use compact_arena::{mk_arena, SmallArena, Idx32};
use rand::{Rng, SeedableRng, rngs::StdRng};
use sdset::{Set, SetBuf};
use slice_group_by::GroupBy;

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

// ---------------------

struct Proximity;

impl Criterion for Proximity {
    fn name(&self) -> &str { "proximity" }
    fn order(&self) -> Order { Order::Asc }

    fn evaluate<'a, 'tag>(
        &self,
        postings_lists: &SmallArena<'tag, SetBuf<DocIndex>>,
        document: &RawDocument<'a, 'tag>,
    ) -> usize
    {
        0
    }
}

// ---------------------

fn main() {
    // let mut rng = StdRng::seed_from_u64(102);
    // let rng = &mut rng;

    // new
    let doc_indexes_new = SetBuf::from_dirty(vec![
        DocIndex { document_id: 0, attribute: 0, word_index: 0  },
        DocIndex { document_id: 0, attribute: 0, word_index: 5  },
        DocIndex { document_id: 0, attribute: 1, word_index: 9  },
        DocIndex { document_id: 0, attribute: 1, word_index: 10 },

        DocIndex { document_id: 1, attribute: 0, word_index: 0 },
        DocIndex { document_id: 1, attribute: 0, word_index: 4 },

        DocIndex { document_id: 2, attribute: 1, word_index: 6 },
        DocIndex { document_id: 2, attribute: 2, word_index: 4 },
    ]);

    // york
    let doc_indexes_york = SetBuf::from_dirty(vec![
        DocIndex { document_id: 0, attribute: 0, word_index: 6  },
        DocIndex { document_id: 0, attribute: 2, word_index: 0  },

        DocIndex { document_id: 1, attribute: 0, word_index: 1 },
        DocIndex { document_id: 1, attribute: 0, word_index: 5 },
    ]);

    mk_arena!(arena);
    let idx_new = arena.add(doc_indexes_new);
    let idx_york = arena.add(doc_indexes_york);

    // new york
    let mut bare_matches = vec![
        BareMatch { document_id: 0, query_index: 0, distance: 0, is_exact: true, postings_list: idx_new },
        BareMatch { document_id: 0, query_index: 1, distance: 0, is_exact: true, postings_list: idx_york },

        BareMatch { document_id: 1, query_index: 0, distance: 0, is_exact: true, postings_list: idx_new },
        BareMatch { document_id: 1, query_index: 1, distance: 0, is_exact: true, postings_list: idx_york },

        BareMatch { document_id: 2, query_index: 0, distance: 0, is_exact: true, postings_list: idx_new },
    ];

    bare_matches.sort_unstable_by_key(|bm| bm.document_id);

    let mut raw_documents = Vec::new();
    for bare_matches in bare_matches.linear_group_by_key(|bm| bm.document_id) {
        let id = bare_matches[0].document_id;
        raw_documents.push(RawDocument { id, bare_matches });
    }

    let criterion = (&Proximity) as &dyn Criterion;

    match criterion.order() {
        Order::Asc => raw_documents.sort_unstable_by_key(|d| criterion.evaluate(&arena, d)),
        Order::Dsc => raw_documents.sort_unstable_by_key(|d| Reverse(criterion.evaluate(&arena, d))),
    }

    for raw_document in raw_documents {
        println!("{:?}", raw_document.id);
    }
}

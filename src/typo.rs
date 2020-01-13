use compact_arena::SmallArena;
use sdset::SetBuf;
use crate::{DocIndex, Criterion, Order, RawDocument};

pub struct Typo;

impl Criterion for Typo {
    fn name(&self) -> &str { "typo" }
    fn order(&self) -> Order { Order::Asc }

    fn evaluate<'a, 'tag>(
        &self,
        postings_lists: &SmallArena<'tag, SetBuf<DocIndex>>,
        document: &RawDocument<'a, 'tag>,
    ) -> usize
    {
        document.bare_matches.iter().map(|bm| bm.distance as usize).sum()
    }
}

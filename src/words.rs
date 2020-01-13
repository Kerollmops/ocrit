use compact_arena::SmallArena;
use sdset::SetBuf;
use crate::{DocIndex, Criterion, Order, RawDocument};

pub struct Words;

impl Criterion for Words {
    fn name(&self) -> &str { "words" }
    fn order(&self) -> Order { Order::Dsc }

    fn evaluate<'a, 'tag>(
        &self,
        postings_lists: &SmallArena<'tag, SetBuf<DocIndex>>,
        document: &RawDocument<'a, 'tag>,
    ) -> usize
    {
        document.bare_matches.len()
    }
}

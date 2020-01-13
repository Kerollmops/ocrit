use std::cmp;
use compact_arena::SmallArena;
use sdset::SetBuf;
use crate::{DocIndex, Criterion, Order, RawDocument};

pub struct Attribute;

impl Criterion for Attribute {
    fn name(&self) -> &str { "attribute" }
    fn order(&self) -> Order { Order::Asc }

    fn evaluate<'a, 'tag>(
        &self,
        postings_lists: &SmallArena<'tag, SetBuf<DocIndex>>,
        document: &RawDocument<'a, 'tag>,
    ) -> usize
    {
        document.bare_matches.len()
    }
}

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
        let mut attr = u16::max_value();

        // we must take the attribute of the best matching words proximity
        for bm in document.bare_matches {
            for di in postings_lists[bm.postings_list].as_slice() {
                attr = cmp::min(attr, di.attribute);
                if attr == 0 { return 0; }
            }
        }

        attr as usize
    }
}

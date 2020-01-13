use std::cmp;
use compact_arena::SmallArena;
use sdset::SetBuf;
use crate::{DocIndex, Criterion, Order, RawDocument};

const MAX_DISTANCE: u16 = 8;

#[inline]
fn index_proximity(lhs: u16, rhs: u16) -> u16 {
    if lhs < rhs {
        cmp::min(rhs - lhs, MAX_DISTANCE)
    } else {
        cmp::min(lhs - rhs, MAX_DISTANCE) + 1
    }
}

#[inline]
fn attribute_proximity(lhs: DocIndex, rhs: DocIndex) -> u16 {
    if lhs.attribute != rhs.attribute { MAX_DISTANCE }
    else { index_proximity(lhs.word_index, rhs.word_index) }
}

pub struct Proximity;

impl Criterion for Proximity {
    fn name(&self) -> &str { "proximity" }
    fn order(&self) -> Order { Order::Asc }

    fn evaluate<'a, 'tag>(
        &self,
        postings_lists: &SmallArena<'tag, SetBuf<DocIndex>>,
        document: &RawDocument<'a, 'tag>,
    ) -> usize
    {
        let mut proximity = 0;

        println!("docid: {}", document.id);
        for bare_matches in document.bare_matches.windows(2) {
            let (bma, bmb) = match bare_matches { [a, b] => (a, b), _ => unreachable!() };

            let pla = &postings_lists[bma.postings_list];
            let plb = &postings_lists[bmb.postings_list];

            let mut min = usize::max_value();
            for dia in pla.as_slice() {
                let seek = (dia.attribute, dia.word_index);
                let result = plb.as_slice().binary_search_by(|p| (p.attribute, p.word_index).cmp(&seek));
                let idx = result.unwrap_or_else(|x| x.saturating_sub(1));
                min = cmp::min(min, attribute_proximity(*dia, plb[idx]) as usize);
            }

            proximity += min;
        }

        println!("  proximity: {:?}", proximity);

        proximity
    }
}

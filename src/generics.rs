use std::collections::{btree_map, BTreeMap};
use std::slice::Iter;

use crate::generics::IterMap::{IterEmpty, IterMapBTree, IterMapVec};

#[derive(Debug)]
pub(crate) enum Container<K, V> {
    Array(Vec<V>),
    MapVec(Vec<(K, V)>),
    MapBTree(BTreeMap<K, V>),
}

pub(crate) enum IterMap<'a, K, V> {
    IterEmpty(),
    IterMapVec(Iter<'a, (K, V)>),
    IterMapBTree(btree_map::Iter<'a, K, V>),
}

pub struct MapIterator<'a, K, V> {
    pub(crate) iter: IterMap<'a, K, V>,
}

impl<'a, K, V> Iterator for MapIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter {
            IterMapVec(iter_vec) => {
                match iter_vec.next() {
                    None => { None }
                    Some((k, v)) => { Some((k, v)) }
                }
            }
            IterMapBTree(iter_map) => {
                match iter_map.next() {
                    None => { None }
                    Some((k, v)) => { Some((k, v)) }
                }
            }
            IterEmpty() => {
                None
            }
        }
    }
}
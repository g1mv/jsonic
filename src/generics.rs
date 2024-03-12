use std::collections::{btree_map, BTreeMap};
use std::slice::Iter;
use crate::generics::IterMap::{IterMapBTree, IterMapVec};

#[derive(Debug)]
pub(crate) enum Container<K, V> {
    Array(Vec<V>),
    MapVec(Vec<(K, V)>),
    MapBTree(BTreeMap<K, V>),
}

pub(crate) enum IterMap<'a, K, V> {
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
            IterMapVec(a) => {
                match a.next() {
                    None => { None }
                    Some((k, v)) => { Some((k, v)) }
                }
            }
            IterMapBTree(b) => {
                match b.next() {
                    None => { None }
                    Some((k, v)) => { Some((k, v)) }
                }
            }
        }
    }
}
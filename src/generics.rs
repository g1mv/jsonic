use std::collections::{btree_map, BTreeMap};
use std::slice::Iter;

use crate::generics::IterArray::{IterArrayEmpty, IterArrayVec};
use crate::generics::IterMap::{IterMapBTree, IterMapEmpty, IterMapVec};

#[derive(Debug)]
pub(crate) enum Container<K, V> {
    Array(Vec<V>),
    MapVec(Vec<(K, V)>),
    MapBTree(BTreeMap<K, V>),
}

pub(crate) enum IterArray<'a, V> {
    IterArrayEmpty(),
    IterArrayVec(Iter<'a, V>),
}

pub struct ArrayIterator<'a, V> {
    pub(crate) iter: IterArray<'a, V>,
}

impl<'a, V> Iterator for ArrayIterator<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter {
            IterArrayVec(iter_vec) => { iter_vec.next() }
            IterArrayEmpty() => { None }
        }
    }
}

pub(crate) enum IterMap<'a, K, V> {
    IterMapEmpty(),
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
            IterMapBTree(iter_map) => { iter_map.next() }
            IterMapEmpty() => { None }
        }
    }
}
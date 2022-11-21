use crate::libspecr::*;

use std::fmt::{Formatter, Debug, Error};

impl<K, V> Default for Map<K, V> where K: GcCompat, V: GcCompat {
    fn default() -> Self {
        Self(GcCow::new(IMHashMap::new()))
    }
}

impl<K, V> Clone for Map<K, V> {
    fn clone(&self) -> Self { Self(self.0) }
}
impl<K, V> Copy for Map<K, V> {}

impl<K, V> Debug for Map<K, V> where K: Hash + Eq + Debug, V: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl<K, V> GcCompat for Map<K, V> where K: GcCompat, V: GcCompat {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl<K, V> GcCompat for IMHashMap<K, V> where K: GcCompat, V: GcCompat {
    fn points_to(&self, m: &mut HashSet<usize>) {
        for (k, v) in self.iter() {
            k.points_to(m);
            v.points_to(m);
        }
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl<K, V> PartialEq for Map<K, V> where K: Eq + GcCompat + Clone + Hash, V: PartialEq + GcCompat + Clone {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl<K, V> Eq for Map<K, V> where K: Eq + GcCompat + Clone + Hash, V: Eq + GcCompat + Clone {}
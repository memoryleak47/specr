use crate::{*, gccow::*};

use std::marker::PhantomData;
use std::sync::RwLock;

impl<T: GcCompat> GcCompat for GcCow<T> {
    fn points_to(&self, buffer: &mut HashSet<usize>) {
        buffer.insert(self.idx);
    }
    fn as_any(&self) -> &dyn Any { self }
}

pub struct GcState {
    pub objs: SparseVec<Box<dyn GcCompat>>,
}

// Note that there exists only one instance of `GC_STATE`.
// Hence tests using `mark_and_sweep` should not be run in parallel!
pub static GC_STATE: RwLock<GcState> = RwLock::new(GcState::new());

impl GcState {
    pub const fn new() -> GcState {
        Self { objs: SparseVec::new() }
    }

    pub fn alloc<T: GcCompat>(&mut self, t: T) -> GcCow<T> {
        let obj: Box<dyn GcCompat> = Box::new(t);
        let idx = self.objs.insert(obj);
        GcCow { idx, phantom: PhantomData }
    }

    pub fn mark_and_sweep(&mut self, roots: HashSet<usize>) {
        // `objs` which are found to be reachable from `roots`, but they're children were not yet added.
        let mut open = roots;

        // `objs` which are found to be reachable from `roots`, whose children have already been added.
        let mut done = HashSet::new();

        while let Some(i) = open.iter().next().cloned() {
            open.remove(&i);
            done.insert(i);

            let mut current = HashSet::new();
            self.objs.get(i).points_to(&mut current);

            for new in current {
                if !done.contains(&new) && !open.contains(&new) {
                    open.insert(new);
                }
            }
        }
        // seen now contains the `usize` which are reachable from roots.
        let seen = done;

        self.objs.retain(seen);
    }
}

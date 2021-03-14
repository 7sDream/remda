use {
    rayon::iter::{IndexedParallelIterator, ParallelIterator},
    std::{cmp::Ordering, collections::BinaryHeap, sync::mpsc::channel},
};

#[derive(Debug)]
struct ReverseFirst<T>(usize, T);

impl<T> PartialEq for ReverseFirst<T> {
    fn eq(&self, rhs: &Self) -> bool {
        rhs.0.eq(&self.0)
    }
}

impl<T> Eq for ReverseFirst<T> {}

impl<T> PartialOrd for ReverseFirst<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        rhs.0.partial_cmp(&self.0)
    }
}

impl<T> Ord for ReverseFirst<T> {
    fn cmp(&self, o: &Self) -> Ordering {
        o.0.cmp(&self.0)
    }
}

// TODO: Write document for this complex but useful function
pub trait SeqForEach: IndexedParallelIterator {
    fn seq_for_each_with<SetUpRet, E: Send, SetUp, F>(self, setup: SetUp, mut f: F) -> Result<(), E>
    where
        SetUp: FnOnce() -> Result<SetUpRet, E> + Send,
        F: FnMut(&mut SetUpRet, Self::Item) -> Result<(), E> + Send,
    {
        let (tx, rx) = channel();
        let mut ret = Ok(());

        rayon::scope(|s| {
            s.spawn(|_| {
                self.enumerate().for_each_with(tx, |sender, idx_and_item| {
                    let _ = sender.send(idx_and_item);
                })
            });
            s.spawn(|_| {
                let mut context = match setup() {
                    Ok(context) => context,
                    Err(e) => {
                        ret = Err(e);
                        return;
                    }
                };
                let mut expected = 0;
                let mut heap = BinaryHeap::new();
                for (idx, item) in rx {
                    heap.push(ReverseFirst(idx, item));
                    while let Some(&ReverseFirst(idx, _)) = heap.peek() {
                        if idx != expected {
                            break;
                        }
                        let ReverseFirst(_idx, item) = heap.pop().unwrap();
                        if let e @ Err(_) = f(&mut context, item) {
                            ret = e;
                            return;
                        }
                        expected += 1;
                    }
                }
            });
        });

        ret
    }
}

impl<P: IndexedParallelIterator> SeqForEach for P {}

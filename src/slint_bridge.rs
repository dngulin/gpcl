use slint::{Model, ModelNotify, ModelTracker};
use std::any::Any;
use std::cell::RefCell;

pub struct Bridge<TSrc, TView> {
    items: RefCell<Vec<(TSrc, TView)>>,
    notify: ModelNotify,
}

impl<TSrc: 'static, TView: Clone + 'static> Model for Bridge<TSrc, TView> {
    type Data = TView;

    fn row_count(&self) -> usize {
        self.items.borrow().len()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.items.borrow().get(row).map(|(_, x)| x).cloned()
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        &self.notify
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<TSrc, TView> Bridge<TSrc, TView> {
    pub fn new(items: Vec<(TSrc, TView)>) -> Self {
        Self {
            items: RefCell::new(items),
            notify: ModelNotify::default(),
        }
    }

    pub fn add(&self, item: (TSrc, TView)) {
        let idx = self.items.borrow().len();

        self.items.borrow_mut().push(item);
        self.notify.row_added(idx, 1);
    }

    pub fn remove(&self, predicate: impl Fn(&TSrc) -> bool) {
        if let Some(idx) = self.find_index(predicate) {
            self.items.borrow_mut().remove(idx);
            self.notify.row_removed(idx, 1);
        }
    }

    fn find_index(&self, predicate: impl Fn(&TSrc) -> bool) -> Option<usize> {
        for (idx, (src, _)) in self.items.borrow().iter().enumerate() {
            if predicate(src) {
                return Some(idx);
            }
        }

        None
    }

    pub fn update_items<F>(&self, mut f: F)
    where
        F: FnMut(&mut TSrc, &mut TView) -> bool,
    {
        self.items
            .borrow_mut()
            .iter_mut()
            .enumerate()
            .for_each(move |(idx, (src, view))| {
                let view_updated = f(src, view);
                if view_updated {
                    self.notify.row_changed(idx);
                }
            });
    }
}

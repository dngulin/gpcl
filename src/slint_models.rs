use slint::{Model, ModelNotify, ModelTracker};
use std::any::Any;
use std::cell::{Ref, RefCell};

pub struct ExtVecModel<TModel, TExt> {
    items: RefCell<Vec<(TModel, TExt)>>,
    notify: ModelNotify,
}

impl<TModel: Clone + 'static, TExt: 'static> Model for ExtVecModel<TModel, TExt> {
    type Data = TModel;

    fn row_count(&self) -> usize {
        self.items.borrow().len()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.items.borrow().get(row).map(|(m, _)| m).cloned()
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        &self.notify
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<TModel, TExt> ExtVecModel<TModel, TExt> {
    pub fn new() -> Self {
        Self {
            items: RefCell::new(Vec::new()),
            notify: ModelNotify::default(),
        }
    }

    pub fn with_items(items: Vec<(TModel, TExt)>) -> Self {
        Self {
            items: RefCell::new(items),
            notify: ModelNotify::default(),
        }
    }

    pub fn clear(&self) {
        let len = self.items.borrow().len();
        self.items.borrow_mut().clear();

        if len > 0 {
            self.notify.row_removed(0, len);
        }
    }

    pub fn add(&self, item: (TModel, TExt)) {
        let idx = self.items.borrow().len();

        self.items.borrow_mut().push(item);
        self.notify.row_added(idx, 1);
    }

    pub fn remove(&self, predicate: impl Fn(&(TModel, TExt)) -> bool) {
        if let Some(idx) = self.find_index(predicate) {
            self.items.borrow_mut().remove(idx);
            self.notify.row_removed(idx, 1);
        }
    }

    fn find_index(&self, predicate: impl Fn(&(TModel, TExt)) -> bool) -> Option<usize> {
        for (idx, pair) in self.items.borrow().iter().enumerate() {
            if predicate(pair) {
                return Some(idx);
            }
        }

        None
    }

    pub fn update_items<F>(&self, mut update_item: F)
    where
        F: FnMut(&mut TModel, &mut TExt) -> bool,
    {
        self.items
            .borrow_mut()
            .iter_mut()
            .enumerate()
            .for_each(move |(idx, (model, ext))| {
                if update_item(model, ext) {
                    self.notify.row_changed(idx);
                }
            });
    }

    pub fn get_ref(&self, idx: usize) -> Option<Ref<'_, (TModel, TExt)>> {
        Ref::filter_map(self.items.borrow(), |items| items.get(idx)).ok()
    }
}

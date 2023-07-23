use std::marker::PhantomData;
use std::collections::HashMap;
use std::cell::{ RefCell, Cell };

/// References a resource by a compile-time known identifier,
/// then once it find a resource with the same identifier
/// caches the resources index.
pub struct ResourceId<T> {
    _phantom: PhantomData<T>,
    state: Cell<ResourceIdCacheState>,
}

#[derive(Clone, Copy)]
enum ResourceIdCacheState {
    Uncached(&'static str),
    Cached(usize),
}

/// Manages storring and fetching specific types of resources.
pub struct Repository<T> {
    identifiers: RefCell<HashMap<String, usize>>,
    resources: Vec<T>,
}

impl<T> Clone for ResourceId<T> {
    fn clone(&self) -> Self {
        Self { _phantom: PhantomData, state: self.state.clone() }
    }
}

impl<T> ResourceId<T> {
    /// Constructs a new [ResourceId] lazily, as it waits to cache
    /// the index of the resource once it finds it.
    pub fn new(identifier: &'static str) -> Self {
        Self { _phantom: PhantomData, state: Cell::new(ResourceIdCacheState::Uncached(identifier)) }
    }
}

impl<T> Default for Repository<T> {
    fn default() -> Self {
        Self {
            identifiers: RefCell::new(HashMap::new()),
            resources: Vec::new(),
        }
    }
}

impl<T> Repository<T> {
    /// Constructs a new [Repository].
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a resource to this [Repository] and returns its Id.
    pub fn add(&mut self, resource: T, identifier: Option<&'static str>) -> ResourceId<T> {
        let index = self.resources.len();
        self.resources.push(resource);
        if let Some(identifier) = identifier {
            self.identifiers.borrow_mut().insert(identifier.to_string(), index);
        }
        ResourceId { _phantom: PhantomData, state: Cell::new(ResourceIdCacheState::Cached(index)) }
    }

    /// Gets a resource using a [ResourceId]. Doesn't check if the Id is valid.
    pub fn get_unchecked(&self, id: &ResourceId<T>) -> &T {
        let index = match id.state.get() {
            ResourceIdCacheState::Cached(index) => index,
            ResourceIdCacheState::Uncached(identifier) => {
                let index = *self.identifiers.borrow().get(identifier).unwrap();
                id.state.set(ResourceIdCacheState::Cached(index));
                index
            }
        };

        &self.resources[index]
    }
}

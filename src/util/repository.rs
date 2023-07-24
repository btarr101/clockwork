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

    /// Gets a resource using a [ResourceId].
    pub fn get(&self, id: &ResourceId<T>) -> Option<&T> {
        let index = match id.state.get() {
            ResourceIdCacheState::Cached(index) => index,
            ResourceIdCacheState::Uncached(identifier) => {
                let index = *self.identifiers.borrow().get(identifier)?;
                id.state.set(ResourceIdCacheState::Cached(index));
                index
            }
        };

        self.resources.get(index)
    }

    /// Gets a resource mutably using a [ResourceId].
    pub fn get_mut(&mut self, id: &ResourceId<T>) -> Option<&mut T> {
        let index = match id.state.get() {
            ResourceIdCacheState::Cached(index) => index,
            ResourceIdCacheState::Uncached(identifier) => {
                let index = *self.identifiers.borrow().get(identifier)?;
                id.state.set(ResourceIdCacheState::Cached(index));
                index
            }
        };

        self.resources.get_mut(index)
    }
}

impl<T> std::ops::Index<&ResourceId<T>> for Repository<T> {
    type Output = T;

    fn index(&self, id: &ResourceId<T>) -> &Self::Output {
        self.get(id).unwrap()
    }
}

impl<T> std::ops::IndexMut<&ResourceId<T>> for Repository<T> {
    fn index_mut(&mut self, id: &ResourceId<T>) -> &mut Self::Output {
        self.get_mut(id).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// I really have been playing too much minecraft...
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct Tool {
        /// The type of tool this is.
        tool_type: ToolType,

        /// How valuable this tool is (from 0 - 100).
        value: usize,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum ToolType {
        Axe,
        Pickaxe,
        Hoe,
    }

    #[test]
    fn test_add() {
        let mut repository = Repository::<Tool>::new();
        let tool = Tool {
            tool_type: ToolType::Axe,
            value: 10,
        };
        let tool_id = repository.add(tool, None);

        let tool_ref = repository[&tool_id];

        assert!(tool_ref == tool)
    }

    #[test]
    fn test_lazy() {
        let mut repository = Repository::<Tool>::new();
        let tool = Tool {
            tool_type: ToolType::Pickaxe,
            value: 100,
        };

        repository.add(tool, Some("Diamond Pick"));
        let tool_id = ResourceId::<Tool>::new("Diamond Pick");

        assert!(repository[&tool_id] == tool);

        // get it again
        assert!(repository[&tool_id] == tool);
    }

    #[test]
    fn test_lazy_noexist() {
        let repository = Repository::<Tool>::new();

        let tool_id = ResourceId::<Tool>::new("Netherite Pick");

        assert!(repository.get(&tool_id).is_none())
    }

    #[test]
    #[should_panic]
    fn test_lazy_noexist_unchecked() {
        let repository = Repository::<Tool>::new();

        let tool_id = ResourceId::<Tool>::new("Wooden Hoe");

        let _ = repository[&tool_id];
    }

    #[test]
    fn test_mut() {
        let mut repository = Repository::<Tool>::new();
        let tool = Tool {
            tool_type: ToolType::Hoe,
            value: 60,
        };

        let tool_id = repository.add(tool, Some("Golden Hoe"));
        let lazy_tool_id = ResourceId::<Tool>::new("Golden Hoe");

        // Change the value property
        let tool_ref = &mut repository[&tool_id];
        assert!(*tool_ref == tool);
        tool_ref.value = 100;

        // Access again.
        let tool_ref = repository[&lazy_tool_id];
        assert!(tool_ref.value == 100);
    }
}

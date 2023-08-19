use std::hash::Hash;
use std::{fmt::Debug, marker::PhantomData};

/// References a resource within a [Repository].
pub struct ResourceId<T> {
    _phantom: PhantomData<T>,
    pub index: usize,
}

impl<T> Clone for ResourceId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ResourceId<T> {}

impl<T> Debug for ResourceId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourceId")
            .field("_phantom", &self._phantom)
            .field("index", &self.index)
            .finish()
    }
}

impl<T> PartialEq for ResourceId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<T> Eq for ResourceId<T> {}

impl<T> Hash for ResourceId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

/// Manages storing and fetching specific types of resources.
///
/// To be completely fair, at the moment it's just a glorified vector,
/// however the extra typing rules and abstraction potential is something
/// that will be used eventually.
pub struct Repository<T> {
    resources: Vec<(Option<T>, usize)>,
}

impl<T> ResourceId<T> {
    /// Constructs a new [ResourceId].
    pub const fn new(index: usize) -> Self {
        Self {
            _phantom: PhantomData,
            index,
        }
    }
}

impl<T> Default for Repository<T> {
    fn default() -> Self {
        Self {
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
    pub fn add(&mut self, resource: T, with_id: Option<ResourceId<T>>) -> ResourceId<T> {
        let index = match with_id {
            Some(resource_id) => resource_id.index,
            None => self.resources.len(),
        };

        if index >= self.resources.len() {
            self.resources.resize_with(index + 1, || (None, 0));
        }

        let generation = self.resources[index].1;
        self.resources[index] = (Some(resource), generation + 1);

        ResourceId::new(index)
    }

    /// Gets a resource using a [ResourceId].
    pub fn get(&self, id: ResourceId<T>) -> Option<&T> {
        self.resources.get(id.index)?.0.as_ref()
    }

    /// Gets a resource mutably using a [ResourceId].
    pub fn get_mut(&mut self, id: ResourceId<T>) -> Option<&mut T> {
        self.resources.get_mut(id.index)?.0.as_mut()
    }

    /// Gets the generation of a resource given a [ResourceId].
    ///
    /// This can be useful to implement caching mechanisms.
    pub fn get_generation(&self, id: ResourceId<T>) -> usize {
        self.resources
            .get(id.index)
            .map(|entry| entry.1)
            .unwrap_or(0)
    }
}

impl<T> std::ops::Index<ResourceId<T>> for Repository<T> {
    type Output = T;

    fn index(&self, id: ResourceId<T>) -> &Self::Output {
        self.get(id).unwrap()
    }
}

impl<T> std::ops::IndexMut<ResourceId<T>> for Repository<T> {
    fn index_mut(&mut self, id: ResourceId<T>) -> &mut Self::Output {
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
        let tool_ref = repository[tool_id];

        assert!(tool_ref == tool)
    }

    #[test]
    fn test_add_with() {
        let mut repository = Repository::<Tool>::new();
        let tool = Tool {
            tool_type: ToolType::Pickaxe,
            value: 100,
        };
        let tool_id = ResourceId::new(10);
        repository.add(tool, Some(tool_id));
        let tool_ref = repository[tool_id];

        assert!(tool_ref == tool)
    }

    #[test]
    fn test_noexist() {
        let repository = Repository::<Tool>::new();

        let tool_id = ResourceId::<Tool>::new(0);

        assert!(repository.get(tool_id).is_none())
    }

    #[test]
    #[should_panic]
    fn test_noexist_unchecked() {
        let repository = Repository::<Tool>::new();

        let tool_id = ResourceId::<Tool>::new(30);

        let _ = repository[tool_id];
    }

    #[test]
    fn test_mut() {
        let mut repository = Repository::<Tool>::new();
        let tool = Tool {
            tool_type: ToolType::Hoe,
            value: 60,
        };

        let tool_id = repository.add(tool, Some(ResourceId::new(10)));

        // Change the value property
        let tool_ref = &mut repository[tool_id];
        assert!(*tool_ref == tool);
        tool_ref.value = 100;

        // Access again.
        let tool_ref = repository[tool_id];
        assert!(tool_ref.value == 100);
    }

    #[test]
    fn test_get_generation() {
        let mut repository = Repository::<Tool>::new();
        let hoe = Tool {
            tool_type: ToolType::Hoe,
            value: 60,
        };

        let pickaxe = Tool {
            tool_type: ToolType::Pickaxe,
            value: 100,
        };

        let tool_id = ResourceId::new(10);

        assert!(repository.get_generation(tool_id) == 0);

        repository.add(hoe, Some(tool_id));

        dbg!(repository.get_generation(tool_id));
        assert!(repository.get_generation(tool_id) == 1);

        repository.add(pickaxe, Some(tool_id));

        assert!(repository.get_generation(tool_id) == 2);
    }
}

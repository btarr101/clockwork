use std::marker::PhantomData;

/// References a resource within a [Repository].
#[derive(Clone, Copy)]
pub struct ResourceId<T> {
    _phantom: PhantomData<T>,
    pub index: usize,
}

/// Manages storing and fetching specific types of resources.
///
/// To be completely fair, at the moment it's just a glorified vector,
/// however the extra typing rules and abstraction potential is something
/// that will be used eventually.
pub struct Repository<T> {
    resources: Vec<Option<T>>,
}

impl<T> ResourceId<T> {
    /// Constructs a new [ResourceId].
    pub fn new(index: usize) -> Self {
        Self { _phantom: PhantomData, index }
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

        self.resources.resize_with(index + 1, || None);
        self.resources[index] = Some(resource);

        ResourceId::new(index)
    }

    /// Gets a resource using a [ResourceId].
    pub fn get(&self, id: ResourceId<T>) -> Option<&T> {
        self.resources.get(id.index)?.as_ref()
    }

    /// Gets a resource mutably using a [ResourceId].
    pub fn get_mut(&mut self, id: ResourceId<T>) -> Option<&mut T> {
        self.resources.get_mut(id.index)?.as_mut()
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
}

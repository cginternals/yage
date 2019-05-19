///
/// Container that holds a specific type of resource.
///
pub struct ResourceManager<T> {
    objects: Vec<T>,
}

impl<T> ResourceManager<T> {
    ///
    /// Create resource manager.
    ///
    /// # Returns
    /// A new instance of ResourceManager.
    ///
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }

    ///
    /// Get number of objects
    ///
    /// # Returns
    /// Number of objects
    ///
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    ///
    /// Get object at index
    ///
    /// # Parameters
    /// - `index`: Index of the resource object
    ///
    /// # Returns
    /// Reference to the object, or None if index is invalid
    ///
    pub fn get(&self, index: usize) -> Option<&T> {
        return self.objects.get(index);
    }

    ///
    /// Get object at index
    ///
    /// # Parameters
    /// - `index`: Index of the resource object
    ///
    /// # Returns
    /// Mutable reference to the object, or None if index is invalid
    ///
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        return self.objects.get_mut(index);
    }

    ///
    /// Add object to manager
    ///
    /// # Parameters
    /// - `obj`: Resource object
    ///
    /// # Returns
    /// Index of the resource object
    ///
    pub fn add(&mut self, obj: T) -> usize {
        // Add object to manager
        self.objects.push(obj);

        // Return index of object
        self.objects.len() - 1
    }
}

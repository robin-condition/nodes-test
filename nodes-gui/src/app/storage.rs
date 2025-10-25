use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ID(usize);

#[derive(Clone)]
pub struct Storage<T> {
    entries: Vec<T>,
    ids: Vec<ID>,
    ids_to_inds: HashMap<ID, usize>,
    next_unallocated_id: ID,
    unused_ids: Vec<ID>,
}

impl<T> Default for Storage<T> {
    fn default() -> Self {
        Self {
            entries: Default::default(),
            ids: Default::default(),
            ids_to_inds: Default::default(),
            next_unallocated_id: ID(0),
            unused_ids: Default::default(),
        }
    }
}

impl<T> Storage<T> {
    fn get_next_id(&mut self) -> ID {
        if let Some(id) = self.unused_ids.pop() {
            return id;
        }

        let new_id = self.next_unallocated_id;
        self.next_unallocated_id = ID(self.next_unallocated_id.0 + 1);
        return new_id;
    }

    pub fn create(&mut self, obj: T) -> (&mut T, ID) {
        let id = self.get_next_id();
        self.entries.push(obj);
        self.ids.push(id);
        let ind = self.entries.len() - 1;
        self.ids_to_inds.insert(id, ind);
        (&mut self.entries[ind], id)
    }

    pub fn remove(&mut self, id: ID) {
        let ind = self.ids_to_inds.remove(&id).unwrap();
        self.unused_ids.push(id);

        if ind == self.entries.len() - 1 {
            self.entries.pop();
            self.ids.pop();
            return;
        }

        let node_to_move = self.entries.pop().unwrap();
        let id_to_update = self.ids.pop().unwrap();
        self.ids_to_inds.insert(id_to_update, ind);
        self.entries[ind] = node_to_move;
        self.ids[ind] = id_to_update;
    }

    pub fn get(&self, id: ID) -> &T {
        &self.entries[*self.ids_to_inds.get(&id).unwrap()]
    }

    pub fn get_mut(&mut self, id: ID) -> &mut T {
        &mut self.entries[*self.ids_to_inds.get(&id).unwrap()]
    }

    pub fn exists(&self, id: ID) -> bool {
        self.ids_to_inds.contains_key(&id)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.entries.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.entries.iter_mut()
    }

    pub fn ids(&self) -> &Vec<ID> {
        &self.ids
    }

    pub fn with_ids(&self) -> impl IntoIterator<Item = (&ID, &T)> {
        self.ids.iter().zip(&self.entries)
    }

    pub fn with_ids_mut(&mut self) -> impl IntoIterator<Item = (&ID, &mut T)> {
        self.ids.iter().zip(&mut self.entries)
    }
}

impl<T> IntoIterator for Storage<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Storage<T> {
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Storage<T> {
    type Item = &'a mut T;

    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

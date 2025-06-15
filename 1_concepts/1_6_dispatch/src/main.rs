use std::borrow::Cow;
use std::collections::HashMap;

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Debug, Clone, PartialEq, Default)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

impl User {
    fn new(id: u64, email: impl Into<Cow<'static, str>>, activated: bool) -> Self {
        Self {
            id,
            email: email.into(),
            activated,
        }
    }
}

#[derive(Debug, Default)]
struct HashMapStorage<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> Storage<K, V> for HashMapStorage<K, V>
where 
    K: std::hash::Hash + Eq + Clone,
{
    fn set(&mut self, key: K, val: V) {
        self.data.insert(key, val);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }
}

// Dynamic dispatch
struct DynamicUserRepository {
    storage: Box<dyn Storage<u64, User>>,
}

impl DynamicUserRepository {
    fn new(storage: Box<dyn Storage<u64, User>>) -> Self {
        Self { storage }
    }

    fn add(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn get(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    fn update(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn remove(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}

// Static dispatch
struct StaticUserRepository<S> {
    storage: S,
}

impl<S> StaticUserRepository<S>
where
    S: Storage<u64, User>,
{
    fn new(storage: S) -> Self {
        Self { storage }
    }

    fn add(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn get(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    fn update(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn remove(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_user_repository() {
        let storage = Box::new(HashMapStorage::<u64, User>::default());
        let mut user_repo = DynamicUserRepository::new(storage);

        let user1 = User::new(1, "user1@test.com", true);
        user_repo.add(user1.clone());

        assert_eq!(user_repo.get(1), Some(&user1));
        assert_eq!(user_repo.remove(1), Some(user1));
        assert_eq!(user_repo.get(1), None);

        assert_eq!(user_repo.get(999), None);
    }

    #[test]
    fn test_static_user_repository() {
        let storage = HashMapStorage::<u64, User>::default();
        let mut user_repo = StaticUserRepository::new(storage);

        let user1 = User::new(1, "user1@test.com", true);
        user_repo.add(user1.clone());

        assert_eq!(user_repo.get(1), Some(&user1));
        assert_eq!(user_repo.remove(1), Some(user1));
        assert_eq!(user_repo.get(1), None);

        assert_eq!(user_repo.get(999), None);
    }
}

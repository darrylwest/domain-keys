use crate::keys::RouteKey;
use anyhow::Result;
/// base data models
use chrono::naive::NaiveDateTime;
use chrono::Utc;
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;
use std::hash::{Hash, Hasher};

pub type TS = NaiveDateTime;

fn get_now() -> TS {
    Utc::now().naive_utc()
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Version {
    pub created_at: TS,
    pub updated_at: TS,
    pub update_count: u64,
    pub hash: u64,
}

impl Version {
    pub fn new(hash: u64) -> Version {
        let now = get_now();
        Version {
            created_at: now,
            updated_at: now,
            update_count: 0,
            hash,
        }
    }

    pub fn update(&self, new_hash: u64) -> Version {
        Version {
            created_at: self.created_at,
            updated_at: get_now(),
            update_count: self.update_count + 1,
            hash: new_hash,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "t", content = "c")]
pub enum Status {
    New(u8),
    Pending(u8),
    Active(u8),
    Inactive(u8),
    Processed(u8),
    Blocked(u8),
    Deleted(u8),
}

impl Default for Status {
    fn default() -> Self {
        Status::New(0)
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Model<T> {
    pub key: Cow<'static, str>,
    pub version: Version,
    pub status: Status,
    pub value: T,
}

impl<T: Default + Clone + PartialOrd + Ord + PartialEq + Eq + Hash> Model<T> {
    /// calculate the new hash value for this model
    pub fn calc_hash(value: &T) -> u64 {
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);

        hasher.finish()
    }

    /// create a new model from the value; calc the new hash
    pub fn new(value: T) -> Model<T> {
        let hash = Model::calc_hash(&value);
        let version = Version::new(hash);
        let key: Cow<'static, str> = Cow::Owned(RouteKey::create());

        Model {
            key,
            version,
            status: Status::New(0),
            value,
        }
    }

    /// create a copy from the model
    pub fn from_model(model: &Model<T>) -> Model<T> {
        Model {
            key: model.key.clone(),
            version: model.version.clone(),
            status: model.status.clone(),
            value: model.value.clone(),
        }
    }

    /// create a new model from existing values of key, version, status and value
    pub fn create_model(key: String, version: &Version, status: &Status, value: &T) -> Model<T> {
        Model {
            key: Cow::Owned(key),
            version: version.clone(),
            status: status.clone(),
            value: value.clone(),
        }
    }
}

pub enum ModelError {
    Email(Cow<'static, str>),
}

trait DomainModel<T> {
    fn validate(model: &Model<T>) -> Vec<ModelError>;
    fn insert(model: Model<T>) -> Result<Model<T>>;
    fn update(model: Model<T>) -> Result<Model<T>>;

    /// this can be used to mask or slim down attribute data
    fn list_view(model: &Model<T>) -> Model<T>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::thread;
    use std::time::Duration;
    // use anyhow::Result;

    #[test]
    fn new_version() {
        let hash = 5_000_000u64;
        let version = Version::new(hash);

        assert_eq!(version.created_at, version.updated_at);
        assert_eq!(version.update_count, 0);
        assert_eq!(version.hash, hash);
    }

    #[test]
    fn update_version() {
        let hash = 2_000_000u64;
        let v1 = Version::new(hash);

        assert_eq!(v1.created_at, v1.updated_at);
        assert_eq!(v1.update_count, 0);
        assert_eq!(v1.hash, hash);

        thread::sleep(Duration::from_micros(1));

        let new_hash = 3_000_000u64;
        let v2 = v1.update(new_hash);

        assert_eq!(
            v1.created_at.timestamp_micros(),
            v2.created_at.timestamp_micros()
        );

        assert_ne!(v2.created_at, v2.updated_at);
        assert!(v2.created_at < v2.updated_at);
        assert_eq!(v1.update_count, 0);
        assert_eq!(v2.update_count, 1);
        assert_eq!(v1.hash, hash);
        assert_eq!(v2.hash, new_hash);
    }

    #[test]
    fn new_model() {
        let value = String::from("my test");
        let model = Model::new(value.clone());

        assert_eq!(model.key.len(), 16);
        assert_eq!(model.version.update_count, 0);
        assert_eq!(model.value, value);
    }

    #[test]
    fn create_model() {
        let key = RouteKey::create();
        let kcopy = String::from(&key);
        let version = Version::new(10u64);
        let value = "me".to_string();
        let status = Status::Active(128);

        let model = Model::create_model(key, &version, &status, &value);

        assert_eq!(model.key, kcopy);
        assert_eq!(model.version, version);
        assert_eq!(model.status, status);
        assert_eq!(model.value, value);
    }

    #[test]
    fn create_person_model() {
        let email = Cow::from("dpw@rcs.com");
        let name = Cow::from("steve johnson");
        let phone = Cow::from("123-555-3333");
        let person = Peep::new(email, name, phone);

        let key = RouteKey::create();
        let kcopy = String::from(&key);
        let version = Version::new(10u64);
        let status = Status::Active(128);

        let model = Model::create_model(key, &version, &status, &person);

        assert_eq!(model.key, kcopy);
        assert_eq!(model.version, version);
        assert_eq!(model.status, status);
        assert_eq!(model.value, person);
    }

    #[test]
    fn calc_hash() {
        let email = Cow::from("dpw@rcs.com");
        let name = Cow::from("steve johnson");
        let phone = Cow::from("123-555-3333");
        let person = Peep::new(email, name, phone);

        let key = RouteKey::create();
        let kcopy = String::from(&key);
        let version = Version::new(10u64);
        assert_eq!(version.update_count, 0);
        let update_count = version.update_count;
        let status = Status::Active(128);

        let model = Model::create_model(key, &version, &status, &person);
        assert_eq!(model.version.update_count, 0);

        let hash = Model::calc_hash(&model.value);

        println!("new hash: {}", hash);
        assert_ne!(model.version.hash, hash);

        let version = model.version.update(hash);
        assert_eq!(version.update_count, 1);
        let mcopy = Model::create_model(kcopy, &version, &model.status, &model.value);

        assert_eq!(mcopy.version.update_count, update_count + 1);
    }

    #[derive(
        Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
    )]
    pub struct Peep {
        pub email: Cow<'static, str>,
        pub name: Cow<'static, str>,
        pub phone: Cow<'static, str>,
    }

    impl Peep {
        pub fn new(
            email: Cow<'static, str>,
            name: Cow<'static, str>,
            phone: Cow<'static, str>,
        ) -> Peep {
            Peep { email, name, phone }
        }
    }

    impl DomainModel<Peep> for Peep {
        fn validate(model: &Model<Peep>) -> Vec<ModelError> {
            let peep = model.value.clone();
            let mut errors: Vec<ModelError> = Vec::new();

            if peep.email.is_empty() {
                let e = ModelError::Email(peep.email);
                errors.push(e)
            }

            errors
        }

        fn insert(model: Model<Peep>) -> Result<Model<Peep>> {
            Ok(model)
        }

        fn update(model: Model<Peep>) -> Result<Model<Peep>> {
            Ok(model)
        }

        fn list_view(model: &Model<Peep>) -> Model<Peep> {
            let mut value = model.value.clone();
            value.phone = Cow::from("");

            Model::create_model(model.key.to_string(), &model.version, &model.status, &value)
        }
    }
}

use super::join_entity::JoinEntities;

pub trait QueryInterface<T, R> {
    fn get_many(&self, with: JoinEntities<R>) -> Result<Vec<T>, std::io::Error>;
    fn get_one_by_id(&self, id: &str, with: JoinEntities<R>) -> Result<T, std::io::Error>;
}

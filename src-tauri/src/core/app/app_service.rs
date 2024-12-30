use crate::core::common::interface::sql::SQLInterface;

pub struct AppService<S>
where
    S: SQLInterface,
{
    pub model: S,
}

impl<S> AppService<S>
where
    S: SQLInterface,
{
    pub fn new(sql: S) -> Self {
        Self { model: sql }
    }
}

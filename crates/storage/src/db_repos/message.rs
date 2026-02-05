use uuid::Uuid;

use crate::{db_repos::interface::DatabaseRepo, db_types::message::DbMessage, error::DbError};

pub struct MessageRepo {
    db: todo!(),
}

impl DatabaseRepo for MessageRepo {
    type DataElement = DbMessage;

    type Error = DbError;

    type Identifier = Uuid;

    fn create(value: Self::DataElement) -> Result<Self::DataElement, Self::Error> {
        todo!()
    }

    fn read() -> Result<Self::DataElement, Self::Error> {
        todo!()
    }

    fn update(
        id: Self::Identifier,
        update: Self::DataElement,
    ) -> Result<Self::DataElement, Self::Error> {
        todo!()
    }

    fn delete(id: Self::Identifier) -> Result<Self::DataElement, Self::Error> {
        todo!()
    }
}

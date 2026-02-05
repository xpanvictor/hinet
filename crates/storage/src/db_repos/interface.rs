pub trait DatabaseRepo {
    type DataElement;
    type Error;
    type Identifier;

    fn create(value: Self::DataElement) -> Result<Self::DataElement, Self::Error>;
    fn read() -> Result<Self::DataElement, Self::Error>;
    fn update(
        id: Self::Identifier,
        update: Self::DataElement,
    ) -> Result<Self::DataElement, Self::Error>;
    fn delete(id: Self::Identifier) -> Result<Self::DataElement, Self::Error>;
}

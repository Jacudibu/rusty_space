use crate::persistence::data::v1::SaveDataCollection;

impl<T, I> From<I> for SaveDataCollection<T>
where
    I: IntoIterator<Item = T>,
{
    fn from(iter: I) -> Self {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

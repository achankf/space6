pub trait Id: Clone + Copy + Into<usize> {
    type Source;

    /** Generates the next id from the source slice, by taking the length of the slice. */
    fn next(data: &[Self::Source]) -> Self;

    /** Projects the index onto the source slice. */
    fn project(self, data: &[Self::Source]) -> Option<&Self::Source> {
        data.get(self.into())
    }

    /**
    Mutable version of Id::project(). See [`Id::project`](Id::project())
    */
    fn project_mut(self, data: &mut [Self::Source]) -> Option<&mut Self::Source> {
        data.get_mut(self.into())
    }
}

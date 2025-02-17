pub trait Model: Sized {
    type NewT;
    type UpdateT;
    type C: diesel::Connection;

    fn new(data: Self::NewT) -> Self;

    fn create(
        conn: &mut Self::C,
        data: Self::NewT
    ) -> Result<Self, diesel::result::Error>;

    fn get_all(
        conn: &mut Self::C,
    ) -> Result<Vec<Self>, diesel::result::Error>;

    fn get_by_id(
        conn: &mut Self::C,
        sticker_id: &String
    ) -> Result<Self, diesel::result::Error>;

    fn delete(
        conn: &mut Self::C,
        sticker_id: &String
    ) -> Result<usize, diesel::result::Error>;

    fn update(
        conn: &mut Self::C,
        data: Self::UpdateT
    ) -> Result<(), diesel::result::Error>;
}
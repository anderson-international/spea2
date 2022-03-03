pub trait Population {
    fn initilise<T: Member>(&self) -> Vec<T>;
}
pub trait Member {
    fn randomise(&self);
}

pub fn update<T>(members: Vec<T>)
where
    T: Member,
{
    for member in members.iter() {
        member.randomise();
    }
}

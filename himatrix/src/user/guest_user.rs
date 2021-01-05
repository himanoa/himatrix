// No logged in user
pub trait GuestUser {
    fn login(&self) ;
    fn register(&self);
}

use bevy::prelude::*;

#[doc(hidden)]
pub use bevy_registration::app;

pub mod prelude {
    pub use super::{ComponentFieldBehaviour, ComponentContains};
    pub use procedural_macros::Behaviour;
}

/// Types that implement this trait contain some amount of T.
pub trait ComponentContains<T>: Component {
    fn get_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        T: 'a;
}

/// A struct that while it could be a component on its own, it more likely is a field inside a component.
/// Has some behaviour. Likely a generic system querying for the Contains<Self> trait.
pub trait ComponentFieldBehaviour: Sized {
    fn app<T: Component + ComponentContains<Self>>(app: &mut App);
}

// All of the nonsense below this comment makes use of https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html
// Basically, we allow ourselves to call maybe_app on MaybeApp<T>, and then if it actually implements ComponentFieldBehaviour, then it will run app.

#[doc(hidden)]
pub struct MaybeApp<T>(pub std::marker::PhantomData<T>);

#[doc(hidden)]
pub trait HasComponentFieldBehaviour<T>: Sized {
    fn maybe_app<C: Component + ComponentContains<T>>(&self, app: &mut App);
}
impl<T: ComponentFieldBehaviour> HasComponentFieldBehaviour<T> for &MaybeApp<T> {
    fn maybe_app<C: Component + ComponentContains<T>>(&self, app: &mut App) {
        T::app::<C>(app);
    }
}

#[doc(hidden)]
pub trait NoComponentFieldBehaviour<T>: Sized {
    fn maybe_app<C: Component + ComponentContains<T>>(&self, app: &mut App);
}
impl<T> NoComponentFieldBehaviour<T> for MaybeApp<T> {
    fn maybe_app<C: Component + ComponentContains<T>>(&self, _: &mut App) {}
}


/*
A Screen represents a single screen in the user interface.
Screens may have dynamic content that must be regularly updated.
This feature is represented in the Screen trait by the update_state
method. Typically the state update will occur asynchronously however
this is managed by a corresponding controller object.
*/

pub trait Screen<T> {
    fn update_state(&mut self, state: Box<T>);
    fn refresh(&mut self);
    fn clear(&mut self);
    fn paint(&mut self);
}


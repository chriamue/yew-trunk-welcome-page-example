use crate::Route;
use yew::prelude::*;

pub trait Switch {
    fn switch(&self, route: &Route) -> Html;
}

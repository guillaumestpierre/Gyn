use dioxus::prelude::*;

use crate::pages::home::Home;
use crate::pages::exos_list::ExosList;
use crate::pages::hist::Hist;
use crate::pages::poids::Poids;
use crate::pages::stats::Stats;
use crate::pages::new::New;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/exos")]
    ExosList {},
    #[route("/hist")]
    Hist {},
    #[route("/poids")]
    Poids {},
    #[route("/stats")]
    Stats {},
    #[route("/new")]
    New {},
}
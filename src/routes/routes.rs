use dioxus::prelude::*;

use crate::components::home::Home;
use crate::components::exos_list::ExosList;
use crate::components::hist::Hist;
use crate::components::poids::Poids;
use crate::components::notes::Notes;
use crate::components::stats::Stats;
use crate::components::new::New;

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
    #[route("/notes")]
    Notes {},
    #[route("/stats")]
    Stats {},
    #[route("/new")]
    New {},
}
use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{common::{ColorScale, PlotType, Visible}, Trace};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Choropleth {
    #[field_setter(default = "PlotType::Choropleth")]
    r#type: PlotType,
    name: Option<String>,
    visible: Option<Visible>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    z: Option<Vec<f64>>,
    geojson: Option<String>,
    locations: Option<Vec<String>>,
    #[serde(rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    #[serde(rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(rename = "showscale")]
    show_scale: Option<bool>,
}

impl Trace for Choropleth {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl Choropleth {
    pub fn new() -> Self {
        Default::default()
    }
}

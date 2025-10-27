use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{common::{ColorBar, ColorScale, LegendGroupTitle, PlotType, Visible}, layout::ColorAxis, Trace};


#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct ParCoords
{
    #[field_setter(default = "PlotType::ParCoords")]
    r#type: PlotType,
    name: Option<String>,
    visible: Option<Visible>,
    /// Sets the reference to a legend to show this trace in.
    legend: Option<bool>,
    /// Set and style the title to appear for the legend group.
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    /// Sets the legend rank for this trace. Items and groups with smaller ranks
    /// are presented on top/left side while with `"reversed"
    /// `legend.trace_order` they are on bottom/right side. The default
    /// legendrank is 1000, so that you can use ranks less than 1000 to
    /// place certain items before all unranked items, and ranks greater
    /// than 1000 to go after all unranked items.
    #[serde(rename = "legendrank")]
    legend_rank: Option<usize>,
    #[serde(rename = "legendwidth")]
    /// Sets the width (in px or fraction) of the legend for this trace.
    legend_width: Option<f64>,
    /// Assigns id labels to each datum. These ids for object constancy of data
    /// points during animation.
    ids: Option<Vec<String>>,

    dimensions: Option<Vec<Dimension>>,

    line: Option<Line>
}

impl Trace for ParCoords {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl ParCoords {
    pub fn new() -> Self {
        Default::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Line {
    #[serde(rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    cauto: Option<bool>,
    cmin: Option<f64>,
    cmax: Option<f64>,
    cmid: Option<f64>,
    color: Option<Vec<f64>>,
    #[serde(rename = "coloraxis")]
    color_axis: Option<ColorAxis>,
    #[serde(rename = "colorbar")]
    color_bar: Option<ColorBar>,
    #[serde(rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(rename = "showscale")]
    show_scale: Option<bool>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Dimension {
    label: Option<String>,
    values: Option<Vec<f64>>,
    range: Option<[f64; 2]>,
    visible: Option<bool>,
}

impl Dimension {
    pub fn new() -> Self {
        Default::default()
    }
}
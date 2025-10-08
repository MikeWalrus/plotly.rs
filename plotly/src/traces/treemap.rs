//! Treemap

use plotly_derive::FieldSetter;
use serde::{ser::Serializer, Serialize};

use crate::{
    color::Color,
    common::{Dim, Domain, Font, Label, LegendGroupTitle, Marker, PlotType, TextPosition, Visible},
    private::{NumOrString, NumOrStringCollection},
};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Treemap {
    #[field_setter(default = "PlotType::Treemap")]
    r#type: PlotType,
    /// Sets the trace name. The trace name appears as the legend item and on
    /// hover.
    name: Option<String>,
    /// Determines whether or not this trace is visible. If
    /// `Visible::LegendOnly`, the trace is not drawn, but can appear as a
    /// legend item (provided that the legend itself is visible).
    visible: Option<Visible>,
    #[serde(rename = "showlegend")]
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
    opacity: Option<f64>,
    /// Assigns id labels to each datum. These ids for object constancy of data
    /// points during animation.
    ids: Option<Vec<String>>,
    /// Sets the parent sectors for each of the sectors. Empty string items are
    /// understood to reference the root node in the hierarchy. If `ids` is
    /// filled, `parents` items are understood to be "ids" themselves. When
    /// `ids` is not set, plotly attempts to find matching items in `labels`,
    /// but beware they must be unique.
    parents: Option<Vec<String>>,
    /// Sets the values associated with each of the sectors. Use with
    /// `branchvalues` to determine how the values are summed.
    values: Option<Vec<f64>>,
    /// Sets the labels of each of the sectors.
    labels: Option<Vec<String>>,
    /// Sets text elements associated with each sector. If trace `textinfo`
    /// contains a `TextInfoFlag::Text`, these elements will be seen on the
    /// chart. If trace `hover_info` contains a `TextInfoFlag::Text` and
    /// `hovertext` is not set, these elements will be seen in the hover
    /// labels.
    text: Option<Vec<String>>,
    #[serde(rename = "textposition")]
    /// Sets the positions of the `text` elements.
    text_position: Option<TextPosition>,
    /// Template string used for rendering the information text that appear on
    /// points. Note that this will override textinfo. Variables are
    /// inserted using %{variable}, for example “y: %{y}”. Numbers are formatted
    /// using d3-format’s syntax %{variable:d3-format},  for example “Price:
    /// %{y:$.2f}”. <https://github.com/d3/d3-format/tree/v1.4.5#d3-format> for details on the formatting syntax.
    /// Dates are formatted using d3-time-format’s syntax
    /// %{variable|d3-time-format}, for example “Day: %{2019-01-01|%A}”. <https://github.com/d3/d3-time-format/tree/v2.2.3#locale_format> for details on the date formatting syntax.
    /// Every attributes that can be specified per-point (the ones that are
    /// arrayOk: true) are available. Finally, the template string has
    /// access to variables label, color, value, percent and text.
    #[serde(rename = "texttemplate")]
    text_template: Option<Dim<String>>,
    /// Sets hover text elements associated with each sector. If a single
    /// string, the same string appears for all data points. If an array of
    /// string, the items are mapped in order of this trace's sectors. To be
    /// seen, trace `hover_info` must contain a `TextInfoFlag::Text`.
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    /// Determines which trace information appear on hover. If `None` or `Skip`
    /// are set, no information is displayed upon hovering. But, if `None` is
    /// set, click and hover events are still fired.
    #[serde(rename = "hoverinfo")]
    hover_info: Option<Dim<HoverInfo>>,
    /// Template string used for rendering the information that appear on hover
    /// box. Note that this will override `HoverInfo`. Variables are
    /// inserted using %{variable}, for example "y: %{y}". Numbers are
    /// formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}".
    /// <https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3_format> for details
    /// on the formatting syntax. Dates are formatted using d3-time-format's
    /// syntax %{variable|d3-time-format}, for example "Day:
    /// %{2019-01-01|%A}". <https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format> for details
    /// on the date formatting syntax. The variables available in
    /// `hover_template` are the ones emitted as event data described at this link <https://plotly.com/javascript/plotlyjs-events/#event-data>.
    /// Additionally, every attributes that can be specified per-point (the ones
    /// that are `arrayOk: true`) are available. Anything contained in tag
    /// `<extra>` is displayed in the secondary box, for example
    /// "<extra>{fullData.name}</extra>". To hide the secondary box
    /// completely, use an empty tag `<extra></extra>`.
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    /// Assigns extra meta information associated with this trace that can be
    /// used in various text attributes. Attributes such as trace `name`, graph,
    /// axis and colorbar `title.text`, annotation `text` `rangeselector`,
    /// `updatemenues` and `sliders` `label` text all support `meta`. To access
    /// the trace `meta` values in an attribute in the same trace, simply use
    /// `%{meta[i]}` where `i` is the index or key of the `meta` item in
    /// question. To access trace `meta` in layout attributes, use
    /// `%{data[n[.meta[i]}` where `i` is the index or key of the `meta` and `n`
    /// is the trace index.
    meta: Option<NumOrString>,
    /// Assigns extra data each datum. This may be useful when listening to
    /// hover, click and selection events.
    #[serde(rename = "customdata")]
    custom_data: Option<NumOrStringCollection>,
    domain: Option<Domain>,
    marker: Option<Marker>,
    /// Sets the font used for `text_info`.
    #[serde(rename = "textfont")]
    text_font: Option<Font>,
    #[serde(rename = "textinfo")]
    text_info: Option<TextInfo>,
    /// Determines how the items in `values` are summed. When set to `Total`,
    /// items in `values` are taken to be value of all its descendants. When set
    /// to `Remainder`, items in `values` corresponding to the root and the
    /// branches sectors are taken to be the extra part not part of the sum of
    /// the values at their leaves.
    #[serde(rename = "branchvalues")]
    branch_values: Option<BranchValues>,
    /// Determines default for `values` when it is not provided, by inferring a
    /// 1 for each of the "Leaves" and/or "Branches", otherwise 0.
    count: Option<FlagList<CountFlag>>,
    tiling: Option<Tiling>,
    #[serde(rename = "pathbar")]
    path_bar: Option<PathBar>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    /// Sets the font used for `text_info` lying inside the sector.
    #[serde(rename = "insidetextfont")]
    inside_text_font: Option<Font>,
    /// Sets the font used for `text_info` lying outside the sector.
    #[serde(rename = "outsidetextfont")]
    outside_text_font: Option<Font>,
    root: Option<Node>,
    /// Sets the level from which this trace hierarchy is rendered. Set `level`
    /// to empty string to start from the root node in the hierarchy. Must be an
    /// "id" if `ids` is filled in, otherwise plotly attempts to find a matching
    /// item in `labels`.
    level: Option<NumOrString>,
    /// Sets the number of rendered sectors from any given `level`. Set
    /// `maxdepth` to "-1" to render all the levels in the hierarchy.
    #[serde(rename = "maxdepth")]
    max_depth: Option<i32>,
    /// Determines whether or not the sectors are reordered from largest to
    /// smallest.
    sort: Option<bool>,
    /// Controls persistence of some user-driven changes to the trace:
    /// `constraintrange` in `parcoords` traces, as well as some `editable:
    /// true` modifications such as `name` and `colorbar.title`. Defaults to
    /// `layout.uirevision`. Note that other user-driven trace attribute changes
    /// are controlled by `layout` attributes: `trace.visible` is controlled by
    /// `layout.legend.uirevision`, `selectedpoints` is controlled by
    /// `layout.selectionrevision`, and `colorbar.(x|y)` (accessible with
    /// `config: {editable: true}`) is controlled by `layout.editrevision`.
    /// Trace changes are tracked by `uid`, which only falls back on trace index
    /// if no `uid` is provided. So if your app can add/remove traces before the
    /// end of the `data` array, such that the same trace has a different index,
    /// you can still preserve user-driven changes if you give each trace a
    /// `uid` that stays with it as it moves.
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl Treemap {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TextInfoFlag {
    Label,
    Text,
    Value,
    Name,
    CurrentPath,
    PercentRoot,
    PercentEntry,
    PercentParent,
}

#[derive(Clone, Debug)]
pub struct FlagList<Flag: Serialize>(Vec<Flag>);

impl<Flag: Serialize> Serialize for FlagList<Flag> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let flag_strings: Vec<String> = self
            .0
            .iter()
            .map(|flag| {
                serde_json::to_value(flag)
                    .ok()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_else(|| "".to_string())
            })
            .collect();
        serializer.serialize_str(&flag_strings.join("+"))
    }
}

#[derive(Clone, Debug)]
pub enum HoverInfo {
    Flags(FlagList<TextInfoFlag>),
    All,
    None,
    Skip,
}

impl Serialize for HoverInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            HoverInfo::Flags(flags) => flags.serialize(serializer),
            HoverInfo::All => serializer.serialize_str("all"),
            HoverInfo::None => serializer.serialize_str("none"),
            HoverInfo::Skip => serializer.serialize_str("skip"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum TextInfo {
    Flags(Vec<TextInfoFlag>),
    None,
}

impl Serialize for TextInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TextInfo::Flags(flags) => FlagList(flags.clone()).serialize(serializer),
            TextInfo::None => serializer.serialize_str("none"),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BranchValues {
    Total,
    Remainder,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CountFlag {
    Branches,
    Leaves,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Tiling {
    /// Determines if the positions obtained from solver are flipped on each
    /// axis.
    flip: Option<FlagList<FlipFlag>>,
    /// Determines d3 treemap solver.
    /// For more info please refer to
    /// <https://github.com/d3/d3-hierarchy#treemap-tiling>
    packing: Option<Packing>,
    /// Sets the inner padding (in px).
    padding: Option<i32>,
    /// When using "squarify" `packing` algorithm, according to <https://github.com/d3/d3-hierarchy/blob/v3.1.1/README.md#squarify_ratio>
    /// this option specifies the desired aspect ratio of the generated
    /// rectangles. The ratio must be specified as a number greater than or
    /// equal to one. Note that the orientation of the generated rectangles
    /// (tall or wide) is not implied by the ratio; for example, a ratio of two
    /// will attempt to produce a mixture of rectangles whose width:height ratio
    /// is either 2:1 or 1:2. When using "squarify", unlike d3 which uses the
    /// Golden Ratio i.e. 1.618034, Plotly applies 1 to increase squares in
    /// treemap layouts.
    #[serde(rename = "squarifyratio")]
    squarify_ratio: Option<f64>,
}

impl Tiling {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FlipFlag {
    X,
    Y,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Packing {
    Squarify,
    Binary,
    Dice,
    Slice,
    #[serde(rename = "slice-dice")]
    SliceDice,
    #[serde(rename = "dice-slice")]
    DiceSlice,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct PathBar {
    /// Determines which shape is used for edges between `barpath` labels.
    #[serde(rename = "edgeshape")]
    edge_shape: Option<EdgeShape>,
    /// Determines on which side of the the treemap the `pathbar` should be
    /// presented.
    side: Option<PathBarSide>,
    /// Sets the font used inside `PathBar`.
    #[serde(rename = "textfont")]
    text_font: Option<Font>,
    /// Sets the thickness of `PathBar` (in px). If not specified the
    /// `pathbar.textfont.size` is used with 3 pixels extra padding on each
    /// side.
    thickness: Option<i32>,
    /// Determines if the path bar is drawn i.e. outside the trace `domain` and
    /// with one pixel gap.
    visible: Option<bool>,
}

impl PathBar {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Clone, Debug)]
pub enum EdgeShape {
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = "|")]
    Pipe,
    #[serde(rename = "/")]
    ForwardSlash,
    #[serde(rename = "\\")]
    BackSlash,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PathBarSide {
    Top,
    Bottom,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
pub struct Node {
    color: Option<Box<dyn Color>>,
}

impl Node {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn serialize_treemap() {
        let treemap_trace = Treemap::new()
            .name("Sample Treemap")
            .visible(Visible::True)
            .legend(true)
            .legend_group_title(LegendGroupTitle::default().text("Group Title"))
            .legend_rank(1)
            .legend_width(200.0)
            .opacity(0.8)
            .ids(vec!["id1".to_string(), "id2".to_string()])
            .parents(vec!["".to_string(), "A".to_string()])
            .values(vec![10.0, 20.0])
            .labels(vec!["A".to_string(), "B".to_string()])
            .text(vec!["Text A".to_string(), "Text B".to_string()])
            .text_position(TextPosition::Inside)
            .text_template("%{label}")
            .hover_text("Hover Text")
            .hover_info(HoverInfo::All)
            .hover_template("%{label}")
            .meta("Meta Info".to_string())
            .custom_data(vec!["Custom Data".to_string()])
            .domain(Domain::new())
            .marker(Marker::new())
            .text_font(Font::new())
            .text_info(TextInfo::Flags(vec![
                TextInfoFlag::Label,
                TextInfoFlag::Value,
            ]))
            .branch_values(BranchValues::Total)
            .count(FlagList(vec![CountFlag::Leaves]))
            .tiling(
                Tiling::new()
                    .packing(Packing::Squarify)
                    .squarify_ratio(1.5)
                    .padding(2),
            )
            .path_bar(
                PathBar::new()
                    .edge_shape(EdgeShape::GreaterThan)
                    .side(PathBarSide::Top)
                    .thickness(30)
                    .visible(true),
            )
            .hover_label(Label::new())
            .inside_text_font(Font::new())
            .outside_text_font(Font::new())
            .root(Node::new().color("rgba(0, 0, 0)".to_string()))
            .level("Level".to_string())
            .max_depth(3)
            .sort(true)
            .ui_revision("Revision".to_string());

        let expected = json!({
            "type": "treemap",
            "name": "Sample Treemap",
            "visible": true,
            "showlegend": true,
            "legendgrouptitle": { "text": "Group Title" },
            "legendrank": 1,
            "legendwidth": 200.0,
            "opacity": 0.8,
            "ids": ["id1", "id2"],
            "parents": ["", "A"],
            "values": [10.0, 20.0],
            "labels": ["A", "B"],
            "text": ["Text A", "Text B"],
            "textposition": "inside",
            "texttemplate": "%{label}",
            "hovertext": "Hover Text",
            "hoverinfo": "all",
            "hovertemplate": "%{label}",
            "meta": "Meta Info",
            "customdata": ["Custom Data"],
            "domain": {},
            "marker": {},
            "textfont": {},
            "textinfo": "label+value",
            "branchvalues": "total",
            "count": "leaves",
            "tiling": {
                "packing": "squarify",
                "squarifyratio": 1.5,
                "padding": 2
            },
            "pathbar": {
                "edgeshape": ">",
                "side": "top",
                "thickness": 30,
                "visible": true
            },
            "hoverlabel": {},
            "insidetextfont": {},
            "outsidetextfont": {},
            "root": {
                "color": "rgba(0, 0, 0)"
            },
            "level": "Level",
            "maxdepth": 3,
            "sort": true,
            "uirevision": "Revision"
        });

        assert_eq!(to_value(treemap_trace).unwrap(), expected);
    }
}

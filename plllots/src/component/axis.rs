use crate::{primitives::AppendPrimitives, utils::calculate_axis_ticks};
use bon::Builder;
use kurbo::{Point, Stroke};
use peniko::{Brush, Color};
use svg::{
    Document, Node,
    node::element::{Path, Text, path::Data},
};

#[derive(Debug, Clone)]
pub enum CartesianAxis {
    Category(Vec<String>),
    /// Numerical data (values)
    Values,
}

/// X-axis configuration and data.
#[derive(Debug, Builder, Clone)]
pub struct XAxis {
    #[builder(default = Stroke::new(1.0))]
    pub stroke: Stroke,
    #[builder(default = peniko::Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub axis_color: Brush,
    #[builder(default = peniko::Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub label_color: Brush,
    pub cartesian_axis: CartesianAxis,
}

/// Y-axis configuration and data.
#[derive(Debug, Builder, Clone)]
pub struct YAxis {
    #[builder(default = Stroke::new(1.0))]
    pub stroke: Stroke,
    #[builder(default = peniko::Brush::Solid(Color::from_rgba8(0xe0, 0xe6, 0xf1, 0xff)))]
    pub axis_color: Brush,
    #[builder(default = peniko::Brush::Solid(Color::from_rgba8(0x6e, 0x70, 0x79, 0xff)))]
    pub label_color: Brush,
    pub cartesian_axis: CartesianAxis,
}

/// Helper structures for axis calculations.
#[derive(Debug, Clone)]
pub enum AxisHelper {
    Category(AxisCategoryHelper),
    Values(AxisValuesHelper),
}

/// Helper for categorical axis calculations.
#[derive(Debug, Clone)]
pub struct AxisCategoryHelper {
    pub amount: usize,
}

/// Helper for numerical axis calculations.
#[derive(Debug, Clone)]
pub struct AxisValuesHelper {
    pub min: f64,
    pub max: f64,
    pub step_size: f64,
}

// impl<'a> AppendPrimitives<'a> for YAxis {
//     fn append_primitives(
//         &'a self,
//         primitives: &mut Vec<crate::primitives::Primitives<'a>>,
//         helper: &mut crate::chart::ChartPlotHelper,
//     ) {
//         match &self.cartesian_axis {
//             CartesianAxis::Category(_items) => todo!(),
//             CartesianAxis::Values(items) => {
//                 let (min, max, step_size) = calculate_axis_ticks(&items);
//                 let sub_tick_spacing = helper.offsets.y_span / (max / step_size);
//                 for sub_tick_index in 1..((max / step_size) as i32 + 1) {
//                     let sub_tick_height =
//                         helper.offsets.y_axis_end - (sub_tick_index as f64 * sub_tick_spacing);

//                     let line = crate::primitives::Line {
//                         stroke: &self.stroke,
//                         stroke_color: &self.axis_color,
//                         coords: (
//                             Point::new(helper.offsets.x_axis_start, sub_tick_height),
//                             Point::new(helper.offsets.x_axis_end, sub_tick_height),
//                         ),
//                     };

//                     primitives.push(crate::primitives::Primitives::Line(line));
//                 }

//                 for sub_tick_index in 0..((max / step_size) as i32 + 1) {
//                     let sub_tick_height =
//                         helper.offsets.y_axis_end - (sub_tick_index as f64 * sub_tick_spacing);
//                     let text = crate::primitives::Text {
//                         text: format!("{}", min + step_size * sub_tick_index as f64),
//                         fill_color: &self.label_color,
//                         font_size: 12.0,
//                         text_anchor: parley::Alignment::End,
//                         coord: Point::new(helper.offsets.x_axis_start - 8.0, sub_tick_height),
//                     };
//                     primitives.push(crate::primitives::Primitives::Text(text));
//                 }

//                 // TODO calulate helper earlier
//                 helper.y_axis = Some(AxisHelper::Values(AxisValuesHelper {
//                     min,
//                     max,
//                     step_size,
//                 }));

//                 // TODO: Data does not belong here
//                 let mut path = crate::primitives::Path {
//                     stroke: &self.stroke,
//                     stroke_color: &self.axis_color,
//                     coords: Vec::with_capacity(items.len()),
//                 };
//                 for (index, y_item) in items.iter().enumerate() {
//                     let y_pos = if let AxisHelper::Values(y_axis_helper) =
//                         &helper.y_axis.as_ref().unwrap()
//                     {
//                         let percentage_height = y_item / y_axis_helper.max;
//                         helper.offsets.y_axis_end - (percentage_height * helper.offsets.y_span)
//                     } else {
//                         unreachable!()
//                     };

//                     match helper.x_axis.as_ref().unwrap() {
//                         AxisHelper::Category(axis_category_helper) => {
//                             let x_spacing =
//                                 helper.offsets.x_span / axis_category_helper.amount as f64;
//                             let x_pos =
//                                 helper.offsets.x_axis_start + (index as f64 + 0.5) * x_spacing;
//                             path.coords.push(Point::new(x_pos, y_pos));
//                         }
//                         AxisHelper::Values(axis_values_helper) => todo!(),
//                     }
//                 }
//                 primitives.push(crate::primitives::Primitives::Path(path));
//             }
//         }
//     }
// }

// impl<'a> AppendPrimitives<'a> for XAxis {
//     fn append_primitives(
//         &'a self,
//         primitives: &mut Vec<crate::primitives::Primitives<'a>>,
//         helper: &mut crate::chart::ChartPlotHelper,
//     ) {
//         match &self.cartesian_axis {
//             CartesianAxis::Category(items) => {
//                 let line = crate::primitives::Line {
//                     stroke: &self.stroke,
//                     stroke_color: &self.axis_color,
//                     coords: (
//                         Point::new(helper.offsets.x_axis_start, helper.offsets.y_axis_end),
//                         Point::new(helper.offsets.x_axis_end, helper.offsets.y_axis_end),
//                     ),
//                 };

//                 primitives.push(crate::primitives::Primitives::Line(line));

//                 let label_spacing = helper.offsets.x_span / items.len() as f64;
//                 for label_index in 0..=items.len() {
//                     let x_pos = helper.offsets.x_axis_start + label_index as f64 * label_spacing;
//                     let y_pos = helper.offsets.y_axis_end + 8.0;
//                     let line = crate::primitives::Line {
//                         stroke: &self.stroke,
//                         stroke_color: &self.axis_color,
//                         coords: (
//                             Point::new(x_pos, helper.offsets.y_axis_end),
//                             Point::new(x_pos, y_pos),
//                         ),
//                     };
//                     primitives.push(crate::primitives::Primitives::Line(line));
//                 }

//                 for (label_index, label) in items.iter().enumerate() {
//                     let x_pos =
//                         helper.offsets.x_axis_start + (label_index as f64 + 0.5) * label_spacing;
//                     let y_pos = helper.offsets.y_axis_end + 14.0;
//                     let text = crate::primitives::Text {
//                         text: label.to_string(),
//                         fill_color: &self.label_color,
//                         font_size: 12.0,
//                         text_anchor: parley::Alignment::Middle,
//                         coord: Point::new(x_pos, y_pos),
//                     };
//                     primitives.push(crate::primitives::Primitives::Text(text));
//                 }

//                 // TODO calulate helper earlier
//                 helper.x_axis = Some(AxisHelper::Category(AxisCategoryHelper {
//                     amount: items.len(),
//                 }))
//             }
//             CartesianAxis::Values(_items) => todo!(),
//         }
//     }
// }

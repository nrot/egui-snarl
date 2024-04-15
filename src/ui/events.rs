//! Keyboard/mouse work with graph
//! 

use std::fmt::Debug;

use egui::{InputState, PointerButton, Response};
#[cfg(feature="egui-probe")]
use egui_probe::EguiProbe;
#[cfg(feature="serde")]
use serde::{Deserialize, Serialize};

///An input event generated by the integration.
pub trait GraphEvents {
    /// Remove hovered wire by second click
    fn remove_hovered_wire(
        &mut self,
        background_response: &Response,
        input_state: &InputState,
    ) -> bool {
        background_response.clicked_by(PointerButton::Secondary)
    }

    /// Start select area
    fn start_select_area(
        &mut self,
        background_response: &Response,
        input_state: &InputState,
    ) -> bool {
        background_response.drag_started_by(PointerButton::Primary) && input_state.modifiers.shift
    }

    /// Stop select area
    fn stop_select_area(
        &mut self,
        background_response: &Response,
        input_state: &InputState,
    ) -> bool {
        background_response.drag_stopped_by(PointerButton::Primary)
    }

    /// Move area by
    fn move_area(&mut self, background_response: &Response, input_state: &InputState) -> bool {
        background_response.dragged_by(PointerButton::Primary)
    }

    /// Move area delta
    fn move_area_delta(&mut self, background_response: &Response, input_state: &InputState) -> egui::Vec2 {
        -background_response.drag_delta()
    }

    /// Cancel new wire
    fn cancel_new_wire(
        &mut self,
        background_response: &Response,
        input_state: &InputState,
    ) -> bool {
        input_state.pointer.button_down(PointerButton::Secondary)
    }

    ///Do centering
    fn do_centering(&mut self, background_response: &Response, input_state: &InputState) -> bool {
        background_response.double_clicked()
    }

    ///Deselect all nodes
    fn deselect_all_nodes(
        &mut self,
        background_response: &Response,
        input_state: &InputState,
    ) -> bool {
        input_state.modifiers.command && background_response.clicked_by(PointerButton::Primary)
    }

    /// Node move
    fn node_move(&mut self, response: &Response, input_state: &InputState) -> bool {
        !input_state.modifiers.shift
            && !input_state.modifiers.command
            && response.dragged_by(PointerButton::Primary)
    }

    /// Node move delta
    fn node_move_delta(&mut self, response: &Response, input_state: &InputState) -> egui::Vec2 {
        response.drag_delta()
    }

    /// Select one node
    fn select_one_node(&mut self, response: &Response, input_state: &InputState) -> bool {
        (response.clicked_by(PointerButton::Primary) || response.dragged_by(PointerButton::Primary))
            && input_state.modifiers.shift
    }

    /// Deselect one node
    fn deselect_one_node(&mut self, response: &Response, input_state: &InputState) -> bool {
        (response.clicked_by(PointerButton::Primary) || response.dragged_by(PointerButton::Primary))
            && input_state.modifiers.command
    }

    /// Node to top
    fn not_to_top(&mut self, response: &Response, input_state: &InputState) -> bool {
        response.clicked() || response.dragged() 
    }

    /// Remove or drop new node
    fn remove_wire(&mut self, response: &Response, input_state: &InputState) -> bool {
        response.clicked_by(PointerButton::Secondary)
    }

    ///Start drag wire
    fn start_drag_wire(&mut self, response: &Response, input_state: &InputState) -> bool {
        response.drag_started_by(PointerButton::Primary)
    }

    ///Stop drag wire
    fn stop_drag_wire(&mut self, response: &Response, input_state: &InputState) -> bool {
        response.drag_stopped()
    }

    /// Start new wire out
    fn start_new_wire_out(&mut self, response: &Response, input_state: &InputState)->bool{
        response.drag_started_by(PointerButton::Primary) && input_state.modifiers.command
    }

    /// Start new wire in
    fn start_new_wire_in(&mut self, response: &Response, input_state: &InputState)->bool{
        response.drag_started_by(PointerButton::Primary) && !input_state.modifiers.command
    }

    /// Drop inputs 
    fn drop_inputs_pin(&mut self, response: &Response, input_state: &InputState)->bool{
        response.drag_started_by(PointerButton::Primary) && input_state.modifiers.command && !input_state.modifiers.shift
    }
}

/// Default Egui-Snarl keybindings
#[derive(Debug, PartialEq, Default)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "egui-probe", derive(egui_probe::EguiProbe))]
pub struct DefaultGraphEvents {}

impl GraphEvents for DefaultGraphEvents {}
impl GraphEventsExtend for DefaultGraphEvents {}

#[cfg(all(feature = "serde", not(feature = "egui-probe")))]
/// Trait GraphEvents with feature 
pub trait GraphEventsExtend : GraphEvents + Deserialize + Serialize {}

#[cfg(all(not(feature = "serde"), feature = "egui-probe"))]
/// Trait GraphEvents with feature 
pub trait GraphEventsExtend : GraphEvents + EguiProbe{}



#[cfg(all(feature = "serde", feature = "egui-probe"))]
/// Trait GraphEvents with feature 
pub trait GraphEventsExtend: GraphEvents  + Serialize + EguiProbe {}

#[cfg(all(not(feature = "serde"), not(feature = "egui-probe")))]
/// Trait GraphEvents with feature 
pub trait GraphEventsExtend : GraphEvents{}

#[derive(Debug, PartialEq, Default)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "egui-probe", derive(egui_probe::EguiProbe))]
enum ItemDragged{
    #[default]
    None,
    Node,
    Wire
}


/// Click and move wire
#[derive(Debug, PartialEq, Default)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "egui-probe", derive(egui_probe::EguiProbe))]
pub struct ClickGraphEvents{
    drag: ItemDragged
}

impl GraphEvents for ClickGraphEvents{
    fn node_move(&mut self, response: &Response, input_state: &InputState) -> bool {
        // !input_state.modifiers.shift
        // && !input_state.modifiers.command
        // && response.dragged_by(PointerButton::Primary)
        match self.drag{
            ItemDragged::None => {
                if !input_state.modifiers.shift && !input_state.modifiers.command && response.clicked_by(PointerButton::Primary){
                    self.drag = ItemDragged::Node;
                    println!("Start drag node");
                    true
                } else {
                    false
                }
            },
            ItemDragged::Node => {
                if input_state.key_pressed(egui::Key::Escape) || response.clicked_by(PointerButton::Primary){
                    self.drag = ItemDragged::None;
                    println!("Stop drag node");
                    false
                } else {
                    true
                }
            },
            ItemDragged::Wire => {
                false
            },
        }
    }

    fn node_move_delta(&mut self, response: &Response, input_state: &InputState) -> egui::Vec2 {
        if self.drag == ItemDragged::Node{
            input_state.pointer.delta()
        } else {
            egui::Vec2::ZERO
        }
    }

}

impl GraphEventsExtend for ClickGraphEvents{}
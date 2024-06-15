use godot::classes::*;
use godot::global::{MouseButton, MouseButtonMask};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Camera2D)]
pub(crate) struct Camera {
    #[export]
    min_zoom: f32,
    #[export]
    max_zoom: f32,
    #[export]
    zoom_rate: f32,
    #[export]
    zoom_delta: f32,
    #[export]
    drag_speed: f32,
    target_zoom: OnReady<f32>,
    base: Base<Camera2D>,
}

#[godot_api]
impl ICamera2D for Camera {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            min_zoom: 0.15,
            max_zoom: 1.0,
            zoom_rate: 16.0,
            zoom_delta: 0.05,
            drag_speed: 1.0,
            target_zoom: OnReady::manual(),
            base,
        }
    }

    fn ready(&mut self) {
        self.target_zoom.init(self.base().get_zoom().x);
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        let event = match event.try_cast::<InputEventMouseButton>() {
            Ok(mouse_event) => {
                if mouse_event.is_pressed() {
                    if mouse_event.get_button_index() == MouseButton::WHEEL_UP {
                        self.target_zoom.set_property(f32::min(
                            self.target_zoom.get_property() + self.zoom_delta,
                            self.max_zoom,
                        ));
                    } else if mouse_event.get_button_index() == MouseButton::WHEEL_DOWN {
                        self.target_zoom.set_property(f32::max(
                            self.target_zoom.get_property() - self.zoom_delta,
                            self.min_zoom,
                        ));
                    } else if mouse_event.get_button_index() == MouseButton::MIDDLE {
                        Input::singleton()
                            .set_default_cursor_shape_ex()
                            .shape(input::CursorShape::DRAG)
                            .done();
                    }
                } else if mouse_event.get_button_index() == MouseButton::MIDDLE {
                    Input::singleton()
                        .set_default_cursor_shape_ex()
                        .shape(input::CursorShape::ARROW)
                        .done();
                }
                return;
            }
            Err(event) => event,
        };
        let event = match event.try_cast::<InputEventMouseMotion>() {
            Ok(mouse_motion_event) => {
                if mouse_motion_event.get_button_mask() == MouseButtonMask::MIDDLE {
                    let new_position = self.base().get_position()
                        - (mouse_motion_event.get_relative() * self.drag_speed
                            / self.base().get_zoom());
                    self.base_mut().set_position(new_position);
                }
                return;
            }
            Err(event) => event,
        };
    }

    fn physics_process(&mut self, _delta: f64) {
        let current_zoom = self.base().get_zoom();
        let target = self.target_zoom.clone();
        let weight = self.zoom_delta * self.zoom_rate;
        let target_zoom = Vector2 {
            x: target,
            y: target,
        };
        self.base_mut()
            .set_zoom(current_zoom.lerp(target_zoom, weight));
    }
}

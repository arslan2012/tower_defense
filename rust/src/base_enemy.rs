use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::classes::NavigationAgent2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct BaseEnemy {
    #[export]
    speed: f32,
    nav_agent: OnReady<Gd<NavigationAgent2D>>,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for BaseEnemy {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            speed: 150.0,
            nav_agent: OnReady::manual(),
            base,
        }
    }

    fn ready(&mut self) {
        self.nav_agent.init(
            self.base()
                .get_node_as::<NavigationAgent2D>("NavigationAgent2D"),
        );
        self.nav_agent.set_max_speed(self.speed);
        let objective = self.base().get_node_as::<Node2D>("/root/Main/Objective");
        self.nav_agent
            .set_target_position(objective.get_global_position());
    }

    fn physics_process(&mut self, _delta: f64) {
        let next_position = self.nav_agent.get_next_path_position();
        let current_position = self.base().get_global_position();
        let new_velocity = current_position.direction_to(next_position) * self.speed;
        self.base_mut().set_velocity(new_velocity);
        self.base_mut().move_and_slide();
    }
}

use crate::prelude::*;

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_agents_system);
        //app.add_system(update_agents_system);
    }
}


fn spawn_agents_system(mut commands: Commands) {
    let mut rnd = thread_rng();
    let shape = shapes::Circle {center: Vec2 { x: 0.0, y: 0.0 }, radius: 10.0};
    let y = rnd.gen_range(-WIN_SIZE.y/2.0..WIN_SIZE.y/2.0);
    let x = rnd.gen_range(-WIN_SIZE.x/2.0..WIN_SIZE.x/2.0);
    commands.spawn((ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..Default::default()
        },
        Stroke::new(Color::BLUE, 4.0),
        Fill::color(Color::YELLOW),
    ))
    .insert(TransformBundle::from_transform(Transform::from_xyz(x, y, 0.0)))
    .insert(RigidBody::Dynamic)
    .insert(Collider::ball(10.0))
    .insert(Sleeping::disabled())
    .insert(Damping {linear_damping: 0.2, angular_damping: 0.5});
}
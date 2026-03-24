
pub trait GameObject {
    fn update(deltaTime: Duration);
    fn render();
}

pub struct Game<GameObjectTypeetc: GameObject> {

}


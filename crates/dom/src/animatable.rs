pub struct Keyframe;
pub enum AnimationOptions {
    Number(usize),
    KeyframeAnimationOptions {
        composite: Option<()>,
        id: Option<String>,
        iteration_composite: Option<()>,
        pseudo_element: Option<String>,
    },
}

pub struct GetAnimationOptions;
pub struct Animation;

pub trait Animatable {
    fn animate(&mut self, keyframes: Option<Vec<Keyframe>>, options: Option<AnimationOptions>);
    fn get_animations(&self, options: Option<GetAnimationOptions>) -> Vec<Animation>;
}

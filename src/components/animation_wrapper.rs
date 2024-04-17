use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AnimationWrapperProps
{
    pub children: Children,
	pub reset: bool,
	pub hidden: Option<bool>,
	pub id: Option<&'static str>,
	pub class: Option<&'static str>,
	pub animation_class: &'static str
}

#[function_component(AnimationWrapper)]
pub fn animation_wrapper(AnimationWrapperProps { children, hidden, reset, id, class, animation_class }: &AnimationWrapperProps) -> Html
{
	html!
	{
		<div
			id={*id}
			hidden={if let Some(hidden) = hidden { *hidden } else { false }}
			class={format!("{} {}", if let Some(class) = class { *class } else { "" }, if *reset { *animation_class } else { "" })}
		>
			{children.clone()}
		</div>
	}
}
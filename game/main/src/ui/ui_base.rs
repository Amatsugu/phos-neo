use bevy::{input_focus::InputFocus, prelude::*};

use crate::ui::{
	components::*,
	events::{Blur, Hover, Press},
};

pub struct BaseUIPlugin;

impl Plugin for BaseUIPlugin
{
	fn build(&self, app: &mut App)
	{
		app.init_resource::<InputFocus>();
		app.add_systems(Update, buttons);
		app.add_observer(on_hover_bg)
			.add_observer(on_blur_bg)
			.add_observer(on_press_bg)
			.add_observer(on_hover_border)
			.add_observer(on_blur_border)
			.add_observer(on_press_border)
			.add_observer(on_hover_text)
			.add_observer(on_blur_text)
			.add_observer(on_press_text);

		app.add_observer(base_color)
			.add_observer(base_border_color)
			.add_observer(base_text_color);
	}
}

fn base_color(added: On<Add, BaseColor>, mut colors: Query<(&mut BackgroundColor, &BaseColor)>)
{
	if let Ok((mut bg, color)) = colors.get_mut(added.entity) {
		bg.0 = color.0;
	}
}

fn base_border_color(added: On<Add, BaseBorderColor>, mut colors: Query<(&mut BorderColor, &BaseBorderColor)>)
{
	if let Ok((mut border, color)) = colors.get_mut(added.entity) {
		border.set_all(color.0);
	}
}

fn base_text_color(added: On<Add, BaseTextColor>, mut colors: Query<(&mut TextColor, &BaseTextColor)>)
{
	if let Ok((mut text, color)) = colors.get_mut(added.entity) {
		text.0 = color.0;
	}
}

fn buttons(
	mut input_focus: ResMut<InputFocus>,
	mut buttons: Query<(Entity, &Interaction, &mut Button), Changed<Interaction>>,
	mut commands: Commands,
)
{
	for (entity, interaction, mut button) in &mut buttons {
		match *interaction {
			Interaction::Pressed => {
				input_focus.set(entity);
				button.set_changed();
				commands.entity(entity).trigger(Press);
			}
			Interaction::Hovered => {
				input_focus.set(entity);
				button.set_changed();
				commands.entity(entity).trigger(Hover);
			}
			Interaction::None => {
				input_focus.clear();
				commands.entity(entity).trigger(Blur);
			}
		}
	}
}

fn on_hover_bg(hover: On<Hover>, mut backgrounds: Query<(&mut BackgroundColor, &HoverColor)>)
{
	if let Ok((mut bg, color)) = backgrounds.get_mut(hover.0) {
		bg.0 = color.0;
	}
}

fn on_blur_bg(hover: On<Blur>, mut backgrounds: Query<(&mut BackgroundColor, &BaseColor)>)
{
	if let Ok((mut bg, color)) = backgrounds.get_mut(hover.0) {
		bg.0 = color.0;
	}
}

fn on_press_bg(hover: On<Press>, mut backgrounds: Query<(&mut BackgroundColor, &PressedColor)>)
{
	if let Ok((mut bg, color)) = backgrounds.get_mut(hover.0) {
		bg.0 = color.0;
	}
}

fn on_hover_border(hover: On<Hover>, mut backgrounds: Query<(&mut BorderColor, &HoverBorderColor)>)
{
	if let Ok((mut border, color)) = backgrounds.get_mut(hover.0) {
		border.set_all(color.0);
	}
}

fn on_blur_border(hover: On<Blur>, mut backgrounds: Query<(&mut BorderColor, &BaseBorderColor)>)
{
	if let Ok((mut border, color)) = backgrounds.get_mut(hover.0) {
		border.set_all(color.0);
	}
}

fn on_press_border(hover: On<Press>, mut backgrounds: Query<(&mut BorderColor, &PressedBorderColor)>)
{
	if let Ok((mut border, color)) = backgrounds.get_mut(hover.0) {
		border.set_all(color.0);
	}
}

fn on_hover_text(hover: On<Hover>, mut backgrounds: Query<(&mut TextColor, &HoverTextColor)>)
{
	if let Ok((mut text, color)) = backgrounds.get_mut(hover.0) {
		text.0 = color.0;
	}
}

fn on_blur_text(hover: On<Blur>, mut backgrounds: Query<(&mut TextColor, &BaseTextColor)>)
{
	if let Ok((mut text, color)) = backgrounds.get_mut(hover.0) {
		text.0 = color.0;
	}
}

fn on_press_text(hover: On<Press>, mut backgrounds: Query<(&mut TextColor, &PressedTextColor)>)
{
	if let Ok((mut text, color)) = backgrounds.get_mut(hover.0) {
		text.0 = color.0;
	}
}

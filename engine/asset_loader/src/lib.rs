pub mod macros {

	#[macro_export]
	macro_rules! create_asset_loader {
		(
			$plugin_name: ident,
			$loader_name: ident,
			$asset_type: ident,
			$extensions: expr,
			$($string_name: ident -> $handle_name: ident)*
		) => {
			use bevy::prelude::*;
			use bevy::asset::{AssetLoader, AssetEvent, LoadContext, AsyncReadExt, io::Reader};
			use bevy::utils::BoxedFuture;
			pub struct $plugin_name;
			impl Plugin for $plugin_name {
				fn build(&self, app: &mut App) {
					app.init_asset::<$asset_type>()
						.init_asset_loader::<$loader_name>()
						.add_systems(Update, finalize);
				}
			}

			fn finalize(
				mut asset_events: EventReader<AssetEvent<$asset_type>>,
				mut assets: ResMut<Assets<$asset_type>>,
				 asset_server: Res<AssetServer>
			) {
				for event in asset_events.read() {
					match event {
						AssetEvent::LoadedWithDependencies { id } => {
							let asset = assets.get_mut(id.clone()).unwrap();
							$(
								asset.$handle_name = asset_server.load(&asset.$string_name);
							)*
						},
						_ => (),
					}
				}
			}

			#[derive(Default)]
			pub struct $loader_name;

			impl AssetLoader for $loader_name {
				type Asset = $asset_type;

				type Settings = ();

				type Error = String;

				fn load<'a>(
					&'a self,
					reader: &'a mut Reader,
					_settings: &'a Self::Settings,
					_load_context: &'a mut LoadContext,
				) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
					return Box::pin(async move {
						let mut data: String = String::new();
						let read_result = reader.read_to_string(&mut data).await;
						if read_result.is_err() {
							return Err(read_result.err().unwrap().to_string());
						}
						let serialized: Result<Self::Asset, serde_json::Error> =
							serde_json::from_str(&data);
						if serialized.is_err() {
							return Err(serialized.err().unwrap().to_string());
						}
						return Ok(serialized.unwrap());
					});
				}

				fn extensions(&self) -> &[&str] {
					$extensions
				}
			}
		};
	}
}

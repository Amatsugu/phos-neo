pub mod macros {

	#[macro_export]
	macro_rules! create_asset_loader {
		(
			$plugin_name: ident,
			$loader_name: ident,
			$asset_type: ident,
			$extensions: expr
		) => {
			use bevy::prelude::*;
			pub struct $plugin_name;
			impl Plugin for $plugin_name {
				fn build(&self, app: &mut App) {
					app.init_asset::<$asset_type>()
						.init_asset_loader::<$loader_name>();
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
					reader: &'a mut bevy::asset::io::Reader,
					_settings: &'a Self::Settings,
					_load_context: &'a mut bevy::asset::LoadContext,
				) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
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

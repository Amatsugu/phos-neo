#[macro_export]
macro_rules! create_asset_loader {
	(
		$plugin_name: ident,
		$loader_name: ident,
		$asset_type: ident,
		$extensions: expr,
		$($string_name: ident -> $handle_name: ident)* ;
		$($string_array_name: ident -> $handle_array_name: ident)* ?
	) => {
		use bevy::prelude::*;
		use bevy::asset::{AssetLoader, AssetEvent, AssetEvents, LoadContext, LoadState, AsyncReadExt, io::Reader};
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

			async fn load(
		        &self,
				reader: & mut dyn bevy::asset::io::Reader,
				_: &Self::Settings,
				load_context: &mut LoadContext<'_>,
		    ) -> Result<Self::Asset, Self::Error> {
				let mut bytes = Vec::new();
				let read_result = reader.read_to_end(&mut bytes).await;
				if read_result.is_err() {
					return Err(read_result.err().unwrap().to_string());
				}
				let serialized: Result<Self::Asset, _> =
					ron::de::from_bytes::<Self::Asset>(&bytes);
				if serialized.is_err() {
					return Err(serialized.err().unwrap().to_string());
				}
				let mut asset = serialized.unwrap();
				$(

					asset.$handle_name = load_context.load(&asset.$string_name);
				)*
				$(
					for i in 0..asset.$string_array_name.len(){
						asset.$handle_array_name.push(load_context.load(&asset.$string_array_name[i]));
					}
				)?
				return Ok(asset);
			}

			fn extensions(&self) -> &[&str] {
				$extensions
			}
		}
	};
}

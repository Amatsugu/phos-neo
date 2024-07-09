pub mod macros {

	#[macro_export]
	macro_rules! create_asset_loader {
		(
			$plugin_name: ident,
			$loader_name: ident,
			$asset_type: ident,
			$asset_loadstate_name: ident,
			$extensions: expr,
			$($string_name: ident -> $handle_name: ident)* ;
			$($string_array_name: ident -> $handle_array_name: ident)* ?
		) => {
			use bevy::prelude::*;
			use bevy::asset::{AssetLoader, AssetEvent, AssetEvents, LoadContext, LoadState, AsyncReadExt, io::Reader};
			use bevy::utils::BoxedFuture;
			pub struct $plugin_name;
			impl Plugin for $plugin_name {
				fn build(&self, app: &mut App) {
					app.init_asset::<$asset_type>()
						.init_asset_loader::<$loader_name>()
						.insert_resource($asset_loadstate_name::default())
						.add_systems(Update, finalize);
				}
			}

			fn finalize(
				mut asset_events: EventReader<AssetEvent<$asset_type>>,
				mut assets: ResMut<Assets<$asset_type>>,
				mut load_state: ResMut<$asset_loadstate_name>,
				 asset_server: Res<AssetServer>
			) {
				for event in asset_events.read() {
					match event {
						AssetEvent::Added { id } => load_state.added += 1,
						AssetEvent::LoadedWithDependencies { id } => {
							let asset = assets.get_mut(id.clone()).unwrap();

							$(

								asset.$handle_name = asset_server.load(&asset.$string_name);
							)*
							$(
								for i in 0..asset.$string_array_name.len(){
									asset.$handle_array_name.push(asset_server.load(&asset.$string_array_name[i]));
								}
							)?
							load_state.loaded += 1;
						},
						_ => (),
					}
				}
			}

			#[derive(Resource, Debug, Default)]
			pub struct $asset_loadstate_name{
				pub loaded: u32,
				pub added: u32,
			}

			impl $asset_loadstate_name{
				pub fn is_all_loaded(&self) -> bool{
					if self.added == 0{
						return false;
					}
					return self.loaded >= self.added;
				}
			}

			#[derive(Default)]
			pub struct $loader_name;

			impl AssetLoader for $loader_name {
				type Asset = $asset_type;

				type Settings = ();

				type Error = String;

				async fn load<'a>(
					&'a self,
					reader: &'a mut Reader<'_>,
					_settings: &'a Self::Settings,
					_load_context: &'a mut LoadContext<'_>,
				) -> Result<Self::Asset, Self::Error> {
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
					let r = serialized.unwrap();
					return Ok(r);
				}

				fn extensions(&self) -> &[&str] {
					$extensions
				}
			}
		};
	}
}

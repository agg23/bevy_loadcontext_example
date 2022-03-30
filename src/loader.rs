use anyhow::Result;
use bevy::asset::{AssetLoader, AssetPath, LoadContext, LoadedAsset};
use bevy::prelude::shape::Cube;
use bevy::prelude::*;

#[derive(Default)]
pub struct CustomLoader;

impl AssetLoader for CustomLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<()>> {
        Box::pin(async move { Ok(load(bytes, load_context).await?) })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["test"];
        EXTENSIONS
    }
}

async fn load<'a, 'b>(_bytes: &'a [u8], load_context: &'a mut LoadContext<'b>) -> Result<()> {
    println!("Loading");
    let mut world = World::default();

    let count = 10;

    let mesh: Mesh = Mesh::from(Cube { size: 1.0 });
    load_context.set_labeled_asset("cube", LoadedAsset::new(mesh));

    for i in 0..count {
        for j in 0..count {
            let label = label(i, j);

            load_context.set_labeled_asset(
                &label,
                LoadedAsset::new(StandardMaterial {
                    base_color: Color::Hsla {
                        hue: (i as f32) / (count as f32) * 360.0,
                        saturation: j as f32 / (count as f32),
                        lightness: 0.5,
                        alpha: 1.0,
                    },
                    ..Default::default()
                }),
            );
        }
    }

    world
        .spawn()
        .insert_bundle((Transform::identity(), GlobalTransform::identity()))
        .with_children(|parent| {
            for i in 0..count {
                for j in 0..count {
                    let cube_asset_path = AssetPath::new_ref(load_context.path(), Some("cube"));

                    let material_label = label(i, j);
                    let material_asset_path =
                        AssetPath::new_ref(load_context.path(), Some(&material_label));
                    parent.spawn_bundle(PbrBundle {
                        mesh: load_context.get_handle(cube_asset_path),
                        material: load_context.get_handle(material_asset_path),
                        transform: Transform::from_translation(Vec3::new(i as f32, j as f32, 0.0)),
                        ..Default::default()
                    });
                }
            }
        });

    load_context.set_default_asset(LoadedAsset::new(Scene::new(world)));

    Ok(())
}

fn label(i: u32, j: u32) -> String {
    format!("mat{i}{j}")
}

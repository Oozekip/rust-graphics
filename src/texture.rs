use std::error::Error;

use gfx::handle::ShaderResourceView;
use gfx::Resources;
use gfx::traits::FactoryExt;
use gfx::format::Rgba8;
use gfx::texture as t;
use image;

pub type Texture<R> = ShaderResourceView<R, [f32; 4]>;

pub fn load_texture<R: Resources, F: FactoryExt<R>>(
    factory: &mut F,
    file_path: &str,
) -> Result<Texture<R>, String> {
    let open_res = image::open(file_path);

    match open_res {
        Ok(img_raw) => {
            let img = img_raw.to_rgba();
            let (width, height) = img.dimensions();

            let kind = t::Kind::D2(width as t::Size, height as t::Size, t::AaMode::Single);

            let result =
                factory.create_texture_immutable_u8::<Rgba8>(kind, &[&img]);

            match result {
                Ok((_, view)) => Ok(view),
                Err(err) => Err(String::from(err.description())),
            }
        }
        Err(err) => Err(String::from(err.description())),
    }
}

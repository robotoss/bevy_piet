// use wgpu::{Device, Queue};

// use std::{fs::File, io::BufWriter};
// use piet_wgsl::prelude::*;

// pub async fn do_render(
//     device: &Device,
//     queue: &Queue,
//     engine: &mut Engine,
// ) -> Result<(), Box<dyn std::error::Error>> {

//     #[allow(unused)]
//     let shaders = init_shaders(device, engine)?;
//     let full_shaders = full_shaders(device, engine)?;
//     let scene = gen_test_scene();
//     //test_scene::dump_scene_info(&scene);
//     //let (recording, buf) = render::render(&scene, &shaders);
//     let (recording, buf) = render_full(&scene, &full_shaders);
//     let downloads = engine.run_recording(&device, &queue, &recording)?;
//     let mapped = downloads.map();
//     device.poll(wgpu::Maintain::Wait);
//     let buf = mapped.get_mapped(buf).await?;

//     if false {
//         dump_buf(bytemuck::cast_slice(&buf));
//     } else {
//         let file = File::create("image.png")?;
//         let w = BufWriter::new(file);
//         let mut encoder = png::Encoder::new(w, 1024, 1024);
//         encoder.set_color(png::ColorType::Rgba);
//         let mut writer = encoder.write_header()?;
//         writer.write_image_data(&buf)?;
//     }
//     Ok(())
// }

// fn dump_buf(buf: &[u32]) {
//     for (i, val) in buf.iter().enumerate() {
//         if *val != 0 {
//             let lo = val & 0x7fff_ffff;
//             if lo >= 0x3000_0000 && lo < 0x5000_0000 {
//                 println!("{}: {:x} {}", i, val, f32::from_bits(*val));
//             } else {
//                 println!("{}: {:x}", i, val);
//             }
//         }
//     }
// }
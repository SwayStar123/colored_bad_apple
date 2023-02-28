use std::io::Cursor;
use base64::{Engine as _, engine::general_purpose};
use image::{self, ImageOutputFormat};
use serde::Deserialize;
use ureq::{self, json};

#[derive(Debug, Deserialize)]
struct Response {
    images: Vec<String>,
    // parameters: serde_json::Value,
    // info: String,
}

// gives the prompt, and whether or not to invert the image
fn get_info_by_frame(frame: u32) -> (String, bool) {
    match frame {
        0..=43 => panic!("These frames are all black"),
        44..=100 => ("1girl, closeup of Hakurei Reimu, red dress, white sleeves, red ribbon, brown hair, looking at the viewer, front facing".to_string(), true),
        101..=218 => ("1girl, Hakurei Reimu, red dress, white sleeves, red ribbon, brown hair, looking at the viewer, front facing".to_string(), true),
        219..=267 => ("1girl, Hakurei Reimu, red dress, white sleeves, red ribbon, brown hair, sideshot, (holding an apple),".to_string(), true),
        268..=322 => ("1girl, Hakurei Reimu, red dress, white sleeves, red ribbon, brown hair, sideshot, (eating an apple)".to_string(), true),
        323..=337 => ("1girl, Hakurei Reimu, red dress, white sleeves, red ribbon, brown hair, sideshot, (holding an apple)".to_string(), true),
        338..=355 => ("1girl, Hakurei Reimu, red dress, white sleeves, red ribbon, brown hair, sideshot".to_string(), true),
        356..=364 => ("1girl, Hakurei Reimu, red dress, white sleeves, brown hair, sideshot, throwing an apple".to_string(), true),
        365..=444 => ("apple".to_string(), true),
        445..=460 => ("1girl, out of frame, holding an apple, night sky, stars, clouds".to_string(), false),
        461..=471 => ("1girl, out of frame, riding a broom, night sky, stars, clouds".to_string(), false),
        472..=479 => ("1girl, Kirisame Marisa, holding an apple, riding a broom, witch, witch hat, maid uniform, black and white clothes, white ribbon around waist, night sky, stars, clouds".to_string(), false),
        480..=526 => ("1girl, Kirisame Marisa, (eating an apple), witch, witch hat, maid uniform, black and white clothes, white ribbon around waist, night sky, stars, clouds".to_string(), false),
        527..=659 => ("1girl, Kirisame Marisa, (eating an apple), riding a broom, witch, witch hat, maid uniform, black and white clothes, white ribbon around waist, night sky, stars, clouds".to_string(), false),
        660..=756 => ("1girl, Kirisame Marisa, riding a broom, witch, witch hat, maid uniform, black and white clothes, white ribbon around waist, night sky, stars, clouds, flying towards a church building".to_string(), false),
        757..=768 => ("1girl, Kirisame Marisa, holding an eaten apple core, riding a broom, witch, witch hat, maid uniform, black and white clothes, white ribbon around waist, night sky, stars, clouds".to_string(), false),
        769..=800 => ("a girls hand and an eaten apple core".to_string(), false),
        801..=821 => ("eaten apple core".to_string(), false),
        822..=853 => ("eaten apple core".to_string(), true),
        854..=1046 => ("1girl, Patchouli Knowledge, holding a book, dancing, striped pink dress, tied pink hair, pink hat, looking at the viewer, front facing".to_string(), true),
        1047..=1065 => ("1girl, Patchouli, out of frame, wiggling her finger".to_string(), true),
        1066..=1092 => ("femenine hand, index finger".to_string(), true),
        1093..=1098 => ("1girl, Remilia Scarlet, red wings, redpink dress, blue hair, looking at the viewer, front facing".to_string(), true),
        1099..=1168 => ("1girl, Remilia Scarlet, red wings, redpink dress, blue hair, holding a white cup near her chest, looking at the viewer, front facing".to_string(), true),
        1169..=1222 => ("1girl, Remilia Scarlet, red wings, redpink dress, blue hair, holding a white cup, looking at the viewer, front facing".to_string(), true),
        1223..=1263 => ("a white cup".to_string(), true),
        1264..=1298 => ("a white cup shattering".to_string(), false),
        1299..=1321 => ("a shard of white glass".to_string(), false),
        1322..=1445 => ("1girl, Izayoi Sakuya, dark blue maid uniform, silver hair, dancing".to_string(), false),
        1446..=1480 => ("1girl, Izayoi Sakuya, dark blue maid uniform, silver hair, throwing a knife".to_string(), false),
        1481..=1505 => ("a knife".to_string(), false),
        1506..=1657 => ("1girl, Flandre Scarlet, red dress, blond hair, looking at the viewer, front facing".to_string(), false),
        1658..=1686 => ("1girl, Flandre Scarlet, red dress, blond hair, grinning, looking at the viewer, front facing".to_string(), false),
        1687..=1742 => ("1girl, Flandre Scarlet red dress, blond hair, grinning, katana in the foreground, looking at the viewer, front facing".to_string(), true),
        1743..=1755 => ("katana blade".to_string(), false),
        1756..=1880 => ("2girls, Konpaku Youmu, green dress, multiple katanas, Saigyouji Yuyuko, purple dress, holding handfan".to_string(), false),
        1881..=1911 => ("sakura tree, pink leaves, leaves falling, 2girls beneath the tree".to_string(), false),
        1912..=2106 => ("1girl, Saigyouji Yuyuko, purple dress, holding handfan, sakura leaves falling in the background".to_string(), false),
        2107..=2134 => ("a single sakura petal/leaf".to_string(), false),
        2135..=2249 => ("1girl, Onozuka Komachi, red hair, blue and white dress, holding a large scythe, on a boat".to_string(), false),
        2250..=2322 => ("1girl, Onozuka Komachi, red hair, blue and white dress, holding a large scythe".to_string(), false),
        2323..=2332 => ("blade of a scythe".to_string(), false),
        2333..=2355 => ("half metal half black background".to_string(), false),
        2356..=2509 => ("1girl, Shiki Eiki Yamazanadu, green hair, purple dress, holding a wooden knife".to_string(), false), //TODO: This wont work as the character is both in black and white
        2510..=2535 => ("wooden blade".to_string(), false),
        2536..=2730 => ("1girl, Fujiwara no Mokou, pink hair, red dress, (fire)".to_string(), false),
        2731..=2782 => ("fire".to_string(), false),
        2783..=2833 => ("2girls, Kamishirasawa Keine, blue and green, fire".to_string(), false),
        2834..=2938 => ("2girls, Kamishirasawa Keine, blue and green, holding hands".to_string(), false),
        2939..=2969 => ("moon".to_string(), false),
        2970..=3133 => ("1girl, Yagokoro Eirin, red and dark blue dress, moon".to_string(), false),
        3134..=3157 => ("moon".to_string(), false),
        3158..=3211 => ("1girl, Houraisan Kaguya, kimono, red dress, pink shirt, black hair, moon".to_string(), false),
        3212..=3289 => ("1girl, Houraisan Kaguya, kimono, red dress, pink shirt, black hair".to_string(), false),
        3290..=3309 => ("1girl, wearing kimono, reaching out to the moon".to_string(), false),
        3310..=3352 => ("moon".to_string(), false),
        3353..=3588 => ("girls, Prismriver Sisters, playing instruments".to_string(), true),




        _ => panic!("No tags for frame {}", frame),
    }
}

fn pad_num(num: u32) -> String {
    let mut num_str = num.to_string();
    while num_str.len() < 4 {
        num_str = format!("0{}", num_str);
    }
    num_str
}

fn main() {
    let url = "http://127.0.0.1:7860";

    let first_image = image::open("frames/mmd/img2960.png").expect("Failed to open image");
    let mut buffer = Vec::new();
    first_image.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Png)
        .expect("Failed to write image to buffer");
    let mut last_image = general_purpose::STANDARD.encode(&buffer);

    for i in 2960..=3588 {
        let frame = pad_num(i);
        let (prompt, to_invert) = get_info_by_frame(i);

        let mut original_image = image::open(format!("frames/original/img{}.png", frame)).expect("Failed to open image");

        if to_invert {
            original_image.invert();
        }

        let mut buffer = Vec::new();
        original_image.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Png)
            .expect("Failed to write image to buffer");

        let base64_og = general_purpose::STANDARD.encode(&buffer);

        let payload = json!({
            "prompt": format!("{prompt}, best quality, beautifully detailed"),
            "negative_prompt": "ugly, nsfw, hentai, back, showing back, ((tree branch))",
            "steps": 20,
            "init_images": [last_image],
            "controlnet_module": "none",
            "controlnet_model": "control_sd15_depth [fef5e48e]",
            "controlnet_input_image": [base64_og],
            "controlnet_guidance": 1.0,
            "sampler_index": "DPM++ SDE Karras",
            "restore_faces": false,
            "width": 480,
            "height": 360,
            "controlnet_guessmode": false,
            "denoising_strength": 0.65,
            "cfg_scale": 7,
            "controlnet_weight": 1.0,
            "n_iter": 1,
            "inpaint_full_res": false,
        });

        let response = ureq::post(&format!("{}/controlnet/img2img", url))
            .send_json(payload)
            .expect("Failed to send request");

        let r: Response = response.into_json().expect("Failed to deserialize response");

        let i = &r.images[0];

        let image_bytes = general_purpose::STANDARD.decode(i.split(',').next().expect("Invalid image data"))
            .expect("Failed to decode image data");
        let image = image::load(Cursor::new(image_bytes), image::ImageFormat::Png)
            .expect("Failed to load image");

        let mut output = std::fs::File::create(format!("output/img{frame}.png"))
            .expect("Failed to create output file");

        image.write_to(&mut output, ImageOutputFormat::Png).unwrap();

        last_image = i.to_string();
    }
}

// fn main() {
//     let url = "http://127.0.0.1:7860";

//     // open output.png
//     let mut original_image = image::open("frames/original/img0100.png").expect("Failed to open image");
    
//     // invert image colors
//     original_image.invert();

//     // save image to output3.png
//     // let mut output = std::fs::File::create("output3.png")
//     //     .expect("Failed to create output file");

//     // original_image.write_to(&mut output, ImageOutputFormat::Png).unwrap();

    
//     let mut buffer = Vec::new();
//     original_image.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Png)
//         .expect("Failed to write image to buffer");
//     let base64_og = general_purpose::STANDARD.encode(&buffer);

//     let mmd_image = image::open("frames/mmd/img0100.png").expect("Failed to open image");
//     let mut buffer = Vec::new();
//     mmd_image.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Png)
//         .expect("Failed to write image to buffer");
//     let base64_mmd = general_purpose::STANDARD.encode(&buffer);

//     let payload = json!({
//         "prompt": "1girl, brown hair, red dress, red ribbon, white sleeves",
//         "negative_prompt": "ugly",
//         "steps": 20,
//         "init_images": [base64_mmd],
//         "controlnet_module": "none",
//         "controlnet_model": "control_sd15_depth [fef5e48e]",
//         "controlnet_input_image": [base64_og],
//         "controlnet_guidance": 1.0,
//         "sampler_index": "DPM++ SDE Karras",
//         "restore_faces": false,
//         "width": 768,
//         "height": 576,
//         "controlnet_guessmode": false,
//         "denoising_strength": 0.95,
//         "cfg_scale": 7,
//         "controlnet_weight": 1.0,
//         "n_iter": 1,
//         "inpaint_full_res": false,

//     });

//     let response = ureq::post(&format!("{}/controlnet/img2img", url))
//     // let response = ureq::post(&format!("{}/sdapi/v1/img2img", url))
//         .send_json(payload)
//         .expect("Failed to send request");

//     let r: Response = response.into_json().expect("Failed to deserialize response");

//     let i = &r.images[0];
//     // for i in r.images {
//         let image_bytes = general_purpose::STANDARD.decode(i.split(',').next().expect("Invalid image data"))
//             .expect("Failed to decode image data");
//         let image = image::load(Cursor::new(image_bytes), image::ImageFormat::Png)
//             .expect("Failed to load image");

//         let mut output = std::fs::File::create("output2.png")
//             .expect("Failed to create output file");

//         image.write_to(&mut output, ImageOutputFormat::Png).unwrap();
//     // }

// }

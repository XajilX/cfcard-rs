use std::collections::HashMap;
use image::{Rgba, RgbaImage, imageops};
use lazy_static::lazy_static;
use rusttype::{Font, Scale};
use imageproc::drawing;

lazy_static! {
    static ref RANK_COLOR: HashMap<&'static str, Rgba<u8>> = HashMap::from([
        ("unranked", Rgba([0, 0, 0, 255])),
        ("newbie", Rgba([0x80, 0x80, 0x80, 255])),
        ("pupil", Rgba([0, 0x80, 0, 255])),
        ("specialist", Rgba([0x03, 0xa8, 0x93, 255])),
        ("expert", Rgba([0, 0, 0xff, 255])),
        ("candidate master", Rgba([0xaa, 0, 0xaa, 255])),
        ("master", Rgba([0xff, 0x8c, 0, 255])),
        ("international master", Rgba([0xff, 0x66, 0, 255])),
        ("grandmaster", Rgba([0xff, 0, 0, 255])),
        ("international grandmaster", Rgba([0xcc, 0, 0, 255])),
        ("legendary grandmaster", Rgba([0x80, 0, 0, 255]))
    ]);
}

pub fn gene_clear(name: &str, rank: &str, rating: i64, avatar: &RgbaImage) -> RgbaImage {
    let mut ret = RgbaImage::from_pixel(518, 320, Rgba([255, 255, 255, 255]));
    let mut re_avatar = imageops::resize(avatar, 160, 160, imageops::FilterType::Triangle);
    let mut img_rank = RgbaImage::from_pixel(160, 160, RANK_COLOR[rank]);
    
    for x in 0..160 {
        for y in 0..160 {
            let (fx, fy) = (x as f32, y as f32);
            if  (fy/(fx - 79.5) > -0.5 && fy/(fx - 79.5)<0.5) ||
                ((fy - 159.)/(fx - 79.5) > -0.5 && (fy - 159.)/(fx - 79.5) < 0.5)
            {
                re_avatar.get_pixel_mut(x, y)[3] = 0;
                img_rank.get_pixel_mut(x, y)[3] = 0;
            } else {
                re_avatar.get_pixel_mut(x, y)[3] = 255;

                let (dx, dy) = (fx - 79., fy - 79.);
                let (da, db) = (dx - dy/1.732051, dy*2./1.732051);
                let dist = (da.abs() + db.abs() + (da + db).abs()) / 2.;
                let beta = dist / 79. * 0.4;
                img_rank.get_pixel_mut(x, y)[3] = ((1. - beta) * 255.) as u8;
            }
        }
    }

    let font_num = Font::try_from_bytes(
        include_bytes!("fonts/NovaMono-Regular.ttf") as &[u8])
        .expect("Error in loading font");
    let font_name = Font::try_from_bytes(
        include_bytes!("fonts/Dosis-SemiBold.ttf") as &[u8])
        .expect("Error in loading font");
    let font_rank = Font::try_from_bytes(
        include_bytes!("fonts/Dosis-Medium.ttf") as &[u8])
        .expect("Error in loading font");
    
    let mut h_name = 60;
    for i in (0..=60).rev() {
        let (tw, _) = drawing::text_size(Scale::uniform(i as f32), &font_name, name);
        if tw <= 285 {
            h_name = i;
            break;
        }
    }
    drawing::draw_text_mut(&mut ret,
        Rgba([0x45, 0x45, 0x45, 0xff]),
        220, 56 + (48 - h_name) / 2,
        Scale::uniform(h_name as f32),
        &font_name, name);

    let scale_rank = Scale::uniform(34.5);
    drawing::draw_text_mut(&mut ret,
        Rgba([0x45, 0x45, 0x45, 0xff]),
        220, 116 + (48 - h_name) / 2,
        scale_rank,
        &font_rank, rank);
    
    let str_rating = if rating >= 0 { rating.to_string() } else { "----".to_string() };
    let scale_rating = Scale::uniform(51.5);
    let (tw, th) = drawing::text_size(scale_rating, &font_num, &str_rating);
    drawing::draw_text_mut(&mut img_rank,
        Rgba([0xff, 0xff, 0xff, 0xff]),
        (160 - tw) / 2, (160 - th) / 2,
        scale_rating,
        &font_num, &str_rating);

    imageops::overlay(&mut ret, &mut re_avatar, 20, 20);
    imageops::overlay(&mut ret, &mut img_rank, 99, 139);
    ret
}

pub fn fail_clear(msg: &str) -> RgbaImage {
    let mut ret = RgbaImage::from_pixel(518, 320, Rgba([255, 255, 255, 255]));
    let font_msg = Font::try_from_bytes(
        include_bytes!("fonts/Dosis-SemiBold.ttf") as &[u8])
        .expect("Error in loading font");
    let scale_msg = Scale::uniform(60.);
    let (w, h) = drawing::text_size(scale_msg, &font_msg, msg);
    drawing::draw_text_mut(&mut ret,
        Rgba([0x80, 0x80, 0x80, 0xff]),
        (518 - w) / 2, (320 - h) / 2,
        scale_msg,
        &font_msg, msg);
    ret
}
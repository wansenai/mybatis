use crate::pb::*;
use anyhow::Result;
use bytes::Bytes;
use lazy_static::lazy_static;
use std::convert::TryFrom;
use super::{
    Engine,
    SpcTransform,
};
use image::{
    DynamicImage,
    ImageBuffer,
    ImageOutputFormat,
};
use photon_rs::{
    effects,
    filters,
    multiple,
    native::open_image_from_bytes,
    transform,
    PhotonImage,
};

lazy_static!{
    // 水印文件加载静态变量
    static ref WATERMARK: PhotonImage = {
        let data = include_bytes!("../image/rust-logo.png");
        let watermark = open_image_from_bytes(data).unwrap();
        transform::resize(&watermark, 64, 64, transform::SamplingFilter::Nearest)
    };
}

pub struct Photon(PhotonImage);

// Bytes转Photon结构
impl TryFrom<Bytes> for Photon {
    type Error = anyhow::Error;

    fn try_from(data: Bytes) -> Result<Self, Self::Error>{
        Ok(Self(open_image_from_bytes(&data)?))
    }
}

impl Engine for Photon {
    fn apply(&mut self, specs: &[Spec]){
        for spec in specs.iter() {
            match spec.data {
                Some(spec::Data::Crop(ref v))       => self.transform(v),
                Some(spec::Data::Contrast(ref v))   => self.transform(v),
                Some(spec::Data::Filter(ref v))     => self.transform(v),
                Some(spec::Data::Fliph(ref v))      => self.transform(v),
                Some(spec::Data::Flipv(ref v))      => self.transform(v),
                Some(spec::Data::Resize(ref v))     => self.transform(v),
                Some(spec::Data::Watermark(ref v))  => self.transform(v),
                // 对目前未知的spec 不做任何处理
                _ => {}
            }
        }
    }

    fn generate(self, format: ImageOutputFormat) -> Vec<u8> {
        image_to_buf(self.0, format)
    }
}

impl SpcTransform<&Crop> for Photon {
    fn transform(&mut self, op: &Crop) {
        let image = transform::crop(&mut self.0, op.x1, op.y1, op.x2, op.y2);
        self.0 = image;
    }
}

impl SpcTransform<&Contrast> for Photon {
    fn transform(&mut self, op: &Contrast) {
        effects::adjust_contrast(&mut self.0, op.contrast);
    }
}

impl SpcTransform<&Flipv> for Photon {
    fn transform(&mut self, _op: &Flipv) {
        transform::flipv(&mut self.0);
    }
}

impl SpcTransform<&Fliph> for Photon {
    fn transform(&mut self, _op: &Fliph) {
        transform::fliph(&mut self.0);
    }
}

impl SpcTransform<&Filter> for Photon {
    fn transform(&mut self, op: &Filter) {
        match filter::Filter::from_i32(op.filter) {
            Some(filter::Filter::Unspecified) => {}
            Some(f) => filters::filter(&mut self.0, f.to_str().unwrap()),
            _ => {}
        }
    }
}

impl SpcTransform<&Resize> for Photon {
    fn transform(&mut self, op: &Resize){
        let image = match resize::ResizeType::from_i32(op.rtype).unwrap() {
            resize::ResizeType::Normal    => transform::resize(&mut self.0, op.width, op.height, resize::SampleFilter::from_i32(op.filter).unwrap().into()),
            resize::ResizeType::SeamCarve => {
                transform::seam_carve(&mut self.0, op.width, op.height)
            }
        };
        self.0 = image;
    }
}

impl SpcTransform<&Watermark> for Photon {
    fn transform(&mut self, op: &Watermark) {
        multiple::watermark(&mut self.0, &WATERMARK, op.x, op.y);
    }
}

fn image_to_buf(image: PhotonImage, format: ImageOutputFormat) -> Vec<u8> {
    let raw_pixels = image.get_raw_pixels();
    let width = image.get_width();
    let height = image.get_height();

    let image_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dynimage = DynamicImage::ImageRgb8(image_buffer);

    let mut buffer = Vec::with_capacity(32768);
    dynimage.write_to(&mut buffer, format).unwrap();

    buffer
}

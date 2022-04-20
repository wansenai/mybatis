use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use photon_rs::transform ::SamplingFilter;
use prost::Message;
use std::convert::TryFrom;

mod abi;
pub use abi::*;

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self {specs}
    }
}

// 使用ImageSpec生成一个字符串
impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        let data = image_spec.encode_to_vec();
        encode_config(data, URL_SAFE_NO_PAD)
    }
}

// 使用ImageSpec 通过一个字符串创建
impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = decode_config(value, URL_SAFE_NO_PAD)?;
        Ok(ImageSpec::decode(&data[..])?)
    }
}

// 辅助函数 photon_rs 相对应的方法需要字符串
impl filter::Filter{
    pub fn to_str(&self) -> Option<&'static str> {
        match self {
            filter::Filter::Unspecified => None,
            filter::Filter::Oceanic     => Some("oceanic"),
            filter::Filter::Islands     => Some("islands"),
            filter::Filter::Marine      => Some("marine"),
        }
    }
}

// 用定义的SampleFilter 和 photon_rs 的 SampleFilter 之间转换
impl From<resize::SampleFilter> for SamplingFilter{
    fn from(v: resize::SampleFilter) -> Self {
        match v {
            resize::SampleFilter::Undefined     => SamplingFilter::Nearest,
            resize::SampleFilter::CatmullRom    => SamplingFilter::CatmullRom,
            resize::SampleFilter::Gaussian      => SamplingFilter::Gaussian,
            resize::SampleFilter::Nearest       => SamplingFilter::Nearest,
            resize::SampleFilter::Lanczos3      => SamplingFilter::Lanczos3,
            resize::SampleFilter::Triangle      => SamplingFilter::Triangle,
        }
    }
}

// 创建辅助函数，让创建一个spec过程简单一点
impl Spec{
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::SeamCarve as i32,
                filter: resize::SampleFilter::Undefined as i32,
            })),
        }
    }

    pub fn new_resize(width: u32, height: u32, filter: resize::SampleFilter) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::Normal as i32,
                filter: filter as i32,
            })),
        }
    }

    pub fn new_filter(filter: filter::Filter) -> Self {
        Self {
            data: Some(spec::Data::Filter(Filter {
                filter: filter as i32,
            })),
        }
    }

    pub fn new_watermark(x: u32, y: u32) -> Self {
        Self {
            data: Some(spec::Data::Watermark(Watermark {
                x,
                y,
            })),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::borrow::Borrow;
    use std::convert::TryInto;

    #[test]
    fn encode_spec_could_be_decode() {
        let spec1 = Spec::new_resize(600, 600, resize::SampleFilter::CatmullRom);
        let spec2 = Spec::new_filter(filter::Filter::Marine);
        let image_spec = ImageSpec::new(vec![spec1, spec2]);
        let s: String = image_spec.borrow().into();
        assert_eq!(image_spec, s.as_str().try_into().unwrap());
    }
}
use crate::pb::Spec;
use image::ImageOutputFormat;

mod photon;
pub use photon::Photon;

// Engine trait 后面可以添加更多的engine 主流程只需要替换engine
pub trait Engine {
    /**
     * 对engine 按照 specs进行有序处理
     */
    fn apply(&mut self, spec: &[Spec]);

    /**
     * 从engine 生成目标图片 这里的self不是引用self
     */
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

// 如果添加更多的spec 只需要实现这个
pub trait SpcTransform<T> {
    /**
     * 对图片使用op 做 transform
     */
    fn transform(&mut self, op: T);
}
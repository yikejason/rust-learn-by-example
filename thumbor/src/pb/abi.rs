/// 一个 ImageSpec 是一个有序的数组，服务器按照 spec 的顺序处理
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSpec {
    #[prost(message, repeated, tag = "1")]
    pub spec: ::prost::alloc::vec::Vec<Spec>,
}
/// 处理图片改变大小
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resize {
    #[prost(uint32, tag = "1")]
    pub width: u32,
    #[prost(uint32, tag = "2")]
    pub height: u32,
    #[prost(enumeration = "resize::ResizeType", tag = "3")]
    pub rtype: i32,
    #[prost(enumeration = "resize::SampFilter", tag = "4")]
    pub filter: i32,
}
/// Nested message and enum types in `Resize`.
pub mod resize {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ResizeType {
        Norml = 0,
        SeamCarve = 1,
    }
    impl ResizeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResizeType::Norml => "NORML",
                ResizeType::SeamCarve => "SEAM_CARVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NORML" => Some(Self::Norml),
                "SEAM_CARVE" => Some(Self::SeamCarve),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SampFilter {
        Undefined = 0,
        Nearest = 1,
        Triangle = 2,
        CatmullRom = 3,
        Gaussian = 4,
        Lanczos3 = 5,
    }
    impl SampFilter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SampFilter::Undefined => "UNDEFINED",
                SampFilter::Nearest => "NEAREST",
                SampFilter::Triangle => "TRIANGLE",
                SampFilter::CatmullRom => "CATMULL_ROM",
                SampFilter::Gaussian => "GAUSSIAN",
                SampFilter::Lanczos3 => "LANCZOS3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNDEFINED" => Some(Self::Undefined),
                "NEAREST" => Some(Self::Nearest),
                "TRIANGLE" => Some(Self::Triangle),
                "CATMULL_ROM" => Some(Self::CatmullRom),
                "GAUSSIAN" => Some(Self::Gaussian),
                "LANCZOS3" => Some(Self::Lanczos3),
                _ => None,
            }
        }
    }
}
/// 处理图片截取
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Crop {
    #[prost(uint32, tag = "1")]
    pub x1: u32,
    #[prost(uint32, tag = "2")]
    pub y1: u32,
    #[prost(uint32, tag = "3")]
    pub x2: u32,
    #[prost(uint32, tag = "4")]
    pub y2: u32,
}
/// 处理水平翻转
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fliph {}
/// 处理垂直翻转
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flipv {}
/// 处理对比度
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contrast {
    #[prost(float, tag = "1")]
    pub contrast: f32,
}
/// 处理滤镜
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    #[prost(enumeration = "filter::Filter", tag = "1")]
    pub filter: i32,
}
/// Nested message and enum types in `Filter`.
pub mod filter {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Filter {
        Unspecified = 0,
        Oceanic = 1,
        Islands = 2,
        Marine = 3,
    }
    impl Filter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Filter::Unspecified => "UNSPECIFIED",
                Filter::Oceanic => "OCEANIC",
                Filter::Islands => "ISLANDS",
                Filter::Marine => "MARINE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "OCEANIC" => Some(Self::Oceanic),
                "ISLANDS" => Some(Self::Islands),
                "MARINE" => Some(Self::Marine),
                _ => None,
            }
        }
    }
}
/// 处理水印
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Watermark {
    #[prost(uint32, tag = "1")]
    pub x: u32,
    #[prost(uint32, tag = "2")]
    pub y: u32,
}
/// 一个 spec 可以包含上述处理方式之一
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spec {
    #[prost(message, optional, tag = "1")]
    pub resize: ::core::option::Option<Resize>,
    #[prost(message, optional, tag = "2")]
    pub crop: ::core::option::Option<Crop>,
    #[prost(message, optional, tag = "3")]
    pub flipv: ::core::option::Option<Flipv>,
    #[prost(message, optional, tag = "4")]
    pub fliph: ::core::option::Option<Fliph>,
    #[prost(message, optional, tag = "5")]
    pub contrast: ::core::option::Option<Contrast>,
    #[prost(message, optional, tag = "6")]
    pub filter: ::core::option::Option<Filter>,
    #[prost(message, optional, tag = "7")]
    pub watermark: ::core::option::Option<Watermark>,
}

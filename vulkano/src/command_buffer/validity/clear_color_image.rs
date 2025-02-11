// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::error;
use std::fmt;

use crate::device::Device;
use crate::device::DeviceOwned;
use crate::image::ImageAccess;
use crate::VulkanObject;

/// Checks whether a clear color image command is valid.
///
/// # Panic
///
/// - Panics if the destination was not created with `device`.
///
pub fn check_clear_color_image<I>(
    device: &Device,
    image: &I,
    base_array_layer: u32,
    layer_count: u32,
    base_mip_level: u32,
    level_count: u32,
) -> Result<(), CheckClearColorImageError>
where
    I: ?Sized + ImageAccess,
{
    assert_eq!(
        image.inner().image.device().internal_object(),
        device.internal_object()
    );

    if !image.inner().image.usage().transfer_destination {
        return Err(CheckClearColorImageError::MissingTransferUsage);
    }

    if base_array_layer + layer_count > image.dimensions().array_layers() {
        return Err(CheckClearColorImageError::OutOfRange);
    }

    if base_mip_level + level_count > image.mip_levels() {
        return Err(CheckClearColorImageError::OutOfRange);
    }

    Ok(())
}

/// Error that can happen from `check_clear_color_image`.
#[derive(Debug, Copy, Clone)]
pub enum CheckClearColorImageError {
    /// The image is missing the transfer destination usage.
    MissingTransferUsage,
    /// The array layers and mipmap levels are out of range.
    OutOfRange,
}

impl error::Error for CheckClearColorImageError {}

impl fmt::Display for CheckClearColorImageError {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt,
            "{}",
            match *self {
                CheckClearColorImageError::MissingTransferUsage => {
                    "the image is missing the transfer destination usage"
                }
                CheckClearColorImageError::OutOfRange => {
                    "the array layers and mipmap levels are out of range"
                }
            }
        )
    }
}

use file_format::FileFormat;

#[test]
fn test_adobe_photoshop_document() {
    let format = FileFormat::from_file("fixtures/image/sample.psd").unwrap();
    assert_eq!(format, FileFormat::AdobePhotoshopDocument);
}

#[test]
fn test_animated_portable_network_graphics() {
    let format = FileFormat::from_file("fixtures/image/sample.apng").unwrap();
    assert_eq!(format, FileFormat::AnimatedPortableNetworkGraphics);
}

#[test]
fn test_apple_icon_image() {
    let format = FileFormat::from_file("fixtures/image/sample.icns").unwrap();
    assert_eq!(format, FileFormat::AppleIconImage);
}

#[test]
fn test_autocad_drawing() {
    let format = FileFormat::from_file("fixtures/image/sample.dwg").unwrap();
    assert_eq!(format, FileFormat::AutocadDrawing);
}

#[test]
fn test_av1_image_file_format() {
    let format = FileFormat::from_file("fixtures/image/sample.avif").unwrap();
    assert_eq!(format, FileFormat::Av1ImageFileFormat);
}

#[test]
fn test_av1_image_file_format_sequence() {
    let format = FileFormat::from_file("fixtures/image/sample.avifs").unwrap();
    assert_eq!(format, FileFormat::Av1ImageFileFormatSequence);
}

#[test]
fn test_better_portable_graphics() {
    let format = FileFormat::from_file("fixtures/image/sample.bpg").unwrap();
    assert_eq!(format, FileFormat::BetterPortableGraphics);
}

#[test]
fn test_canon_raw2() {
    let format = FileFormat::from_file("fixtures/image/sample.cr2").unwrap();
    assert_eq!(format, FileFormat::CanonRaw2);
}

#[test]
fn test_canon_raw3() {
    let format = FileFormat::from_file("fixtures/image/sample.cr3").unwrap();
    assert_eq!(format, FileFormat::CanonRaw3);
}

#[test]
fn test_cineon() {
    let format = FileFormat::from_file("fixtures/image/sample.cin").unwrap();
    assert_eq!(format, FileFormat::Cineon);
}

#[test]
fn test_digital_picture_exchange() {
    let format = FileFormat::from_file("fixtures/image/sample.dpx").unwrap();
    assert_eq!(format, FileFormat::DigitalPictureExchange);
}

#[test]
fn test_djvu() {
    let format = FileFormat::from_file("fixtures/image/sample.djvu").unwrap();
    assert_eq!(format, FileFormat::Djvu);
}

#[test]
fn test_experimental_computing_facility() {
    let format = FileFormat::from_file("fixtures/image/sample.xcf").unwrap();
    assert_eq!(format, FileFormat::ExperimentalComputingFacility);
}

#[test]
fn test_flexible_image_transport_system() {
    let format = FileFormat::from_file("fixtures/image/sample.fits").unwrap();
    assert_eq!(format, FileFormat::FlexibleImageTransportSystem);
}

#[test]
fn test_free_lossless_image_format() {
    let format = FileFormat::from_file("fixtures/image/sample.flif").unwrap();
    assert_eq!(format, FileFormat::FreeLosslessImageFormat);
}

#[test]
fn test_fujifilm_raw() {
    let format = FileFormat::from_file("fixtures/image/sample.raf").unwrap();
    assert_eq!(format, FileFormat::FujifilmRaw);
}

#[test]
fn test_graphics_interchange_format() {
    let format = FileFormat::from_file("fixtures/image/sample.gif").unwrap();
    assert_eq!(format, FileFormat::GraphicsInterchangeFormat);
}

#[test]
fn test_high_efficiency_image_coding() {
    let format = FileFormat::from_file("fixtures/image/sample.heic").unwrap();
    assert_eq!(format, FileFormat::HighEfficiencyImageCoding);
}

#[test]
fn test_high_efficiency_image_coding_sequence() {
    let format = FileFormat::from_file("fixtures/image/sample.heics").unwrap();
    assert_eq!(format, FileFormat::HighEfficiencyImageCodingSequence);
}
#[test]
fn test_high_efficiency_image_file_format() {
    let format = FileFormat::from_file("fixtures/image/sample.heif").unwrap();
    assert_eq!(format, FileFormat::HighEfficiencyImageFileFormat);
}

#[test]
fn test_high_efficiency_image_file_format_sequence() {
    let format = FileFormat::from_file("fixtures/image/sample.heifs").unwrap();
    assert_eq!(format, FileFormat::HighEfficiencyImageFileFormatSequence);
}

#[test]
fn test_joint_photographic_experts_group() {
    let format = FileFormat::from_file("fixtures/image/sample.jpg").unwrap();
    assert_eq!(format, FileFormat::JointPhotographicExpertsGroup);
}

#[test]
fn test_jpeg2000_part1() {
    let format = FileFormat::from_file("fixtures/image/sample.jp2").unwrap();
    assert_eq!(format, FileFormat::Jpeg2000Part1);
}

#[test]
fn test_jpeg2000_part2() {
    let format = FileFormat::from_file("fixtures/image/sample.jpx").unwrap();
    assert_eq!(format, FileFormat::Jpeg2000Part2);
}

#[test]
fn test_jpeg2000_part3() {
    let format = FileFormat::from_file("fixtures/image/sample.mj2").unwrap();
    assert_eq!(format, FileFormat::Jpeg2000Part3);
}

#[test]
fn test_jpeg2000_part6() {
    let format = FileFormat::from_file("fixtures/image/sample.jpm").unwrap();
    assert_eq!(format, FileFormat::Jpeg2000Part6);
}

#[test]
fn test_jpeg_extended_range() {
    let format = FileFormat::from_file("fixtures/image/sample.jxr").unwrap();
    assert_eq!(format, FileFormat::JpegExtendedRange);
}

#[test]
fn test_jpeg_ls() {
    let format = FileFormat::from_file("fixtures/image/sample.jls").unwrap();
    assert_eq!(format, FileFormat::JpegLs);
}

#[test]
fn test_jpeg_xl() {
    let format = FileFormat::from_file("fixtures/image/sample.jxl").unwrap();
    assert_eq!(format, FileFormat::JpegXl);
}

#[test]
fn test_khronos_texture() {
    let format = FileFormat::from_file("fixtures/image/sample.ktx").unwrap();
    assert_eq!(format, FileFormat::KhronosTexture);
}

#[test]
fn test_khronos_texture2() {
    let format = FileFormat::from_file("fixtures/image/sample.ktx2").unwrap();
    assert_eq!(format, FileFormat::KhronosTexture2);
}

#[test]
fn test_microsoft_direct_draw_surface() {
    let format = FileFormat::from_file("fixtures/image/sample.dds").unwrap();
    assert_eq!(format, FileFormat::MicrosoftDirectDrawSurface);
}

#[test]
fn test_nikon_electronic_file() {
    let format = FileFormat::from_file("fixtures/image/sample.nef").unwrap();
    assert_eq!(format, FileFormat::NikonElectronicFile);
}

#[test]
fn test_olympus_raw_format() {
    let format = FileFormat::from_file("fixtures/image/sample.orf").unwrap();
    assert_eq!(format, FileFormat::OlympusRawFormat);
}

#[test]
fn test_openexr() {
    let format = FileFormat::from_file("fixtures/image/sample.exr").unwrap();
    assert_eq!(format, FileFormat::Openexr);
}

#[cfg(feature = "reader-zip")]
#[test]
fn test_openraster() {
    let format = FileFormat::from_file("fixtures/image/sample.ora").unwrap();
    assert_eq!(format, FileFormat::Openraster);
}

#[test]
fn test_panasonic_raw() {
    let format = FileFormat::from_file("fixtures/image/sample.rw2").unwrap();
    assert_eq!(format, FileFormat::PanasonicRaw);
}

#[test]
fn test_portable_network_graphics() {
    let format = FileFormat::from_file("fixtures/image/sample.png").unwrap();
    assert_eq!(format, FileFormat::PortableNetworkGraphics);
}

#[test]
fn test_radiance_hdr() {
    let format = FileFormat::from_file("fixtures/image/sample.hdr").unwrap();
    assert_eq!(format, FileFormat::RadianceHdr);
}

#[cfg(feature = "reader-xml")]
#[test]
fn test_scalable_vector_graphics() {
    let format = FileFormat::from_file("fixtures/image/sample.svg").unwrap();
    assert_eq!(format, FileFormat::ScalableVectorGraphics);
}

#[test]
fn test_tag_image_file_format() {
    let format = FileFormat::from_file("fixtures/image/sample.tiff").unwrap();
    assert_eq!(format, FileFormat::TagImageFileFormat);
}

#[test]
fn test_webp() {
    let format = FileFormat::from_file("fixtures/image/sample.webp").unwrap();
    assert_eq!(format, FileFormat::Webp);
}

#[test]
fn test_windows_bitmap() {
    let format = FileFormat::from_file("fixtures/image/sample.bmp").unwrap();
    assert_eq!(format, FileFormat::WindowsBitmap);
}

#[test]
fn test_windows_cursor() {
    let format = FileFormat::from_file("fixtures/image/sample.cur").unwrap();
    assert_eq!(format, FileFormat::WindowsCursor);
}

#[test]
fn test_windows_icon() {
    let format = FileFormat::from_file("fixtures/image/sample.ico").unwrap();
    assert_eq!(format, FileFormat::WindowsIcon);
}

#[test]
fn test_windows_metafile() {
    let format = FileFormat::from_file("fixtures/image/sample.wmf").unwrap();
    assert_eq!(format, FileFormat::WindowsMetafile);
}

//! Common video configurations used in vga programming.

use super::registers::{
    AttributeControllerIndex, CrtcControllerIndex, GraphicsControllerIndex, SequencerIndex,
};

/// Represents a set of vga registers for a given mode.
#[derive(Debug)]
pub struct VgaConfiguration {
    /// Represents the configuration value for the miscellaneous output register.
    pub miscellaneous_output: u8,
    /// Represents the configuration values for the sequencer registers.
    pub sequencer_registers: &'static [(SequencerIndex, u8)],
    /// Represents the configuration values for the crtc controller registers.
    pub crtc_controller_registers: &'static [(CrtcControllerIndex, u8)],
    /// Represents the configuration values for the graphics controller registers.
    pub graphics_controller_registers: &'static [(GraphicsControllerIndex, u8)],
    /// Represents the configuration values for the attribute controller registers.
    pub attribute_controller_registers: &'static [(AttributeControllerIndex, u8)],
}

/// Register values for Vga mode 40x25 Text.
pub const MODE_40X25_CONFIGURATION: VgaConfiguration = VgaConfiguration {
    // Configuration values acquired from https://www.singlix.com/trdos/archive/vga/Graphics%20in%20pmode.pdf
    miscellaneous_output: 0x67,
    sequencer_registers: &[
        (SequencerIndex::SequencerReset, 0x03),
        (SequencerIndex::ClockingMode, 0x08),
        (SequencerIndex::PlaneMask, 0x03),
        (SequencerIndex::CharacterFont, 0x00),
        (SequencerIndex::MemoryMode, 0x02),
    ],
    crtc_controller_registers: &[
        (CrtcControllerIndex::HorizontalTotal, 0x2D),
        (CrtcControllerIndex::HorizontalDisplayEnableEnd, 0x27),
        (CrtcControllerIndex::HorizontalBlankingStart, 0x28),
        (CrtcControllerIndex::HorizontalBlankingEnd, 0x90),
        (CrtcControllerIndex::HorizontalSyncStart, 0x2B),
        (CrtcControllerIndex::HorizontalSyncEnd, 0xA0),
        (CrtcControllerIndex::VeritcalTotal, 0xBF),
        (CrtcControllerIndex::Overflow, 0x1F),
        (CrtcControllerIndex::PresetRowScan, 0x00),
        (CrtcControllerIndex::MaximumScanLine, 0x4F),
        (CrtcControllerIndex::TextCursorStart, 0x0D),
        (CrtcControllerIndex::TextCursorEnd, 0x0E),
        (CrtcControllerIndex::StartAddressHigh, 0x00),
        (CrtcControllerIndex::StartAddressLow, 0x00),
        (CrtcControllerIndex::TextCursorLocationHigh, 0x00),
        (CrtcControllerIndex::TextCursorLocationLow, 0xA0),
        (CrtcControllerIndex::VerticalSyncStart, 0x9C),
        (CrtcControllerIndex::VerticalSyncEnd, 0x8E),
        (CrtcControllerIndex::VerticalDisplayEnableEnd, 0x8F),
        (CrtcControllerIndex::Offset, 0x14),
        (CrtcControllerIndex::UnderlineLocation, 0x1F),
        (CrtcControllerIndex::VerticalBlankingStart, 0x96),
        (CrtcControllerIndex::VerticalBlankingEnd, 0xB9),
        (CrtcControllerIndex::ModeControl, 0xA3),
        (CrtcControllerIndex::LineCompare, 0xFF),
    ],
    graphics_controller_registers: &[
        (GraphicsControllerIndex::SetReset, 0x00),
        (GraphicsControllerIndex::EnableSetReset, 0x00),
        (GraphicsControllerIndex::ColorCompare, 0x00),
        (GraphicsControllerIndex::DataRotate, 0x00),
        (GraphicsControllerIndex::ReadPlaneSelect, 0x00),
        (GraphicsControllerIndex::GraphicsMode, 0x10),
        (GraphicsControllerIndex::Miscellaneous, 0x0E),
        (GraphicsControllerIndex::ColorDontCare, 0x00),
        (GraphicsControllerIndex::BitMask, 0xFF),
    ],
    attribute_controller_registers: &[
        (AttributeControllerIndex::PaletteRegister0, 0x00),
        (AttributeControllerIndex::PaletteRegister1, 0x01),
        (AttributeControllerIndex::PaletteRegister2, 0x02),
        (AttributeControllerIndex::PaletteRegister3, 0x03),
        (AttributeControllerIndex::PaletteRegister4, 0x04),
        (AttributeControllerIndex::PaletteRegister5, 0x05),
        (AttributeControllerIndex::PaletteRegister6, 0x14),
        (AttributeControllerIndex::PaletteRegister7, 0x07),
        (AttributeControllerIndex::PaletteRegister8, 0x38),
        (AttributeControllerIndex::PaletteRegister9, 0x39),
        (AttributeControllerIndex::PaletteRegisterA, 0x3A),
        (AttributeControllerIndex::PaletteRegisterB, 0x3B),
        (AttributeControllerIndex::PaletteRegisterC, 0x3C),
        (AttributeControllerIndex::PaletteRegisterD, 0x3D),
        (AttributeControllerIndex::PaletteRegisterE, 0x3E),
        (AttributeControllerIndex::PaletteRegisterF, 0x3F),
        (AttributeControllerIndex::ModeControl, 0x0C),
        (AttributeControllerIndex::OverscanColor, 0x00),
        (AttributeControllerIndex::MemoryPlaneEnable, 0x0F),
        (AttributeControllerIndex::HorizontalPixelPanning, 0x08),
        (AttributeControllerIndex::ColorSelect, 0x00),
    ],
};

/// Register values for Vga mode 40x50 Text.
pub const MODE_40X50_CONFIGURATION: VgaConfiguration = VgaConfiguration {
    // Configuration values acquired from https://www.singlix.com/trdos/archive/vga/Graphics%20in%20pmode.pdf
    miscellaneous_output: 0x67,
    sequencer_registers: &[
        (SequencerIndex::SequencerReset, 0x03),
        (SequencerIndex::ClockingMode, 0x08),
        (SequencerIndex::PlaneMask, 0x03),
        (SequencerIndex::CharacterFont, 0x00),
        (SequencerIndex::MemoryMode, 0x02),
    ],
    crtc_controller_registers: &[
        (CrtcControllerIndex::HorizontalTotal, 0x2D),
        (CrtcControllerIndex::HorizontalDisplayEnableEnd, 0x27),
        (CrtcControllerIndex::HorizontalBlankingStart, 0x28),
        (CrtcControllerIndex::HorizontalBlankingEnd, 0x90),
        (CrtcControllerIndex::HorizontalSyncStart, 0x2B),
        (CrtcControllerIndex::HorizontalSyncEnd, 0xA0),
        (CrtcControllerIndex::VeritcalTotal, 0xBF),
        (CrtcControllerIndex::Overflow, 0x1F),
        (CrtcControllerIndex::PresetRowScan, 0x00),
        (CrtcControllerIndex::MaximumScanLine, 0x47),
        (CrtcControllerIndex::TextCursorStart, 0x06),
        (CrtcControllerIndex::TextCursorEnd, 0x07),
        (CrtcControllerIndex::StartAddressHigh, 0x00),
        (CrtcControllerIndex::StartAddressLow, 0x00),
        (CrtcControllerIndex::TextCursorLocationHigh, 0x04),
        (CrtcControllerIndex::TextCursorLocationLow, 0x60),
        (CrtcControllerIndex::VerticalSyncStart, 0x9C),
        (CrtcControllerIndex::VerticalSyncEnd, 0x8E),
        (CrtcControllerIndex::VerticalDisplayEnableEnd, 0x8F),
        (CrtcControllerIndex::Offset, 0x14),
        (CrtcControllerIndex::UnderlineLocation, 0x1F),
        (CrtcControllerIndex::VerticalBlankingStart, 0x96),
        (CrtcControllerIndex::VerticalBlankingEnd, 0xB9),
        (CrtcControllerIndex::ModeControl, 0xA3),
        (CrtcControllerIndex::LineCompare, 0xFF),
    ],
    graphics_controller_registers: &[
        (GraphicsControllerIndex::SetReset, 0x00),
        (GraphicsControllerIndex::EnableSetReset, 0x00),
        (GraphicsControllerIndex::ColorCompare, 0x00),
        (GraphicsControllerIndex::DataRotate, 0x00),
        (GraphicsControllerIndex::ReadPlaneSelect, 0x00),
        (GraphicsControllerIndex::GraphicsMode, 0x10),
        (GraphicsControllerIndex::Miscellaneous, 0x0E),
        (GraphicsControllerIndex::ColorDontCare, 0x00),
        (GraphicsControllerIndex::BitMask, 0xFF),
    ],
    attribute_controller_registers: &[
        (AttributeControllerIndex::PaletteRegister0, 0x00),
        (AttributeControllerIndex::PaletteRegister1, 0x01),
        (AttributeControllerIndex::PaletteRegister2, 0x02),
        (AttributeControllerIndex::PaletteRegister3, 0x03),
        (AttributeControllerIndex::PaletteRegister4, 0x04),
        (AttributeControllerIndex::PaletteRegister5, 0x05),
        (AttributeControllerIndex::PaletteRegister6, 0x14),
        (AttributeControllerIndex::PaletteRegister7, 0x07),
        (AttributeControllerIndex::PaletteRegister8, 0x38),
        (AttributeControllerIndex::PaletteRegister9, 0x39),
        (AttributeControllerIndex::PaletteRegisterA, 0x3A),
        (AttributeControllerIndex::PaletteRegisterB, 0x3B),
        (AttributeControllerIndex::PaletteRegisterC, 0x3C),
        (AttributeControllerIndex::PaletteRegisterD, 0x3D),
        (AttributeControllerIndex::PaletteRegisterE, 0x3E),
        (AttributeControllerIndex::PaletteRegisterF, 0x3F),
        (AttributeControllerIndex::ModeControl, 0x0C),
        (AttributeControllerIndex::OverscanColor, 0x00),
        (AttributeControllerIndex::MemoryPlaneEnable, 0x0F),
        (AttributeControllerIndex::HorizontalPixelPanning, 0x08),
        (AttributeControllerIndex::ColorSelect, 0x00),
    ],
};

/// Register values for Vga mode 80x25 Text.
pub const MODE_80X25_CONFIGURATION: VgaConfiguration = VgaConfiguration {
    // Configuration values acquired from https://www.singlix.com/trdos/archive/vga/Graphics%20in%20pmode.pdf
    miscellaneous_output: 0x67,
    sequencer_registers: &[
        (SequencerIndex::SequencerReset, 0x03),
        (SequencerIndex::ClockingMode, 0x00),
        (SequencerIndex::PlaneMask, 0x03),
        (SequencerIndex::CharacterFont, 0x00),
        (SequencerIndex::MemoryMode, 0x02),
    ],
    crtc_controller_registers: &[
        (CrtcControllerIndex::HorizontalTotal, 0x5F),
        (CrtcControllerIndex::HorizontalDisplayEnableEnd, 0x4F),
        (CrtcControllerIndex::HorizontalBlankingStart, 0x50),
        (CrtcControllerIndex::HorizontalBlankingEnd, 0x82),
        (CrtcControllerIndex::HorizontalSyncStart, 0x55),
        (CrtcControllerIndex::HorizontalSyncEnd, 0x81),
        (CrtcControllerIndex::VeritcalTotal, 0xBF),
        (CrtcControllerIndex::Overflow, 0x1F),
        (CrtcControllerIndex::PresetRowScan, 0x00),
        (CrtcControllerIndex::MaximumScanLine, 0x4F),
        (CrtcControllerIndex::TextCursorStart, 0x0D),
        (CrtcControllerIndex::TextCursorEnd, 0x0E),
        (CrtcControllerIndex::StartAddressHigh, 0x00),
        (CrtcControllerIndex::StartAddressLow, 0x00),
        (CrtcControllerIndex::TextCursorLocationHigh, 0x00),
        (CrtcControllerIndex::TextCursorLocationLow, 0x50),
        (CrtcControllerIndex::VerticalSyncStart, 0x9C),
        (CrtcControllerIndex::VerticalSyncEnd, 0x0E),
        (CrtcControllerIndex::VerticalDisplayEnableEnd, 0x8F),
        (CrtcControllerIndex::Offset, 0x28),
        (CrtcControllerIndex::UnderlineLocation, 0x1F),
        (CrtcControllerIndex::VerticalBlankingStart, 0x96),
        (CrtcControllerIndex::VerticalBlankingEnd, 0xB9),
        (CrtcControllerIndex::ModeControl, 0xA3),
        (CrtcControllerIndex::LineCompare, 0xFF),
    ],
    graphics_controller_registers: &[
        (GraphicsControllerIndex::SetReset, 0x00),
        (GraphicsControllerIndex::EnableSetReset, 0x00),
        (GraphicsControllerIndex::ColorCompare, 0x00),
        (GraphicsControllerIndex::DataRotate, 0x00),
        (GraphicsControllerIndex::ReadPlaneSelect, 0x00),
        (GraphicsControllerIndex::GraphicsMode, 0x10),
        (GraphicsControllerIndex::Miscellaneous, 0x0E),
        (GraphicsControllerIndex::ColorDontCare, 0x00),
        (GraphicsControllerIndex::BitMask, 0xFF),
    ],
    attribute_controller_registers: &[
        (AttributeControllerIndex::PaletteRegister0, 0x00),
        (AttributeControllerIndex::PaletteRegister1, 0x01),
        (AttributeControllerIndex::PaletteRegister2, 0x02),
        (AttributeControllerIndex::PaletteRegister3, 0x03),
        (AttributeControllerIndex::PaletteRegister4, 0x04),
        (AttributeControllerIndex::PaletteRegister5, 0x05),
        (AttributeControllerIndex::PaletteRegister6, 0x14),
        (AttributeControllerIndex::PaletteRegister7, 0x07),
        (AttributeControllerIndex::PaletteRegister8, 0x38),
        (AttributeControllerIndex::PaletteRegister9, 0x39),
        (AttributeControllerIndex::PaletteRegisterA, 0x3A),
        (AttributeControllerIndex::PaletteRegisterB, 0x3B),
        (AttributeControllerIndex::PaletteRegisterC, 0x3C),
        (AttributeControllerIndex::PaletteRegisterD, 0x3D),
        (AttributeControllerIndex::PaletteRegisterE, 0x3E),
        (AttributeControllerIndex::PaletteRegisterF, 0x3F),
        (AttributeControllerIndex::ModeControl, 0x0C),
        (AttributeControllerIndex::OverscanColor, 0x00),
        (AttributeControllerIndex::MemoryPlaneEnable, 0x0F),
        (AttributeControllerIndex::HorizontalPixelPanning, 0x08),
        (AttributeControllerIndex::ColorSelect, 0x00),
    ],
};

/// Register values for Vga mode 640x480x16 Graphics.
pub const MODE_640X480X16_CONFIGURATION: VgaConfiguration = VgaConfiguration {
    // Configuration values acquired from https://www.singlix.com/trdos/archive/vga/Graphics%20in%20pmode.pdf
    // and https://forum.osdev.org/viewtopic.php?f=1&t=20137&hilit=640x480x16
    miscellaneous_output: 0xE3,
    sequencer_registers: &[
        (SequencerIndex::SequencerReset, 0x03),
        (SequencerIndex::ClockingMode, 0x01),
        (SequencerIndex::PlaneMask, 0x08),
        (SequencerIndex::CharacterFont, 0x00),
        (SequencerIndex::MemoryMode, 0x06),
    ],
    crtc_controller_registers: &[
        (CrtcControllerIndex::HorizontalTotal, 0x5F),
        (CrtcControllerIndex::HorizontalDisplayEnableEnd, 0x4F),
        (CrtcControllerIndex::HorizontalBlankingStart, 0x50),
        (CrtcControllerIndex::HorizontalBlankingEnd, 0x82),
        (CrtcControllerIndex::HorizontalSyncStart, 0x54),
        (CrtcControllerIndex::HorizontalSyncEnd, 0x80),
        (CrtcControllerIndex::VeritcalTotal, 0x0B),
        (CrtcControllerIndex::Overflow, 0x3E),
        (CrtcControllerIndex::PresetRowScan, 0x00),
        (CrtcControllerIndex::MaximumScanLine, 0x40),
        (CrtcControllerIndex::TextCursorStart, 0x00),
        (CrtcControllerIndex::TextCursorEnd, 0x00),
        (CrtcControllerIndex::StartAddressHigh, 0x00),
        (CrtcControllerIndex::StartAddressLow, 0x00),
        (CrtcControllerIndex::TextCursorLocationHigh, 0x00),
        (CrtcControllerIndex::TextCursorLocationLow, 0x00),
        (CrtcControllerIndex::VerticalSyncStart, 0xEA),
        (CrtcControllerIndex::VerticalSyncEnd, 0x0C),
        (CrtcControllerIndex::VerticalDisplayEnableEnd, 0xDF),
        (CrtcControllerIndex::Offset, 0x28),
        (CrtcControllerIndex::UnderlineLocation, 0x00),
        (CrtcControllerIndex::VerticalBlankingStart, 0xE7),
        (CrtcControllerIndex::VerticalBlankingEnd, 0x04),
        (CrtcControllerIndex::ModeControl, 0xE3),
        (CrtcControllerIndex::LineCompare, 0xFF),
    ],
    graphics_controller_registers: &[
        (GraphicsControllerIndex::SetReset, 0x00),
        (GraphicsControllerIndex::EnableSetReset, 0x00),
        (GraphicsControllerIndex::ColorCompare, 0x00),
        (GraphicsControllerIndex::DataRotate, 0x00),
        (GraphicsControllerIndex::ReadPlaneSelect, 0x03),
        (GraphicsControllerIndex::GraphicsMode, 0x00),
        (GraphicsControllerIndex::Miscellaneous, 0x05),
        (GraphicsControllerIndex::ColorDontCare, 0x0F),
        (GraphicsControllerIndex::BitMask, 0xFF),
    ],
    attribute_controller_registers: &[
        (AttributeControllerIndex::PaletteRegister0, 0x00),
        (AttributeControllerIndex::PaletteRegister1, 0x01),
        (AttributeControllerIndex::PaletteRegister2, 0x02),
        (AttributeControllerIndex::PaletteRegister3, 0x03),
        (AttributeControllerIndex::PaletteRegister4, 0x04),
        (AttributeControllerIndex::PaletteRegister5, 0x05),
        (AttributeControllerIndex::PaletteRegister6, 0x14),
        (AttributeControllerIndex::PaletteRegister7, 0x07),
        (AttributeControllerIndex::PaletteRegister8, 0x38),
        (AttributeControllerIndex::PaletteRegister9, 0x39),
        (AttributeControllerIndex::PaletteRegisterA, 0x3A),
        (AttributeControllerIndex::PaletteRegisterB, 0x3B),
        (AttributeControllerIndex::PaletteRegisterC, 0x3C),
        (AttributeControllerIndex::PaletteRegisterD, 0x3D),
        (AttributeControllerIndex::PaletteRegisterE, 0x3E),
        (AttributeControllerIndex::PaletteRegisterF, 0x3F),
        (AttributeControllerIndex::ModeControl, 0x01),
        (AttributeControllerIndex::OverscanColor, 0x00),
        (AttributeControllerIndex::MemoryPlaneEnable, 0x0F),
        (AttributeControllerIndex::HorizontalPixelPanning, 0x00),
        (AttributeControllerIndex::ColorSelect, 0x00),
    ],
};

/// Register values for Vga mode 320x200x256 Graphics.
pub const MODE_320X200X256_CONFIGURATION: VgaConfiguration = VgaConfiguration {
    // Configuration values acquired from https://www.singlix.com/trdos/archive/vga/Graphics%20in%20pmode.pdf
    miscellaneous_output: 0x63,
    sequencer_registers: &[
        (SequencerIndex::SequencerReset, 0x03),
        (SequencerIndex::ClockingMode, 0x01),
        (SequencerIndex::PlaneMask, 0x0F),
        (SequencerIndex::CharacterFont, 0x00),
        (SequencerIndex::MemoryMode, 0x0E),
    ],
    crtc_controller_registers: &[
        (CrtcControllerIndex::HorizontalTotal, 0x5F),
        (CrtcControllerIndex::HorizontalDisplayEnableEnd, 0x4F),
        (CrtcControllerIndex::HorizontalBlankingStart, 0x50),
        (CrtcControllerIndex::HorizontalBlankingEnd, 0x82),
        (CrtcControllerIndex::HorizontalSyncStart, 0x54),
        (CrtcControllerIndex::HorizontalSyncEnd, 0x80),
        (CrtcControllerIndex::VeritcalTotal, 0xBF),
        (CrtcControllerIndex::Overflow, 0x1F),
        (CrtcControllerIndex::PresetRowScan, 0x00),
        (CrtcControllerIndex::MaximumScanLine, 0x41),
        (CrtcControllerIndex::TextCursorStart, 0x00),
        (CrtcControllerIndex::TextCursorEnd, 0x00),
        (CrtcControllerIndex::StartAddressHigh, 0x00),
        (CrtcControllerIndex::StartAddressLow, 0x00),
        (CrtcControllerIndex::TextCursorLocationHigh, 0x00),
        (CrtcControllerIndex::TextCursorLocationLow, 0x00),
        (CrtcControllerIndex::VerticalSyncStart, 0x9C),
        (CrtcControllerIndex::VerticalSyncEnd, 0x0E),
        (CrtcControllerIndex::VerticalDisplayEnableEnd, 0x8F),
        (CrtcControllerIndex::Offset, 0x28),
        (CrtcControllerIndex::UnderlineLocation, 0x40),
        (CrtcControllerIndex::VerticalBlankingStart, 0x96),
        (CrtcControllerIndex::VerticalBlankingEnd, 0xB9),
        (CrtcControllerIndex::ModeControl, 0xA3),
        (CrtcControllerIndex::LineCompare, 0xFF),
    ],
    graphics_controller_registers: &[
        (GraphicsControllerIndex::SetReset, 0x00),
        (GraphicsControllerIndex::EnableSetReset, 0x00),
        (GraphicsControllerIndex::ColorCompare, 0x00),
        (GraphicsControllerIndex::DataRotate, 0x00),
        (GraphicsControllerIndex::ReadPlaneSelect, 0x00),
        (GraphicsControllerIndex::GraphicsMode, 0x40),
        (GraphicsControllerIndex::Miscellaneous, 0x05),
        (GraphicsControllerIndex::ColorDontCare, 0x0F),
        (GraphicsControllerIndex::BitMask, 0xFF),
    ],
    attribute_controller_registers: &[
        (AttributeControllerIndex::PaletteRegister0, 0x00),
        (AttributeControllerIndex::PaletteRegister1, 0x01),
        (AttributeControllerIndex::PaletteRegister2, 0x02),
        (AttributeControllerIndex::PaletteRegister3, 0x03),
        (AttributeControllerIndex::PaletteRegister4, 0x04),
        (AttributeControllerIndex::PaletteRegister5, 0x05),
        (AttributeControllerIndex::PaletteRegister6, 0x06),
        (AttributeControllerIndex::PaletteRegister7, 0x07),
        (AttributeControllerIndex::PaletteRegister8, 0x08),
        (AttributeControllerIndex::PaletteRegister9, 0x09),
        (AttributeControllerIndex::PaletteRegisterA, 0x0A),
        (AttributeControllerIndex::PaletteRegisterB, 0x0B),
        (AttributeControllerIndex::PaletteRegisterC, 0x0C),
        (AttributeControllerIndex::PaletteRegisterD, 0x0D),
        (AttributeControllerIndex::PaletteRegisterE, 0x0E),
        (AttributeControllerIndex::PaletteRegisterF, 0x0F),
        (AttributeControllerIndex::ModeControl, 0x41),
        (AttributeControllerIndex::OverscanColor, 0x00),
        (AttributeControllerIndex::MemoryPlaneEnable, 0x0F),
        (AttributeControllerIndex::HorizontalPixelPanning, 0x00),
        (AttributeControllerIndex::ColorSelect, 0x00),
    ],
};

/// Register values for Vga mode 320x200x256x Graphics.
pub const MODE_320X240X256_CONFIGURATION: VgaConfiguration = VgaConfiguration {
    // Configuration values acquired from https://www.singlix.com/trdos/archive/vga/Graphics%20in%20pmode.pdf
    miscellaneous_output: 0x63,
    sequencer_registers: &[
        (SequencerIndex::SequencerReset, 0x03),
        (SequencerIndex::ClockingMode, 0x01),
        (SequencerIndex::PlaneMask, 0x0F),
        (SequencerIndex::CharacterFont, 0x00),
        (SequencerIndex::MemoryMode, 0x06),
    ],
    crtc_controller_registers: &[
        (CrtcControllerIndex::HorizontalTotal, 0x5F),
        (CrtcControllerIndex::HorizontalDisplayEnableEnd, 0x4F),
        (CrtcControllerIndex::HorizontalBlankingStart, 0x50),
        (CrtcControllerIndex::HorizontalBlankingEnd, 0x82),
        (CrtcControllerIndex::HorizontalSyncStart, 0x54),
        (CrtcControllerIndex::HorizontalSyncEnd, 0x80),
        (CrtcControllerIndex::VeritcalTotal, 0x0D),
        (CrtcControllerIndex::Overflow, 0x3E),
        (CrtcControllerIndex::PresetRowScan, 0x00),
        (CrtcControllerIndex::MaximumScanLine, 0x41),
        (CrtcControllerIndex::TextCursorStart, 0x00),
        (CrtcControllerIndex::TextCursorEnd, 0x00),
        (CrtcControllerIndex::StartAddressHigh, 0x00),
        (CrtcControllerIndex::StartAddressLow, 0x00),
        (CrtcControllerIndex::TextCursorLocationHigh, 0x00),
        (CrtcControllerIndex::TextCursorLocationLow, 0x00),
        (CrtcControllerIndex::VerticalSyncStart, 0xEA),
        (CrtcControllerIndex::VerticalSyncEnd, 0xAC),
        (CrtcControllerIndex::VerticalDisplayEnableEnd, 0xDF),
        (CrtcControllerIndex::Offset, 0x28),
        (CrtcControllerIndex::UnderlineLocation, 0x00),
        (CrtcControllerIndex::VerticalBlankingStart, 0xE7),
        (CrtcControllerIndex::VerticalBlankingEnd, 0x06),
        (CrtcControllerIndex::ModeControl, 0xE3),
        (CrtcControllerIndex::LineCompare, 0xFF),
    ],
    graphics_controller_registers: &[
        (GraphicsControllerIndex::SetReset, 0x00),
        (GraphicsControllerIndex::EnableSetReset, 0x00),
        (GraphicsControllerIndex::ColorCompare, 0x00),
        (GraphicsControllerIndex::DataRotate, 0x00),
        (GraphicsControllerIndex::ReadPlaneSelect, 0x00),
        (GraphicsControllerIndex::GraphicsMode, 0x40),
        (GraphicsControllerIndex::Miscellaneous, 0x05),
        (GraphicsControllerIndex::ColorDontCare, 0x0F),
        (GraphicsControllerIndex::BitMask, 0xFF),
    ],
    attribute_controller_registers: &[
        (AttributeControllerIndex::PaletteRegister0, 0x00),
        (AttributeControllerIndex::PaletteRegister1, 0x01),
        (AttributeControllerIndex::PaletteRegister2, 0x02),
        (AttributeControllerIndex::PaletteRegister3, 0x03),
        (AttributeControllerIndex::PaletteRegister4, 0x04),
        (AttributeControllerIndex::PaletteRegister5, 0x05),
        (AttributeControllerIndex::PaletteRegister6, 0x06),
        (AttributeControllerIndex::PaletteRegister7, 0x07),
        (AttributeControllerIndex::PaletteRegister8, 0x08),
        (AttributeControllerIndex::PaletteRegister9, 0x09),
        (AttributeControllerIndex::PaletteRegisterA, 0x0A),
        (AttributeControllerIndex::PaletteRegisterB, 0x0B),
        (AttributeControllerIndex::PaletteRegisterC, 0x0C),
        (AttributeControllerIndex::PaletteRegisterD, 0x0D),
        (AttributeControllerIndex::PaletteRegisterE, 0x0E),
        (AttributeControllerIndex::PaletteRegisterF, 0x0F),
        (AttributeControllerIndex::ModeControl, 0x41),
        (AttributeControllerIndex::OverscanColor, 0x00),
        (AttributeControllerIndex::MemoryPlaneEnable, 0x0F),
        (AttributeControllerIndex::HorizontalPixelPanning, 0x00),
        (AttributeControllerIndex::ColorSelect, 0x00),
    ],
};

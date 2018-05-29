//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// Mock good response to a LOG command
pub const LOG_RESPONSE_GOOD: [u8; 38] = [
    0xAA, 0x44, 0x12, 0x1C, 0x1, 0x0, 0x80, 0x20, 0x6, 0x0, 0x0, 0x0, 0xFF, 0x78, 0xD1, 0xB, 0x6,
    0x67, 0xC9, 0x9, 0x0, 0x0, 0x0, 0x0, 0xFB, 0xFD, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x4F, 0x4B,
    0x10, 0x9D, 0x33, 0xB1,
];

// Mock good response to an UNLOG command
pub const UNLOG_RESPONSE_GOOD: [u8; 38] = [
    0xAA, 0x44, 0x12, 0x1C, 0x24, 0x0, 0x80, 0x20, 0x6, 0x0, 0x0, 0x0, 0xFF, 0x78, 0xD1, 0xB, 0x85,
    0x6F, 0xC9, 0x9, 0x0, 0x0, 0x0, 0x0, 0xFB, 0xFD, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x4F, 0x4B,
    0x41, 0xFE, 0xF5, 0x3F,
];

// Mock good response to an UNLOGALL command
pub const UNLOG_ALL_RESPONSE_GOOD: [u8; 38] = [
    0xAA, 0x44, 0x12, 0x1C, 0x26, 0x0, 0x80, 0x20, 0x6, 0x0, 0x0, 0x0, 0xFF, 0x78, 0xD1, 0xB, 0x45,
    0x80, 0xC9, 0x9, 0x0, 0x0, 0x0, 0x0, 0xFB, 0xFD, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x4F, 0x4B,
    0x83, 0x1, 0x39, 0x5C,
];

pub const LOG_VERSION_COMMAND: [u8; 64] = [
    0xAA, 0x44, 0x12, 0x1C, 0x1, 0x0, 0x0, 0xC0, 0x20, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x20, 0x0, 0x0, 0x0, 0x25, 0x0, 0x0, 0x0,
    0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x30, 0x8E, 0x33, 0x3C,
];

pub const VERSION_LOG: [u8; 144] = [
    0xAA, 0x44, 0x12, 0x1C, 0x25, 0x0, 0x0, 0x20, 0x70, 0x0, 0x0, 0x0, 0x7D, 0x78, 0xD1, 0xB, 0x38,
    0x5E, 0xC9, 0x9, 0x0, 0x0, 0x48, 0x0, 0x81, 0x36, 0xFA, 0x33, 0x1, 0x0, 0x0, 0x0, 0x1, 0x0,
    0x0, 0x0, 0x47, 0x31, 0x53, 0x42, 0x30, 0x47, 0x54, 0x54, 0x30, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x42, 0x4A, 0x59, 0x41, 0x31, 0x35, 0x31, 0x32, 0x30, 0x30, 0x33, 0x38, 0x48, 0x0, 0x0,
    0x0, 0x4F, 0x45, 0x4D, 0x36, 0x31, 0x35, 0x2D, 0x32, 0x2E, 0x30, 0x30, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x4F, 0x45, 0x4D, 0x30, 0x36, 0x30, 0x36, 0x30, 0x30, 0x52, 0x4E, 0x30, 0x30, 0x30, 0x30, 0x0,
    0x4F, 0x45, 0x4D, 0x30, 0x36, 0x30, 0x32, 0x30, 0x31, 0x52, 0x42, 0x30, 0x30, 0x30, 0x30, 0x0,
    0x32, 0x30, 0x31, 0x35, 0x2F, 0x4A, 0x61, 0x6E, 0x2F, 0x32, 0x38, 0x0, 0x31, 0x35, 0x3A, 0x32,
    0x37, 0x3A, 0x32, 0x39, 0x0, 0x0, 0x0, 0x0, 0xC6, 0x5E, 0x86, 0x47,
];

pub const ERROR_LOG: [u8; 76] = [
    0xAA, 0x44, 0x12, 0x1C, 0x5E, 0x00, 0x00, 0x20, 0x2C, 0x00, 0x00, 0x00, 0x34, 0x82, 0xAE, 0xB0,
    0x56, 0x18, 0x00, 0x00, 0x48, 0x00, 0x67, 0xB9, 0x00, 0x00, 0xC0, 0x07, 0x01, 0x00, 0x00, 0x00,
    0x13, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x4E, 0x6F, 0x20, 0x56, 0x61, 0x6C, 0x69, 0x64,
    0x20, 0x50, 0x6F, 0x73, 0x69, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x43, 0x61, 0x6C, 0x63, 0x75, 0x6C,
    0x61, 0x74, 0x65, 0x64, 0x00, 0x00, 0x00, 0x00, 0x01, 0x5C, 0xD2, 0xDB,
];
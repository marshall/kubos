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
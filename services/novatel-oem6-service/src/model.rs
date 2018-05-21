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

use failure::Fail;
use novatel_oem6_api::*;
use std::cell::{Cell, RefCell};
use std::io::Error;
use std::sync::mpsc::sync_channel;

use objects::*;

pub struct Subsystem {
    pub oem: OEM6,
    pub last_cmd: Cell<AckCommand>,
    pub errors: RefCell<Vec<String>>,
}

impl Subsystem {
    pub fn new(bus: &'static str) -> OEMResult<Subsystem> {
        let (_log_send, log_recv) = sync_channel(5);
        let (_response_send, response_recv) = sync_channel(5);

        let oem = OEM6::new(bus, BaudRate::Baud9600, log_recv, response_recv)?;

        // TODO: Spawn read thread. Will use `log_send` and `response_send`

        println!("Kubos OEM6 service started");

        Ok(Subsystem {
            oem,
            last_cmd: Cell::new(AckCommand::None),
            errors: RefCell::new(vec![]),
        })
    }

    pub fn configure_hardware(
        &self,
        input: Vec<ConfigStruct>,
    ) -> Result<ConfigureHardwareResponse, Error> {
        let mut success = true;
        let mut errors = "".to_owned();
        let mut config = "".to_owned();

        for entry in input.iter() {
            let result = run!(
                match entry.option {
                    ConfigOption::LogErrorData => self.oem.request_errors(entry.hold),
                    ConfigOption::LogPositionData => {
                        self.oem
                            .request_position(entry.interval, entry.offset, entry.hold)
                    }
                    ConfigOption::UnlogAll => self.oem.request_unlog_all(entry.hold),
                    ConfigOption::UnlogErrorData => {
                        self.oem.request_unlog(MessageID::RxStatusEvent)
                    }
                    ConfigOption::UnlogPositionData => self.oem.request_unlog(MessageID::BestXYZ),
                },
                self.errors
            );

            success &= result.is_ok();
            if let Err(err) = result {
                if !errors.is_empty() {
                    errors.push_str(". ");
                }
                errors.push_str(&format!("{:?}: {}", entry.option, err));
            }

            if !config.is_empty() {
                config.push_str(", ");
            }
            config.push_str(&format!("{:?}(Hold: {})", entry.option, entry.hold));
            if entry.interval != 0.0 {
                config.push_str(&format!(": {}+{}sec", entry.interval, entry.offset));
            }
        }

        Ok(ConfigureHardwareResponse {
            success,
            errors,
            config,
        })
    }

    pub fn passthrough(&self, command: String) -> Result<GenericResponse, Error> {
        // Convert the hex values in the string into actual hex values
        // Ex. "c3c2" -> [0xc3, 0xc2]
        let tx: Vec<u8> = command
            .as_bytes()
            .chunks(2)
            .into_iter()
            .map(|chunk| u8::from_str_radix(::std::str::from_utf8(chunk).unwrap(), 16).unwrap())
            .collect();

        let result = run!(self.oem.passthrough(tx.as_slice()), self.errors);

        Ok(GenericResponse {
            success: result.is_ok(),
            errors: match result {
                Ok(_) => "".to_owned(),
                Err(err) => err,
            },
        })
    }
}

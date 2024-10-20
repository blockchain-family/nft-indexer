use anyhow::Result;
use nekoton_abi::*;

use super::RunLocalSimple;

pub mod sid;

#[derive(Copy, Clone)]
pub struct SidContract<'a>(pub ExecutionContext<'a>);

impl SidContract<'_> {
    pub fn supports_interfaces(&self, interfaces: &[u32]) -> Result<bool> {
        let mut inputs: Option<[ton_abi::Token; 2]> = None;

        for &interface in interfaces {
            let inputs = match &mut inputs {
                Some(inputs) => {
                    inputs[1] = make_interface_id(interface);
                    inputs
                }
                None => inputs.insert([
                    0u32.token_value().named("answerId"),
                    make_interface_id(interface),
                ]),
            };

            if !self
                .0
                .run_local_responsible_simple(sid::supports_interface(), inputs.as_ref())?
                .unpack_first::<bool>()?
            {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn supports_interface(&self, interface: u32) -> Result<bool> {
        let inputs = [
            0u32.token_value().named("answerId"),
            make_interface_id(interface),
        ];
        let result = self
            .0
            .run_local_responsible_simple(sid::supports_interface(), &inputs)?
            .unpack_first()?;
        Ok(result)
    }
}

fn make_interface_id(interface: u32) -> ton_abi::Token {
    interface.token_value().named("interfaceID")
}

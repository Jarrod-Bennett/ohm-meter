#![no_std]
#![cfg_attr(test, no_main)]

use ohm_meter as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {}

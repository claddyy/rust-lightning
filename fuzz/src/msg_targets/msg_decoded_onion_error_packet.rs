// This file is Copyright its original authors, visible in version control
// history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

// This file is auto-generated by gen_target.sh based on msg_target_template.txt
// To modify it, modify msg_target_template.txt and run gen_target.sh instead.

#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::msg_targets::utils::VecWriter;
use crate::utils::test_logger;

#[inline]
pub fn msg_decoded_onion_error_packet_test<Out: test_logger::Output>(data: &[u8], _out: Out) {
	test_msg!(lightning::ln::msgs::DecodedOnionErrorPacket, data);
}

#[no_mangle]
pub extern "C" fn msg_decoded_onion_error_packet_run(data: *const u8, datalen: usize) {
	let data = unsafe { std::slice::from_raw_parts(data, datalen) };
	test_msg!(lightning::ln::msgs::DecodedOnionErrorPacket, data);
}

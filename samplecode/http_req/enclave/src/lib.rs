// Copyright (C) 2017-2019 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![crate_name = "httpreqenclave"]
#![crate_type = "staticlib"]
#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

#[cfg(target_env = "sgx")]
extern crate sgx_types;

use http_req::{request::{Method, RequestBuilder}, tls, uri::Uri};
use sgx_types::*;
use std::ffi::CStr;
use std::net::TcpStream;
use std::os::raw::c_char;
use std::prelude::v1::*;

#[no_mangle]
pub extern "C" fn send_http_request(fd: c_int, hostname: *const c_char) -> sgx_status_t {
    let hostname = unsafe { CStr::from_ptr(hostname).to_str() };
    let hostname = hostname.expect("Failed to recover hostname");

    //Parse uri and assign it to variable `addr`
    let addr: Uri = hostname.parse().unwrap();

    //Connect to remote host
    let stream = TcpStream::new(fd).unwrap();

    //Open secure connection over TlsStream, because of `addr` (https)
    let mut stream = tls::Config::default()
        .connect(addr.host().unwrap_or(""), stream)
        .unwrap();

    //Container for response's body
    let mut writer = Vec::new();

    let param_body = "MerSplitMsg=999991905069036%5E90%3B999991905069037%5E10&TranType=0004&TranDate=20190930&SplitMethod=1&RemoteAddr=101.87.163.240&BusiType=0001&OrderAmt=8641&Version=20150922&CardTranData=E%2FAq3HeyFKBS0vKpewplTJV6Mr1MtdOG%2Frfz63V090exTJJ9%2BDALM6%2B2xcFSLxStzjiMcXm4xzETYi%2Fb7zEPhA5vlsBBPigTsa%2Fqukj0DzbpGV0hFo4JE%2BeNM5AsAPXlSDX12nI%2BFkEeK9aZdr9I1yuwa%2FVyxFwNJgKx3h84Oh%2BJkW4nJk%2FApb9k7F1gto2VWPr%2FXwzAyTwNpNJkFQjfuosZCMbjWUClnzsuco4wxuRi9F7Ek%2FQHjU7QL9IEoQJIWsmW2CTfnwoofHTJsX08jwluWV69Q2G6QqLgCQEwGNag4yifzNM4V2fxePJ6TFs5JZh7OghgSBmBNPe6CaqHug%3D%3D&MerOrderNo=20190926094524339&Signature=Ps8ZH8KXNpkCKBiIlQXU3FP9S2Mix%2FG5MNTyC7%2B2KS48vC2W1K%2FAg9B3V7jLxBcdyu4BoKUm4sNC5qBO7%2BcW3g0dptfn%2FYq8PdYlqov1K4SlkvhE5Zu4erpQv3W8%2Bgflxcyf67omUTfV%2BoT9BmSAxqlJFU%2FiJ3uJlBM25OD8%2FqnQSAqB7RzW6eHlsAyZtKnZnQiB5uOrOCLrI8tt1hJlPaGfcBIbhBZh33uuwym7PUns%2BeYl82vX%2FnAZpG8s5CUwXbD%2BcBrdVN6HFji7DVJjPfN0smeP5alWNHt65cWm%2FvioOlG0RqobUD4eOg54lDRUDoD9zkJ44jm99k50qlt9gg%3D%3D&SplitType=0001&MerId=000091905069034&MerBgUrl=http%3A%2F%2F127.0.0.1%3A8085%2Fsgx%2Ftransaction%2Fnotify%2Fback&TranTime=090756";

    //Add header `Connection: Close`
    let response = RequestBuilder::new(&addr)
        .method(Method::POST)
        .header("Content-type", "application/x-www-form-urlencoded;charset=UTF-8")
        .body(param_body.as_bytes())
        .send(&mut stream, &mut writer)
        .unwrap();

    println!("{}", String::from_utf8_lossy(&writer));
    println!("Status: {} {}", response.status_code(), response.reason());

    sgx_status_t::SGX_SUCCESS
}

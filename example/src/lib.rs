// Copyright 2019. Starry, Inc. All Rights Reserved.
//
// MIT License
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Software written by Preston Carpenter <pcarpenter@starry.com>
use telegraf_plugin::link_to_go;

macro_rules! map(
    { $($key:expr => $value:expr),* } => {
        {
            #[allow(unused_mut)]
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )*
                m
        }
    };
);

#[link_to_go("Description of the plugin", "<sample_config>")]
fn collect_metric() {
    let fields = map! {"rust_key".into() => "rust_value".into()};
    let tags = map! {
        "int".into() => 100i8.into(),
        "uint".into() => 255u8.into(),
        "double".into() => 3.1459.into(),
        "bool".into() => (!false).into(),
        "string".into() => "hello world".into()
    };
    record_field("rust-field", &fields, &tags, None);
    record_gauge("rust-gauge", &fields, &tags, None);
    record_counter("rust-counter", &fields, &tags, None);
    record_summary("rust-summary", &fields, &tags, None);
    record_histogram("rust-histogram", &fields, &tags, None);
}

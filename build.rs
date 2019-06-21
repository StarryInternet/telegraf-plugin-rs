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
fn main() {
    let builder = bindgen::builder()
        .derive_debug(true)
        .derive_default(true)
        .derive_copy(true)
        // NOTE This is due to implementing `Drop` in the macro.
        .no_copy("go_value")
        .generate_comments(true)
        .blacklist_function("sample_config")
        .blacklist_function("description")
        .blacklist_function("gather")
        .header("telegraf-plugin-go-glue/c_api.h")
        .ctypes_prefix("libc");
    let generated = builder.generate().unwrap();
    generated.write_to_file("src/gen.rs").unwrap();
}

/*
MIT No Attribution

Copyright 2022 Rickard Ernst Björn Lundin

Permission is hereby granted, free of charge, to any person obtaining a copy of this
software and associated documentation files (the "Software"), to deal in the Software
without restriction, including without limitation the rights to use, copy, modify,
merge, publish, distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
use arrow2::array::Array;
use chrono::offset::Local;
use arrowalloy::api::OneShot;

macro_rules! info {
    ($($arg:tt)*) => {
        println!(
            "{} [INFO] [Rust]\t{}",
            Local::now().format("[%Y-%m-%d %H:%M:%S]"),
            format!($($arg)*)
        );
    }
}


struct DeltaBackend {
    ddlname: String,
}

impl OneShot for DeltaBackend {
    fn set_lib(&self) -> usize {
        1

    }

    fn from_chunks(&self, arrays: Vec<Box<dyn Array>>) -> usize {

        info!("Hello! Delta backend , displaying given table and its columns");

        for (i, array) in arrays.iter().enumerate() {
            info!("array{}: {:?}", i + 1, array);
        }

        arrays.len()
    }
}

error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        InvalidMagicNumber(got: u32) {
            description("invalid magic number")
            display("invalid magic number: got 0x{:x} but expected 0x{:x}",
                    got, ::raw::class_file::MAGIC)
        }
    }
}

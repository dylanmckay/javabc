error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        MalformedFile(message: String) {
            description("malformed file")
            display("malformed file: {}", message)
        }
    }
}


error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Toml(::toml::de::Error);
        Image(::image::ImageError);
    }
    errors {
        MissingField(t: String) {
            description("Missing necessary field")
            display("Missing necessary field : {}", t)
        }
        UnexpectedValue(field: String, value: String) {
            description("Unexpected name")
            display("Unexpected value \"{}\" for field {}", value, field)
        }
        ImageSizeError(input: (u32, u32), image: (u32, u32)) {
            description("Contradiction between toml input and image file")
            display("Expected size is ({}, {}), but png file size is ({}, {})",
                        input.0, input.1, image.0, image.1)
        }
    }
}


const MEGABYTE: u64 = 10_u64.pow(6);
// size and start with the name `image_occlusion...`
// compress the files, then write to the exact same location
// problem: image compression takes a FK TON of time
fn main() {
    let anki_dir = {
        let mut t = dirs::data_dir().unwrap();
        t.push("Anki2");
        t.push("User 1");
        t.push("collection.media");
        t
    };

    std::fs::read_dir(&anki_dir)
        .unwrap()
        .filter_map(|f| f.ok())
        .for_each(|f| {
            let n = f.file_name();
            let name = n.to_str().unwrap();
            if name.ends_with("jpeg") && name.contains("image_occlusion") {
                // try getting the size of the file
                if f.metadata().unwrap().len() > 2 * MEGABYTE {
                    let data = std::fs::read(f.path()).unwrap();

                    let img = image::load_from_memory(&data).unwrap();

                    let buffer = std::fs::File::create(f.path()).unwrap();

                    let mut img_encoder =
                        image::codecs::jpeg::JpegEncoder::new_with_quality(buffer, 50);

                    img_encoder
                        .encode(img.as_bytes(), img.width(), img.height(), img.color())
                        .unwrap();
                }
            }
        });
}

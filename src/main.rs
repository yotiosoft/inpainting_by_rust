use texture_synthesis as ts;

fn main() -> Result<(), ts::Error> {
    let texsynth = ts::Session::builder()
        // let the generator know which part we would like to fill in
        // if we had more examples, they would be additional information
        // the generator could use to inpaint
        .inpaint_example(
            &"imgs/5_mask.png",
            // load a "corrupted" example with missing red information we would like to fill in
            ts::Example::builder(&"imgs/5.png")
                // we would also like to prevent sampling from "corrupted" red areas
                // otherwise, generator will treat that those as valid areas it can copy from in the example,
                // we could also use SampleMethod::Ignore to ignore the example altogether, but we
                // would then need at least 1 other example image to actually source from
                // example.set_sample_method(ts::SampleMethod::Ignore);
                .set_sample_method(&"imgs/5_mask.png"),
            // Inpaint requires that inputs and outputs be the same size, so it's a required
            // parameter that overrides both `resize_input` and `output_size`
            ts::Dims::new(1800, 1355),
        )
        // Ignored
        .resize_input(ts::Dims::new(1800, 1355))
        // Ignored
        .output_size(ts::Dims::new(1800, 1355))
        .build()?;

    let generated = texsynth.run(None);

    //save the result to the disk
    generated.save("out/05.png")
}

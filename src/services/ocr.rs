use anyhow::*;
use image::DynamicImage;
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;

pub fn image2text(image: DynamicImage) -> Result<String, Error> {
    let detection_model_data = include_bytes!("../../models/text-detection.rten");
    let recognition_model_data = include_bytes!("../../models/text-recognition.rten");

    let detection_model = Model::load_static_slice(detection_model_data)?;
    let recognition_model = Model::load_static_slice(recognition_model_data)?;

    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })?;

    let image = image.to_rgba8();

    // Apply standard image pre-processing expected by this library (convert
    // to greyscale, map range to [-0.5, 0.5]).
    let image_source = ImageSource::from_bytes(image.as_raw(), image.dimensions())?;
    let ocr_input = engine.prepare_input(image_source)?;

    // Detect and recognize text. If you only need the text and don't need any
    // layout information, you can also use `engine.get_text(&ocr_input)`,
    // which returns all the text in an image as a single string.

    // Get oriented bounding boxes of text words in input image.
    let word_rects = engine.detect_words(&ocr_input)?;

    // Group words into lines. Each line is represented by a list of word
    // bounding boxes.
    let line_rects = engine.find_text_lines(&ocr_input, &word_rects);

    // Recognize the characters in each line.
    let line_texts = engine.recognize_text(&ocr_input, &line_rects)?;

    let mut text = String::from("");

    for line in line_texts
        .iter()
        .flatten()
        // Filter likely spurious detections. With future model improvements
        // this should become unnecessary.
        .filter(|l| l.to_string().len() > 1)
    {
        text.push_str(&line.to_string());
        text.push_str("\n");
    }

    Ok(text)
}

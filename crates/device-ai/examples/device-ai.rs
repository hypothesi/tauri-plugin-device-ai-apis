use std::{env, process};

use device_ai::{
    BarcodeOptions, ClassificationOptions, DeviceAi, ImageSource, LlmGenerateOptions,
    LlmRewriteOptions, LlmRewriteTone, LlmSessionOptions, LlmStreamEvent, LlmSummarizeOptions,
    OcrOptions, RecognitionOptions, Result, SynthesisOptions,
};
use serde::Serialize;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        print_usage();
        process::exit(1);
    };

    let ai = DeviceAi::new();

    match command.as_str() {
        "capabilities" => print_json(&ai.capabilities()),
        "speech-recognize" => {
            let options = match args.next() {
                Some(path) => RecognitionOptions::new()
                    .with_audio_source(device_ai::AudioSource::from_path(path)),
                None => RecognitionOptions::new()
                    .with_audio_source(device_ai::AudioSource::microphone()),
            };

            print_json(&ai.speech().recognize(options)?)
        }
        "speech-stream" => {
            let result = ai.speech().start_recognition(RecognitionOptions::new());

            match result {
                Ok(session_id) => println!("Started unexpected streaming session: {session_id}"),
                Err(error) => eprintln!("{error}"),
            }

            Ok(())
        }
        "speech-voices" => print_json(&ai.speech().voices()?),
        "speech-speak" => {
            let text = require_remaining(args);

            ai.speech().speak(&text, SynthesisOptions::new())?;
            println!("Speech synthesis completed.");
            Ok(())
        }
        "vision-ocr" => {
            let path = require_next(&mut args, "vision-ocr <image-path>");
            print_json(
                &ai.vision()
                    .recognize_text(ImageSource::from_path(path), OcrOptions::new())?,
            )
        }
        "vision-barcodes" => {
            let path = require_next(&mut args, "vision-barcodes <image-path>");
            print_json(
                &ai.vision()
                    .detect_barcodes(ImageSource::from_path(path), BarcodeOptions::new())?,
            )
        }
        "vision-faces" => {
            let path = require_next(&mut args, "vision-faces <image-path>");
            print_json(
                &ai.vision().detect_faces(
                    ImageSource::from_path(path),
                    device_ai::FaceOptions::new()
                        .detect_landmarks(true)
                        .classify_attributes(true),
                )?,
            )
        }
        "vision-classify" => {
            let path = require_next(&mut args, "vision-classify <image-path>");
            print_json(
                &ai.vision().classify_image(
                    ImageSource::from_path(path),
                    ClassificationOptions::new()
                        .max_results(5)
                        .min_confidence(0.5),
                )?,
            )
        }
        "text-language" => {
            let text = require_remaining(args);

            print_json(&ai.text().identify_language(&text)?)
        }
        "text-translate" => {
            let from = require_next(&mut args, "text-translate <from> <to> <text>");
            let to = require_next(&mut args, "text-translate <from> <to> <text>");
            let text = require_remaining(args);

            print_json(&ai.text().translate(&text, &from, &to)?)
        }
        "llm-availability" => print_json(&ai.llm().check_availability()?),
        "llm-model-info" => print_json(&ai.llm().model_info()?),
        "llm-generate" => {
            let prompt = require_remaining(args);

            print_json(&ai.llm().generate(LlmGenerateOptions::new(prompt))?)
        }
        "llm-stream" => {
            let prompt = require_remaining(args);

            ai.llm()
                .generate_stream(LlmGenerateOptions::new(prompt), |event| {
                    print_stream_event(event);
                    Ok(())
                })?;
            Ok(())
        }
        "llm-session" => {
            let prompt = require_remaining(args);

            let session_id = ai.llm().create_session(LlmSessionOptions::new())?;
            let reply = ai.llm().session_send(session_id.clone(), prompt)?;

            print_json(&reply)?;
            ai.llm().destroy_session(session_id)
        }
        "llm-session-stream" => {
            let prompt = require_remaining(args);

            let session_id = ai.llm().create_session(LlmSessionOptions::new())?;
            let result = ai
                .llm()
                .session_send_stream(session_id.clone(), prompt, |event| {
                    print_stream_event(event);
                    Ok(())
                });
            ai.llm().destroy_session(session_id)?;
            result
        }
        "llm-summarize" => {
            let text = require_remaining(args);

            print_json(&ai.llm().summarize(LlmSummarizeOptions::new(text))?)
        }
        "llm-rewrite" => {
            let tone = require_next(&mut args, "llm-rewrite <casual|formal|professional> <text>");
            let text = require_remaining(args);

            let tone = match tone.as_str() {
                "casual" => LlmRewriteTone::Casual,
                "formal" => LlmRewriteTone::Formal,
                "professional" => LlmRewriteTone::Professional,
                _ => {
                    print_usage();
                    process::exit(1);
                }
            };

            print_json(&ai.llm().rewrite(LlmRewriteOptions::new(text).tone(tone))?)
        }
        "help" | "--help" | "-h" => {
            print_usage();
            Ok(())
        }
        _ => {
            print_usage();
            process::exit(1);
        }
    }
}

fn collect_remaining(args: impl Iterator<Item = String>) -> String {
    args.collect::<Vec<_>>().join(" ")
}

fn require_remaining(args: impl Iterator<Item = String>) -> String {
    let value = collect_remaining(args);

    if value.is_empty() {
        print_usage();
        process::exit(1);
    }

    value
}

fn print_json<T: Serialize>(value: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value).unwrap());
    Ok(())
}

fn print_stream_event(event: LlmStreamEvent) {
    match event {
        LlmStreamEvent::Delta { content } => print!("{content}"),
        LlmStreamEvent::Done {
            content,
            finish_reason,
            usage,
        } => {
            if !content.is_empty() {
                println!("\n---");
                println!("{content}");
            } else {
                println!();
            }
            println!("finishReason: {finish_reason:?}");
            if let Some(usage) = usage {
                println!(
                    "usage: prompt={:?}, completion={:?}, total={:?}",
                    usage.prompt_tokens, usage.completion_tokens, usage.total_tokens
                );
            }
        }
        LlmStreamEvent::Error { message } => eprintln!("stream error: {message}"),
    }
}

fn require_next(args: &mut impl Iterator<Item = String>, usage: &str) -> String {
    require_one(args.next(), usage)
}

fn require_one(value: Option<String>, usage: &str) -> String {
    match value {
        Some(value) => value,
        None => {
            eprintln!("Usage: cargo run -p device-ai --example device-ai -- {usage}");
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!(
        "Usage:
  cargo run -p device-ai --example device-ai -- capabilities
  cargo run -p device-ai --example device-ai -- speech-recognize [audio-path]
  cargo run -p device-ai --example device-ai -- speech-stream
  cargo run -p device-ai --example device-ai -- speech-voices
  cargo run -p device-ai --example device-ai -- speech-speak <text>
  cargo run -p device-ai --example device-ai -- vision-ocr <image-path>
  cargo run -p device-ai --example device-ai -- vision-barcodes <image-path>
  cargo run -p device-ai --example device-ai -- vision-faces <image-path>
  cargo run -p device-ai --example device-ai -- vision-classify <image-path>
  cargo run -p device-ai --example device-ai -- text-language <text>
  cargo run -p device-ai --example device-ai -- text-translate <from> <to> <text>
  cargo run -p device-ai --example device-ai -- llm-availability
  cargo run -p device-ai --example device-ai -- llm-model-info
  cargo run -p device-ai --example device-ai -- llm-generate <prompt>
  cargo run -p device-ai --example device-ai -- llm-stream <prompt>
  cargo run -p device-ai --example device-ai -- llm-session <prompt>
  cargo run -p device-ai --example device-ai -- llm-session-stream <prompt>
  cargo run -p device-ai --example device-ai -- llm-summarize <text>
  cargo run -p device-ai --example device-ai -- llm-rewrite <casual|formal|professional> <text>"
    );
}

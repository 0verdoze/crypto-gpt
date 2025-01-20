use std::{io::Cursor, sync::LazyLock, thread, time::Duration};

use async_openai::{config::OpenAIConfig, types::{ChatCompletionRequestMessage, ChatCompletionRequestMessageContentPart, ChatCompletionRequestMessageContentPartImageArgs, ChatCompletionRequestMessageContentPartTextArgs, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageArgs, ChatCompletionRequestUserMessageContent, CreateChatCompletionRequest, ImageUrlArgs, ImageUrlDetail}, Client};
use base64::Engine;
use clipboard_win::{set_clipboard, Getter};
use image::ImageFormat;
use keyboard_windows::{CharSender, Key, KeypressHandler, Mutex};
use tokio::task::JoinHandle;
use tray_logger::TrayLogger;

mod prompts;
mod tray_logger;

// cheatsheet
// ctrl + ; -> clear image from prompt (not cleared automatically)
// ctrl + ' -> change model for text prompt
// ctrl + [ -> change if model should get a summary
// ctrl + , -> save clipboard content (image/text) 
// ctrl + . -> run prompt
// ctrl + / -> paste latest output

static CONTEXT: LazyLock<Mutex<Context>> = LazyLock::new(Default::default);
static TYPER_CONTEXT: LazyLock<Mutex<TyperContext>> = LazyLock::new(Default::default);

pub static LOGGER: LazyLock<TrayLogger> = LazyLock::new(|| {
    TrayLogger::new(env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).build())
});

#[derive(Debug)]
struct Context {
    prompt: String,
    image: String,

    model: GptModel,
    summary_prompt: bool,
    
    job: Option<JoinHandle<()>>,
    output_prompt: Option<String>,
}

#[derive(Default)]
struct TyperContext {
    s: Vec<char>,
    head: usize,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum GptModel {
    Gpt35Turbo,
    #[default]
    Gpt4O,
    GptO1,
}

fn main() {
    log::set_logger(&*LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    KeypressHandler::set_keypress_callback(|key, was_hook_called| {
        if !was_hook_called && key.is_alphanumeric_symbol() {
            let mut ctx = TYPER_CONTEXT.lock();

            if ctx.head < ctx.s.len() {
                let next_char = ctx.s[ctx.head];
                ctx.head += 1;

                CharSender::send_char(next_char, true);

                if ctx.head == ctx.s.len() {
                    LOGGER.set_icon([120, 120, 120, 255]);
                }

                true
            } else {
                false
            }
        } else if key == Key::EscapeKey {
            let mut ctx = TYPER_CONTEXT.lock();
            if ctx.head < ctx.s.len() {
                ctx.head = ctx.s.len();
                LOGGER.set_icon([120, 120, 120, 255]);

                true
            } else {
                false
            }
        } else {
            false
        }
    });

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // clear image
    KeypressHandler::add_hotkey(&[Key::LControlKey, Key::SemicolonKey], |_| {
        log::info!("image cleared");
        CONTEXT.lock().image.clear();

        true
    });

    // save prompt/image
    KeypressHandler::add_hotkey(&[Key::LControlKey, Key::CommaKey], |_| {
        let mut ctx = CONTEXT.lock();
        ctx.prompt.clear();

        match get_clipboard() {
            ClipboardData::Image(img) => {
                log::info!("image captured");
                ctx.image = img
            },
            ClipboardData::Text(s) => {
                log::info!("text captured (`{s}`)");
                ctx.prompt = s
            },
            ClipboardData::NoData => {
                log::warn!("no data captured");
            }
        }

        true
    });

    // run prompt
    let runtime_handle = runtime.handle().clone();
    KeypressHandler::add_hotkey(&[Key::LControlKey, Key::PeriodKey], move |_| {
        let mut ctx = CONTEXT.lock();
        ctx.job.as_mut().map(|handle| handle.abort());
        
        ctx.job = None;
        ctx.output_prompt = None;

        runtime_handle.spawn(async {
            let result = worker(get_client()).await;

            match result {
                Ok(output) => {
                    let mut ctx = CONTEXT.lock();
                    ctx.output_prompt = output;
                    ctx.job = None;

                    let output_len = ctx.output_prompt.as_ref()
                        .map(String::len)
                        .unwrap_or_default();

                    let input_len = ctx.prompt.len();
                    let image = !ctx.image.is_empty();
                    
                    log::info!("output: {}", ctx.output_prompt.as_deref().unwrap_or_default());
                    log::info!("generation done, input len: {input_len}, image: {image}, output len: {output_len}");
                    LOGGER.set_icon([0, 127, 0, 255]);
                },
                Err(err) => {
                    log::error!("{}", err);
                }
            }
        });

        true
    });

    // paste output
    KeypressHandler::add_hotkey(&[Key::LControlKey, Key::SlashKey], |_| {
        let ctx = CONTEXT.lock();

        if let Some(output_prompt) = ctx.output_prompt.as_ref() {
            if let Err(err) = set_clipboard(clipboard_win::Unicode, output_prompt.clone()) {
                log::error!("unable to set prompt: {}", err);
            } else {
                log::info!("copied `{}`", output_prompt);
                LOGGER.set_icon([0, 0, 0, 255]);
            }
        } else {
            log::warn!("no prompt generated");
        }

        true
    });

    // ctrl + ' -> change model for text prompt
    KeypressHandler::add_hotkey(&[Key::LControlKey, Key::QuoteKey], |_| {
        let mut ctx = CONTEXT.lock();

        let new_model = match ctx.model {
            GptModel::Gpt35Turbo => GptModel::Gpt4O,
            GptModel::Gpt4O => GptModel::GptO1,
            GptModel::GptO1 => GptModel::Gpt35Turbo,
        };
        ctx.model = new_model;

        log::info!("model changed to {}", new_model.to_string());

        true
    });

    // ctrl + [ -> change if model should get a summary
    #[cfg(feature = "toggle_summary")]
    KeypressHandler::add_hotkey(&[Key::LControlKey, Key::LBracketKey], |_| {
        let mut ctx = CONTEXT.lock();

        ctx.summary_prompt = !ctx.summary_prompt;
        log::info!("summary changed to {}", ctx.summary_prompt);

        true
    });

    // ctrl + ] -> auto typer
    KeypressHandler::add_hotkey(&[Key::LControlKey, Key::RBracketKey], |_| {
        let ctx = CONTEXT.lock();
        let output = ctx.output_prompt.clone();
        drop(ctx);

        CharSender::release_key(Key::LControlKey, true);

        if let Some(output) = output {
            LOGGER.set_icon([160, 32, 240, 255]);

            let mut ctx = TYPER_CONTEXT.lock();
            ctx.head = 0;
            ctx.s = output.chars().collect();
        } else {
            log::warn!("auto type: no output generated")
        }

        true
    });

    // ctrl + c -> copy text, without notifing about c-key press
    // doesn't work for any webbrowser i tested, works okeish for windows forms apps
    #[cfg(feature = "handle_copy")]
    KeypressHandler::add_hotkey(&[Key::LControlKey, Key::CKey], |_| unsafe {
        let active_window = GetForegroundWindow();

        let active_window_thread = GetWindowThreadProcessId(active_window, None);
        let current_thread = GetCurrentThreadId();

        AttachThreadInput(active_window_thread, current_thread, true);

        let control_id = GetFocus();
        let mut buf = [0; 128];

        SendMessageW(control_id, WM_GETTEXT, WPARAM(buf.len()), LPARAM(buf.as_mut_ptr() as isize));
        // for testing
        println!("{}", String::from_utf16_lossy(&buf));

        let mut start = 0u32;
        let mut end = 0u32;
        SendMessageW(control_id, EM_GETSEL, WPARAM(mem::transmute(&mut start)), LPARAM(mem::transmute(&mut end)));

        if (start as usize) < buf.len() && (end as usize) < buf.len() && start <= end {
            // for testing
            println!("{}", String::from_utf16_lossy(&buf[start as usize..end as usize]));
        }

        AttachThreadInput(active_window_thread, current_thread, false);

        true
    });

    KeypressHandler::add_hotkey(&[Key::LControlKey, Key::EscapeKey], |_| {
        std::process::exit(0)
    });

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}

async fn worker(client: Client<OpenAIConfig>) -> anyhow::Result<Option<String>> {
    let ctx = CONTEXT.lock();

    let main_prompt = ChatCompletionRequestSystemMessageArgs::default()
        .content(prompts::MACHINE_LEARNING_EXPERT)
        .build()?;

    let summary_prompt = ChatCompletionRequestUserMessageArgs::default()
        .content(prompts::LESSON_SUMARY)
        .build()?;

    let mut user_prompt = ChatCompletionRequestUserMessage::default();
    let mut user_content_parts = vec![
        ChatCompletionRequestMessageContentPart::Text(
            ChatCompletionRequestMessageContentPartTextArgs::default()
                .text(&ctx.prompt)
                .build()?
        ),
    ];

    let contains_image;
    if ctx.image.len() > 0 {
        user_content_parts.push(ChatCompletionRequestMessageContentPart::Image(
            ChatCompletionRequestMessageContentPartImageArgs::default()
                .image_url(
                    ImageUrlArgs::default()
                        .url(&ctx.image)
                        .detail(ImageUrlDetail::High)
                        .build()?
                ).build()?
        ));

        contains_image = true;
    } else {
        contains_image = false;
    }

    user_prompt.content = ChatCompletionRequestUserMessageContent::Array(user_content_parts);

    let mut request = CreateChatCompletionRequest::default();

    if contains_image && request.model == GptModel::Gpt35Turbo {
        request.model = GptModel::Gpt4O.to_string();
    } else {
        request.model = ctx.model.to_string();
    }

    request.messages = vec![
        ChatCompletionRequestMessage::System(main_prompt),
    ];

    if ctx.summary_prompt {
        request.messages.push(ChatCompletionRequestMessage::User(summary_prompt));
    }

    request.messages.push(ChatCompletionRequestMessage::User(user_prompt));

    request.max_tokens = Some(2048);
    request.temperature = Some(1.3);

    drop(ctx);

    let resp = client.chat().create(request).await?;

    Ok((|| -> Option<String> {
        resp.choices
            .first()?
            .message
            .content
            .clone()
    })())
}

enum ClipboardData {
    Text(String),
    Image(String),
    NoData,
}

fn get_clipboard() -> ClipboardData {
    let clip = clipboard_win::Clipboard::new();
    if clip.is_err() {
        return ClipboardData::NoData;
    }

    get_clipboard_image().map(ClipboardData::Image)
        .or_else(|| get_clipboard_text().map(ClipboardData::Text))
        .unwrap_or(ClipboardData::NoData)
}

fn get_clipboard_image() -> Option<String> {
    let mut bytes: Vec<u8> = Vec::new();
    
    clipboard_win::formats::Bitmap.read_clipboard(&mut bytes).ok()?;
    if !bytes.is_empty() {
        let encoder = base64::engine::GeneralPurpose::new(&base64::alphabet::STANDARD, Default::default());
        
        let img = image::load_from_memory_with_format(&bytes, ImageFormat::Bmp).unwrap();
        
        let mut jpeg = Cursor::new(Vec::new());
        img.write_to(&mut jpeg, ImageFormat::Jpeg).unwrap();

        let encoded = encoder.encode(&jpeg.into_inner());
        
        Some(format!("data:image/jpeg;base64,{}", encoded))
    } else {
        None
    }
}

fn get_clipboard_text() -> Option<String> {
    let mut str = String::new();

    clipboard_win::formats::Unicode.read_clipboard(&mut str).ok()?;
    (str.len() > 0).then_some(str)
}

fn get_client() -> Client<OpenAIConfig> {
    let config = OpenAIConfig::new()
        .with_api_key(env!("OPENAI_API_KEY"))
        .with_org_id(env!("OPENAI_ORG_ID"));

    Client::with_config(config)
}

impl ToString for GptModel {
    fn to_string(&self) -> String {
        match self {
            Self::Gpt35Turbo => "gpt-3.5-turbo-0125",
            Self::Gpt4O => "gpt-4o",
            Self::GptO1 => "o1",
        }.to_string()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            prompt: Default::default(),
            image: Default::default(),
            model: Default::default(),
            summary_prompt: true,
            job: Default::default(),
            output_prompt: Default::default(),
        }
    }
}

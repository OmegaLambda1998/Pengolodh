use bpaf::{OptionParser, Parser, construct, long, positional};

// === Whisper ===

#[derive(Debug, Clone)]
pub enum WhisperTasks {
    Transcribe { in_file: String, out_file: String },
    Translate { in_file: String, out_file: String },
}

fn whisper_tasks() -> OptionParser<WhisperTasks> {
    let transcribe = whisper_transcribe()
        .to_options()
        .descr("Transcribe Audio File")
        .command("transcribe");

    let translate = whisper_translate()
        .to_options()
        .descr("Translate Audio File")
        .command("translate");

    construct!([transcribe, translate]).to_options()
}

fn whisper_transcribe() -> impl Parser<WhisperTasks> {
    let in_file = long("in")
        .short('i')
        .help("Path to audio file to transcribe")
        .argument("in_file");
    let out_file = long("out")
        .short('o')
        .help("Path to write the transcribed audio to")
        .argument("out_file");
    construct!(WhisperTasks::Transcribe { in_file, out_file })
}

fn whisper_translate() -> impl Parser<WhisperTasks> {
    let in_file = long("in")
        .short('i')
        .help("Path to audio file to translate")
        .argument("in_file");
    let out_file = long("out")
        .short('o')
        .help("Path to write the translated audio to. Defaults to `IN_FILE.txt`")
        .argument("out_file");
    construct!(WhisperTasks::Transcribe { in_file, out_file })
}

// === LLaMa ===

#[derive(Debug, Clone)]
pub enum LLaMaTasks {
    Query { query: String },
}

fn llama_tasks() -> OptionParser<LLaMaTasks> {
    let query = llama_query().to_options().descr("Query").command("query");

    construct!([query]).to_options()
}

fn llama_query() -> impl Parser<LLaMaTasks> {
    let query = positional("QUERY").help("Query to");
    construct!(LLaMaTasks::Query { query })
}

// === Pengolodh ===

#[derive(Debug, Clone)]
pub enum Options {
    WhisperOptions { command: WhisperTasks },
    LLaMaOptions { command: LLaMaTasks },
}

fn whisper_options() -> impl Parser<Options> {
    let command = whisper_tasks().descr("Whisper Tasks").command("whisper");
    construct!(Options::WhisperOptions { command })
}

fn llama_options() -> impl Parser<Options> {
    let command = llama_tasks().descr("LLaMa Tasks").command("llama");
    construct!(Options::LLaMaOptions { command })
}

pub fn options() -> OptionParser<Options> {
    let whisper = whisper_options();
    let llama = llama_options();

    construct!([whisper, llama]).to_options()
}

fn main() {
    let opts = options().run();
    match opts {
        Options::WhisperOptions { command } => whisper(command),
        Options::LLaMaOptions { command } => llama(command),
    }
}

fn whisper(cmd: WhisperTasks) {
    match cmd {
        WhisperTasks::Transcribe { in_file, out_file } => {
            println!("Transcribing {in_file:?} => {out_file:?}")
        }
        WhisperTasks::Translate { in_file, out_file } => {
            println!("Translating {in_file:?} => {out_file:?}")
        }
    }
}

fn llama(cmd: LLaMaTasks) {
    match cmd {
        LLaMaTasks::Query { query } => {
            println!("Querying: {query:?}")
        }
    }
}

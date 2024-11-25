mod subtitle_parser;

fn main() {
    let subtitle_parser = subtitle_parser::SubtitleParser::default();

    subtitle_parser.modify_ass_folder();
}
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Write};
use rand::Rng;
use std::fs;
use std::path::Path;
mod types;

pub struct SubtitleParser {
    input_folder: String,
    output_folder: String
}

impl SubtitleParser {
    pub fn default() -> Self {
        return Self {
            input_folder: String::from("subtitles_input"),
            output_folder: String::from("subtitles_output")
        };
    }

    fn create_reader(&self, file_path: &str) -> BufReader<File> {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        return reader
    }
    
    pub fn  parse_srt(&self, file_path: &str) {
        let reader = self.create_reader(file_path);

        for line in reader.lines() {
            match line {
                Err(err) => {panic!("{}", err)},
                Ok(line_text) => {}
            }
        }
    }

    fn create_output_file(&self, file_path: &str) -> fs::File {
        let path = Path::new(file_path);
        let name = path.file_stem().unwrap_or_default();
        let extension = path.extension().unwrap_or_default().to_str().unwrap();
        let postfix = "_modified";
        let output_path = format!("{}{}.{}", Path::new(&self.output_folder).join(name).to_str().unwrap(), postfix, extension);
        let mut output_file = fs::File::create(output_path).unwrap();
        return output_file;
    }

    pub fn modify_ass_folder(&self) {
        let files = fs::read_dir(&self.input_folder).unwrap();
        for file in files {
            let file_path = file.unwrap().path();
            self.modify_ass(file_path.to_str().unwrap());
        }
    }

    pub fn  modify_ass(&self, file_path: &str) {
        //Creating a changed file in the output folder
        let mut output_file = self.create_output_file(file_path);

        let reader = self.create_reader(file_path);

        for line in reader.lines() {

            match line {
                Err(err) => {panic!("{}", err)},
                Ok(line_text) => {
                    //If the current line is not dialogue => skip parsing it
                    if (line_text.chars().take(8).collect::<String>() != "Dialogue") {
                        writeln!(output_file, "{}", line_text).unwrap();
                        continue;
                    }
                    
                    //Finding the starting index of the actual text, it starts after we meet ",," twice
                    let text_start_index = self.get_text_start_index_ass(&line_text);
                    //Converting line_text to Vec to permit modifying chars by index
                    let mut line_text_vec: Vec<char> = line_text.chars().collect();

                    let mut curr_word_len: u16 = 0;
                    let mut curr_word_start_index: u16 = 0;
                    let mut ignore_chars: bool = false;
                    //Starting and ending indexes of words that can potentially be hidden
                    let mut word_indexes: Vec<types::WordIndexes> = Vec::new();
                    let mut index: usize = text_start_index;

                    while index <= (line_text_vec.len() - 1) {
                        let ch = line_text_vec[index];

                        //If we are inside the styling comment located inside "{}" brackets, wait for it to end
                        if ignore_chars == true {
                            match ch {
                                '}' => {
                                    ignore_chars = false;
                                },
                                _ => {}
                            }

                            index += 1;
                            continue;
                        } 

                        //If we are not inside style comment located inside "{}" brackets
                        match ch {
                            '{' => {
                                //We are inside of a styling comment
                                ignore_chars = true;
                            },
                            ' ' => {
                                if curr_word_len != 0 {                         
                                    word_indexes.push(types::WordIndexes {
                                        start: curr_word_start_index, 
                                        end: curr_word_start_index + curr_word_len
                                    });
                                    curr_word_len = 0;     
                                }                           
                            },
                            '\\' => {
                                if line_text_vec[index + 1] == 'N' {
                                    if curr_word_len != 0 { 
                                        word_indexes.push(types::WordIndexes {
                                            start: curr_word_start_index, 
                                            end: curr_word_start_index + curr_word_len
                                        });
                                        curr_word_len = 0;
                                    }
                                    index += 2;
                                    continue;                       
                                }
                            },
                            '!' | '?' | ',' | ':' | ';' | '\'' | '\"' | '.' | '…' => {
                                if curr_word_len != 0 {
                                    word_indexes.push(types::WordIndexes {
                                        start: curr_word_start_index, 
                                        end: curr_word_start_index + curr_word_len
                                    });
                                    curr_word_len = 0;
                                }
                            },
                            '-' => {
                                if curr_word_len != 0 {curr_word_len += 1}                                
                            },
                            _ => {
                                //If we meet a normal character
                                if curr_word_len == 0 {curr_word_start_index = index as u16};
                                curr_word_len += 1;
                                if index == (line_text_vec.len() - 1) {
                                    word_indexes.push(types::WordIndexes {
                                        start: curr_word_start_index, 
                                        end: curr_word_start_index + curr_word_len
                                    });
                                }
                            }
                        }
                        
                        index += 1;
                    }
                    //Choosing a random word to hide from the line
                    let mut rng = rand::thread_rng();
                    let chosen_word = &word_indexes[rng.gen_range(0..word_indexes.len())];
                    
                    for i in chosen_word.start..chosen_word.end {
                        line_text_vec[i as usize] = '_';    
                    };
                    let new_line = line_text_vec.iter().collect::<String>();
                    writeln!(output_file, "{}", new_line).unwrap();
                }
            }
        }
    }

    fn get_text_start_index_ass(&self, line: &str) -> usize {
        let line_vec : Vec<char> = line.chars().collect();
        let mut separator_counter = 0;
        let mut text_buffer: String = String::new();
        let mut text_start_index: usize = 0;
        let mut index = line_vec.len() - 1;

        for ch in line_vec.iter().rev() {
            if *ch == ',' {
                text_buffer.push(*ch);
                if text_buffer == ",," {
                    text_buffer = String::new();
                    text_start_index = index + 1;
                    break;
                }
            } else {
                text_buffer = String::new();
            }
            index -= 1;
        }

        return text_start_index
    }


}
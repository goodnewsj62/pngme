use crate::args::{DecodeArgs, EncodeArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use std::fs::{ read, File};
use std::io::Write;
use std::path::Path;
use std::process;
use std::str::FromStr;

pub fn encode(args: EncodeArgs){
    let png =  from_file(&args.filepath);

    if let Some(png) =png{
        let chunk_type =  ChunkType::from_str(&args.chunk_type);
        if let Err(val) =  chunk_type {
            println!("{}",  val);
            process::exit(1);
        }

        let chunk =  Chunk::new(chunk_type.unwrap(), args.message.bytes().collect::<Vec<u8>>());
        
        let mut png =  png;
        png.append_chunk(chunk);

        encode_data_to_file(png, &args.output_file.unwrap_or_else(||format!("{}",  &args.filepath)));
    }

}


pub fn decode(args:DecodeArgs){
    let png =  from_file(&args.filepath);

    if let Some(png) =png{
        let chunk  =  match png.chunk_by_type(&args.chunk_type) {
            Some(chunk_t) => chunk_t,
            None =>{
                println!("couldn't find chunk with chunk type {}",  &args.chunk_type);
                process::exit(1);
            }
        };

        println!("content of chunk-type {} : {}", &args.chunk_type , chunk.data_as_string().expect("could not decode message to string"));
    }
}

pub fn display_content(file_path:  &str){
    let png =  from_file(&file_path);

    if let Some(png) =png{
        println!("{}", png.to_string());
    }
}

pub fn remove(filepath:&str, chunk_type:&str ){
    let png =  from_file(&filepath);

    if let  Some(png) = png{
        let mut png =  png;
        match png.remove_first_chunk(&chunk_type) {
            Ok(chunk_t) => chunk_t,
            Err(_) =>{
                println!("couldn't find chunk with chunk type {}",  &chunk_type);
                process::exit(1);
            }
        };

        println!("chunk with chunk-type {} has been removed", &chunk_type );
    }
}


fn from_file<T:AsRef<Path>>( path:T ) -> Option<Png>{
    let file =  read(&path);
    match file {
        Ok(bytes) =>{
            let png =  Png::try_from(bytes.as_ref());

            match png {
                Ok(png) => Some(png),
                Err(_) =>{
                    println!("");
                    return  None;
                }
            }
        }
        
        Err(err) =>{
            let path_str: &str =  path.as_ref().to_str().expect("not a valid string");
            println!("Error {} reading {}",err ,path_str);
            return None;
        }
    }
}

fn encode_data_to_file<T:AsRef<Path>>(png:Png,  output:T ){
    let display =  output.as_ref().display();
    let mut file =  match File::create(&output) {
        Ok(file) => file,
        Err(reason)=>{
            println!("could not open output file {} because {}",  display, reason);
            process::exit(1)
        }
    };

    match file.write_all(&png.as_bytes()) {
        Ok(_) => {
            println!("encoding the data was successful");
        }
        Err(reason)=>{
            println!("could not write to  output file {} because {}",  display, reason);
            process::exit(1)
        }
    }
}


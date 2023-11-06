use std::{error::Error, process::Command};
use stringvideo::process_image;
fn main() -> Result<(), Box<dyn Error>> {

    let nombre = "pikachu2";
    let extension = "jpg";
    let carpeta = "c:/Users/tomas/Desktop/stringart/stringvideo/imagess/";
    let mut n: u32 = 0;
    let input = format!("{}.{}",nombre,extension);
    let img = image::open(&input)?;
    
    process_image(img,2000,256,carpeta,extension,nombre,&mut n);
    let comando = format!("C:/Users/tomas/AppData/Local/Programs/Python/Python310/python.exe c:/Users/tomas/Desktop/stringart/stringvideo/hacervideo.py {} {} {} {}",carpeta,nombre,extension,n);
    println!("{comando}");
   
    Command::new("cmd")
            .args(["/C", &comando])
            .output()
            .expect("failed to execute process");
  
    Ok(())
}


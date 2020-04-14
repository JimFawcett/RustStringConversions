/////////////////////////////////////////////////////////////
// rust_strings - demonstrate conversions                  //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 11 Apr 2020  //
/////////////////////////////////////////////////////////////
/*
   Rust strings, String and str, are utf-8 encoded
   std::path::PathBuf may have non utf-8 characters
     - that means we often have to convert
   std::ffi::OsString also may have non utf-8 characters
     - so more conversions

   This demo provides ways to convert between these types
*/
#[allow(unused_imports)]
use std::path::{Path, PathBuf};
#[allow(unused_imports)]
use std::ffi::{OsStr, OsString};
use std::env;
use std::io;
#[allow(unused_imports)]
use std::convert;
use std::borrow::Cow;

#[allow(dead_code)]
pub fn path_buf_to_string(pb:&PathBuf) -> String {
    // pb may have non utf-8 characters
    let cd_cow:Cow<'_, str> = pb.to_string_lossy();  
    // Cow<'_, str> is a clone on write string
    // https://deterministic.space/secret-life-of-cows.html
    // replaces non utf-8 chars with sentinal
    cd_cow.to_string()
}

#[allow(dead_code)]
pub fn string_to_path_buf(st:&str) -> PathBuf {
    let mut path_buf:PathBuf = PathBuf::new();
    path_buf.push(st);
    path_buf
}

#[allow(dead_code)]
pub fn path_buf_to_os_string(pb:&PathBuf) -> OsString {
    pb.clone().into_os_string()
}

#[allow(dead_code)]
pub fn os_string_to_path_buf(oss:&OsString) -> PathBuf {
    let mut path_buf:PathBuf = PathBuf::new();
    path_buf.push(oss);
    path_buf
}

#[allow(dead_code)]
pub fn os_string_to_string(oss:&OsString) -> String {
    oss.to_string_lossy().to_string()
}

#[allow(dead_code)]
pub fn string_to_os_string(str:&str) -> OsString {
    let mut os_str:OsString = OsString::new();
    os_str.push(str);
    os_str
}

#[allow(dead_code)]
pub fn demo1() -> io::Result<()> {

    print!("\n  -- convert str to String and back to str --");
    /*-- str to String --*/
    let sl = "a string";
    print!("\n  sl:str = {:?}", sl);
    let st = sl.to_string();
    print!("\n  st:String = {:?}", &st);

    /*-- String to str --*/
    let sl2 = &st;
    print!("\n  sl2:str = {:?}", sl2);

    print!("\n\n  -- convert PathBuf to String and back to PathBuf --");
    /*-- PathBuf to String --*/
    let cd:PathBuf = env::current_dir()?; 
    print!("\n  cd:PathBuf = {:?}", &cd); 
    // may not be valid utf-8
    let cd_cow:Cow<'_, str> = cd.to_string_lossy();  
    // Cow<'_, str> is a clone on write string
    // https://deterministic.space/secret-life-of-cows.html
    // replaces non utf-8 chars with sentinal
    let cd_strg:String = cd_cow.to_string();
    print!("\n  cd_strg:ToString = {:?}", &cd_strg);

    /*-- String to PathBuf --*/
    let mut path_buf:PathBuf = PathBuf::new();
    path_buf.push(cd_strg);
    print!("\n  path_buf:PathBuf = {:?}", &path_buf);

    print!("\n\n  -- convert PathBuf to OsString and back to PathBuf --");
    /*-- PathBuf to OsString --*/
    let os_str:OsString = path_buf.into_os_string();
    print!("\n  os_str:OsString = {:?}", &os_str);

    /*-- OsString to PathBuf --*/
    let mut path_buf:PathBuf = PathBuf::new();
    path_buf.push(&os_str);
    print!("\n  path_buf:PathBuf = {:?}", path_buf);

    print!("\n\n  -- convert OsString to String and back to OsString --");
    /*-- OsString to String --*/
    let cd_strg:String = os_str.to_string_lossy().to_string();
    print!("\n  cd_strg:String = {:?}", &cd_strg);

    /*-- String to OsString --*/
    let mut os_str:OsString = OsString::new();
    os_str.push(cd_strg);
    print!("\n  os_str:OsString = {:?}", &os_str);
    
    Ok(())
}

#[allow(dead_code)]
pub fn demo2() -> io::Result<()> {

    print!("\n  -- convert str to String and back to str --");
    /*-- str to String --*/
    let sl = "a string";
    print!("\n  sl:str = {:?}", sl);
    let st = sl.to_string();
    print!("\n  st:String = {:?}", &st);

    /*-- String to str --*/
    let sl2 = &st;
    print!("\n  sl2:str = {:?}", sl2);

    print!("\n\n  -- convert PathBuf to String and back to PathBuf --");
    /*-- PathBuf to String --*/
    let cd:PathBuf = env::current_dir()?; 
    print!("\n  cd:PathBuf = {:?}", &cd); 
    let str:String = path_buf_to_string(&cd);
    print!("\n  str:String = {:?}", &str); 
    
    /*-- String to PathBuf --*/
    let pb:PathBuf = string_to_path_buf(&str);
    print!("\n  pb:PathBuf = {:?}", &pb);

    print!("\n\n  -- convert PathBuf to OsString and back to PathBuf --");
    /*-- PathBuf to OsString --*/
    let os_str:OsString = path_buf_to_os_string(&pb);
    print!("\n  os_str:OsString = {:?}", &os_str);

    /*-- OsString to PathBuf --*/
    let pb:PathBuf = os_string_to_path_buf(&os_str);
    print!("\n  pb:PathBuf = {:?}", &pb);

    print!("\n\n  -- convert OsString to String and back to OsString --");
    /*-- OsString to String --*/
    let str:String = os_string_to_string(&os_str);
    print!("\n  str:String = {:?}", &str);

    /*-- String to OsString --*/
    let os_str:OsString = string_to_os_string(&str);
    print!("\n  os_str:OsString = {:?}", &os_str);

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn convert_pathbuf_to_string() {
        let mut pb:PathBuf = PathBuf::new();
        pb.push("a PathBuf");
        let s:String = path_buf_to_string(&pb);
        assert_eq!(s, "a PathBuf");
    }
    #[test]
    fn convert_string_to_pathbuf() {
        let s:String = "test2 string".to_string();
        let pb1:PathBuf = string_to_path_buf(&s);
        let mut pb2:PathBuf = PathBuf::new();
        pb2.push("test2 string");
        assert_eq!(pb1, pb2);
    }
    #[test]
    fn convert_pathbuf_to_osstring() {
        let mut pb:PathBuf = PathBuf::new();
        pb.push("test3 string");
        let oss1:OsString = path_buf_to_os_string(&pb);
        let mut oss2:OsString = OsString::new();
        oss2.push("test3 string");
        assert_eq!(oss1, oss2);
    }
    #[test]
    fn convert_osstring_to_pathbuf() {
        let mut oss:OsString = OsString::new();
        oss.push("test4 string");
        let pb1:PathBuf = os_string_to_path_buf(&oss);
        let mut pb2:PathBuf = PathBuf::new();
        pb2.push("test4 string");
        assert_eq!(pb1, pb2);
    }
    #[test]
    fn convert_osstring_to_string() {
        let mut oss:OsString = OsString::new();
        oss.push("test5 string");
        let str:String = os_string_to_string(&oss);
        assert_eq!(str, "test5 string".to_string());
    }
    #[test]
    fn convert_string_to_osstring() {
        let str:String = String::from("test6 string");
        let oss1:OsString = string_to_os_string(&str);
        let mut oss2:OsString = OsString::new();
        oss2.push("test6 string");
        assert_eq!(oss1, oss2);
    }
}

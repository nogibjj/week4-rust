use std::fs::File;
use std::io::{self, Read};

fn main(){
    let s = readfilename().unwrap_or_else(|err| panic!("IO error: {:?}", err));
    let s2=readstring().unwrap_or_else(|err|panic!("IO error:{:?}",err));
	println!("{} {}", s,s2);

}

fn readfilename() -> Result<String, io::Error> {
    // 打开文件，f是`Result<文件句柄,io::Error>`
    let f = File::open("hello.txt");

    let mut f = match f {
        // 打开文件成功，将file句柄赋值给f
        Ok(file) => file,
        // 打开文件失败，将错误返回(向上传播)
        Err(e) => return Err(e),
    };
    // 创建动态字符串s
    let mut s = String::new();
    // 从f文件句柄读取数据并写入s中
    match f.read_to_string(&mut s) {
        // 读取成功，返回Ok封装的字符串
        Ok(_) => Ok(s),
        // 将错误向上传播
        Err(e) => Err(e),
    }
}

fn readstring()->Result<String,io::Error>{
    let mut s=String::new();
    File::open("test.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

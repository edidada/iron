// https://kaisery.github.io/trpl-zh-cn/ch09-00-error-handling.html

#[cfg(test)]
mod tests {
    use std::fs::File;

    #[test]
    fn it_works() {
        panic!("crash and burn");
    }

//    [profile.release]
//    panic = 'abort'

    #[test]
    fn cause_panic(){
        let v = vec![1, 2, 3];

        v[99];
    }

    #[test]
    fn dafsdfa(){
        use std::fs::File;
        let f = File::open("hessssllo.txt");

//        let f: u32 = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem opening the file: {:?}", error)
            },
        };
    }


    #[test]
    fn adaf(){
        use std::fs::File;
        use std::io::ErrorKind;

        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            },
        };
    }

//    失败时 panic 的简写：unwrap 和 expect
    #[test]
    fn adsafasd(){
        let f = File::open("hello.txt").unwrap();
    }

    #[test]
    fn sdfasdf(){
        let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }


    #![allow(unused_variables)]
    fn mainssss() {
        use std::io;
        use std::io::Read;
        use std::fs::File;

        #[test]
        fn read_username_from_file() -> Result<String, io::Error> {
            let f = File::open("hello.txt");

            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s = String::new();

            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }
    }



    #![allow(unused_variables)]
    fn mdfsdain() {
        use std::io;
        use std::io::Read;
        use std::fs::File;

        fn read_username_from_file() -> Result<String, io::Error> {
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }
    }



    #![allow(unused_variables)]
    fn maisdn() {
        use std::io;
        use std::io::Read;
        use std::fs::File;

        fn read_username_from_file() -> Result<String, io::Error> {
            let mut s = String::new();

            File::open("hello.txt")?.read_to_string(&mut s)?;

            Ok(s)
        }
    }


}

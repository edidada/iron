#[cfg(test)]
mod tests {
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    struct Point<T> {
        x: T,
        y: T,
    }

    struct Points<T, U> {
        x: T,
        y: U,
    }


    #[test]
    fn mainss() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);
        assert_eq!(result, 100);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
        assert_eq!(result, 'y');

        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };

//        let wont_work = Point { x: 5, y: 4.0 }; //error[E0308]: mismatched types^^^ expected integer, found floating-point number

        let both_integer = Points { x: 5, y: 10 };
        let both_float = Points { x: 1.0, y: 4.0 };
        let integer_and_float = Points { x: 5, y: 4.0 };
    }

    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }


    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }


//    impl Pointsd<f32> {
//        fn distance_from_origin(&self) -> f32 {
//            (self.x.powi(2) + self.y.powi(2)).sqrt()
//        }
//    }

    #[test]
    fn maisdassn() {
        let p = Point { x: 5, y: 10 };

        println!("p.x = {}", p.x());
    }
}


pub trait Summary {
    fn summarize(&self) -> String;
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn mainss(){
    let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

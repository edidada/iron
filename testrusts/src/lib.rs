#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

//    fn largest<T>(list: &[T]) -> T {
//        let mut largest = list[0];
//
//        for &item in list.iter() {
//            if item > largest {
//                largest = item;
//            }
//        }
//
//        largest
//    }
//
//    #[test]
//    fn it_workss() {
//        let number_list = vec![34, 50, 25, 100, 65];
//
//        let result = largest(&number_list);
//        println!("The largest number is {}", result);
//
//        let char_list = vec!['y', 'm', 'a', 'q'];
//
//        let result = largest(&char_list);
//        println!("The largest char is {}", result);
//    }

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

    #[test]
    fn test_trait(){
        let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }
}

fn main() {
//    iter_types();
    produce_iterators();
}

//consuming adaptors


fn iter_types() {
    let mut text = vec!["some", "apple", "table"];
    //    let mut i = 0;

    //    while i < text.len() {
    //        println!("i: {}, item: {}", i, text[i]);
    //        i += 1;
    //    }

    // iter use ref | into_iter take ownership | iter_mut use mut ref
    for item in text.iter() {
        println!("item: {}", item);
    }

    for item in text.iter_mut() {
        println!("item: {}", item)
    }

    for item in text.into_iter() {
        println!("item: {}", item)
    }
}

fn produce_iterators() {
    let a: Vec<i32> = vec![1, 2, 3];
    let x = a.iter()
        .map(|x| x + 1)
        .filter(|x| *x > 2);

    let y = x.collect::<Vec<i32>>();
    println!("result {y:?}");
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_sum() {
        let a = vec![1, 2, 3,];
        let a_iter = a.iter();

        let total: i32 = a_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 44,
                style: "sneakers".to_string(),
            },
            Shoe {
                size: 38,
                style: "sandal".to_string(),
            },
            Shoe {
                size: 44,
                style: "boot".to_string(),
            }
        ];

        let my_size = shoes_in_size(shoes, 44);

        assert_eq!(
                my_size,
                vec![
                    Shoe {
                        size: 44,
                        style: "sneakers".to_string(),
                    },
                    Shoe {
                        size: 44,
                        style: "boot".to_string(),
                    },
                ]);
    }
}


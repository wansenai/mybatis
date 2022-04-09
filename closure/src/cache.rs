struct Cache<T> where T: Fn(u32) -> u32{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cache<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cache<T> {
        Cache {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random: u32) {
    let mut expensive_result = Cache::new(|num| {
        // check intensity
        assert_eq!(intensity, intensity);
        num
    });

    if intensity < 25 {
        if random == 3 {
            println!("{}", expensive_result.value(random));
        } else {
            println!("{}", expensive_result.value(intensity));
        }
    } else {
        println!("{}", expensive_result.value(intensity));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    
    #[test]
    fn test_cache() {
        let x_closure = |x| x;
        let s = x_closure(5);

        assert_eq!(5,s);

        let x: u32 = 12;
        let y: u32 = 3;
        generate_workout(x, y);

        let mut c = Cache::new(|a| a);
        let v1 = c.value(2);
        // 缓存为2 无论如何取都是2 
        let v2 = c.value(3);

        assert_ne!(v2, 3)
    }
}
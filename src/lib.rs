use std::collections::HashMap;


pub struct Cached<'a, Ti, To> where
Ti: std::hash::Hash + std::cmp::Eq + Copy,
To: Clone,
{
    capacity: usize,
    func: Box<dyn Fn(Ti) -> To + 'a>,
    cache: HashMap<Ti, (To, usize)>,
    priority: Vec<Ti>,
}


impl<'a, Ti, To> Cached<'a, Ti, To> where
Ti: std::hash::Hash + std::cmp::Eq + Copy,
To: Clone,
{
    pub fn new(capacity: usize, func: Box<dyn Fn(Ti) -> To + 'a>) -> Self {
        let cache = HashMap::with_capacity(capacity);
        let priority = Vec::with_capacity(capacity);
        Cached{ capacity, func, cache, priority }
    }

    pub fn call(&mut self, x: Ti) -> To {
        if self.cache.contains_key(&x) {
            let (val, idx) = self.cache.get(&x).unwrap().clone();

            if idx > 0  // Swap priority order with next item in list
            {
                let other = self.priority.get(idx - 1).unwrap().clone();
                self.priority.swap(idx, idx - 1);
                self.cache.entry(other).and_modify(|(_, idx)| *idx = *idx + 1);
                self.cache.entry(x).and_modify(|(_, idx)| *idx = *idx - 1);
            }

            val
        } else {
            let val = (self.func)(x);

            if self.priority.len() == self.capacity{
                let k = self.priority.pop().unwrap();
                self.cache.remove(&k);
            }

            let idx = self.priority.len();
            self.priority.push(x);
            self.cache.insert(x, (val.clone(), idx));

            val
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::{Arc, Mutex};

    #[test]
    fn it_works() {
        let func = |(a, b): (i64, i64)| a * 2 + b;
        let mut cached = Cached::new(24, Box::new(func));
        assert_eq!(2 + 2, 4);
        assert_eq!(cached.call((4, 4)), 12);
        assert_eq!(cached.call((4, 4)), 12);
    }

    #[test]
    fn test_with_side_effects() {
        let mutate_me: Arc<Mutex<i64>> = Arc::new(Mutex::new(12));
        let func = |(a, b): (i64, i64)| {let val = a * 3 - b; *mutate_me.lock().unwrap() = val; val};
        let mut cached = Cached::new(24, Box::new(func));
        assert_eq!(cached.call((4, 3)), 9);
        assert_eq!(*mutate_me.lock().unwrap(), 9);
        assert_eq!(cached.call((2, 2)), 4);
        assert_eq!(*mutate_me.lock().unwrap(), 4);
        assert_eq!(cached.call((4, 3)), 9);
        assert_eq!(*mutate_me.lock().unwrap(), 4);
    }
}

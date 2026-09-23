pub struct AverCollect {
    list: Vec<i32>,
    aver: f64,
}

impl AverCollect {
    pub fn new() -> AverCollect {
        AverCollect {
            list: vec![],
            aver: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn average(&self) -> f64 {
        self.aver
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(val) => {
                self.update_average();
                Some(val)
            }
            None => None,
        }
    }

    fn update_average(&mut self) {
        if self.list.is_empty() {
            self.aver = 0.0;
            return;
        }
        let total: i32 = self.list.iter().sum();
        self.aver = total as f64 / self.list.len() as f64;
    }
}

impl Default for AverCollect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_average() {
        let mut c = AverCollect::new();
        c.add(2);
        c.add(4);
        assert_eq!(c.average(), 3.0);
    }

    #[test]
    fn remove_updates_average() {
        let mut c = AverCollect::new();
        c.add(2);
        c.add(4);
        assert_eq!(c.remove(), Some(4));
        assert_eq!(c.average(), 2.0);
        assert_eq!(c.remove(), Some(2));
        assert_eq!(c.average(), 0.0);
        assert_eq!(c.remove(), None);
    }
}
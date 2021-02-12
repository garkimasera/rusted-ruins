/// if let alternative for iterator
macro_rules! if_first {
    {$p:pat = $e:expr; $b:block} => {
        let mut iter = std::iter::IntoIterator::into_iter($e);
        loop {
            if let Some(value) = iter.next() {
                match value {
                    $p => {
                        break $b;
                    }
                    _ => (),
                }
            } else {
                break;
            }
        }
    };
    {$p:pat = $e:expr; $b:block else $elsb:block} => {{
        let mut iter = std::iter::IntoIterator::into_iter($e);
        let result = loop {
            if let Some(value) = iter.next() {
                match value {
                    $p => {
                        break Some($b);
                    }
                    _ => (),
                }
            }
        };
        if let Some(result) = result {
            result
        } else {
            $elsb
        }
    }};
}

macro_rules! if_first {
    {$p:pat = $e:expr; $b:block} => {
        'search_loop: loop {
            for e in $e {
                match e {
                    $p => {
                        break 'search_loop $b;
                    }
                    _ => (),
                }
            }
            break 'search_loop ();
        }
    };
    {$p:pat = $e:expr; $b:block else $elsb:block} => {{
        let search_loop_result = 'search_loop: loop {
            for e in $e {
                match e {
                    $p => {
                        break 'search_loop Some($b);
                    }
                    _ => (),
                }
            }
            break None;
        };
        if let Some(result) = search_loop_result {
            result
        } else {
            $elsb
        }
    }};
}

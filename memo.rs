//    a.sort_by(|x, y| x.cmp(y).reverse());
//    a.sort_by_key(|&x| -x)
//    for (i, &x) in a.iter().enumerate() { // foreachのような。
//        &xの&を抜くと・・the trait bound `{integer}: std::ops::AddAssign<&i32>` is not satisfied
//        手元では通ったけど。
//    }

// let patterns: Vec<Vec<char>> = ["dream", "dreamer", "erase", "eraser"]
//        .iter()
//        .map(|s| s.chars().rev().collect())
//        .collect();
//    let s: Vec<char> = read::<String>().chars().rev().collect();
//    let mut s = &s[..]; // Vec<char>の参照をスライスして返す。参照は可変だが値はimmutablei,
//    sは上書き
//    let mut succeeded = true;
//    while s.len() > 0 {
//        let matched = patterns.iter().find(|&p| s.starts_with(p));
//        if let Some(p) = matched {
//            s = &s[p.len()..];
//        } else {
//            succeeded = false;
//            break;
//        }
//    }
//
//    let patterns: Vec<Vec<char>> = ["dream", "dreamer", "erase", "eraser"]
//        .iter()
//        .map(|s| s.chars().rev().collect())
//        .collect();
//    let s: Vec<char> = read::<String>().chars().rev().collect();
//    let mut s = &s[..];
//    let mut succeeded = true;
//    while s.len() > 0 {
//        let matched = patterns.iter().find(|&p| s.starts_with(p));
//        if let Some(p) = matched {
//            s = &s[p.len()..];
//        } else {
//            succeeded = false;
//            break;
//        }
//    }
//    println!("{}", if succeeded { "YES" } else { "NO" });
//
// let mut v: Vec<(i32, i32, i32)> = (0..n).map(|_| (read(), read(), read())).collect();
//     v.insert(0, (0, 0, 0));
//     let yes = v[..].windows(2).all(|w| {
//         let (t, x, y) = w[0];
//         let (nt, nx, ny) = w[1];
//         let time = nt - t;
//         let dist = (nx - x).abs() + (ny - y).abs();
//         dist <= time && time % 2 == dist % 2
//     });
//     println!("{}", if yes { "Yes" } else { "No" });

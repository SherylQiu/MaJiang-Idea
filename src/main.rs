use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

fn main() {
    let mut deck: Vec<String> = Vec::new();

    for i in 1..=9 {
        for _ in 0..4 {
            deck.push(format!("{}万", i));
            deck.push(format!("{}条", i));
            deck.push(format!("{}筒", i));
        }
    }

    for _ in 0..4 {
        deck.push("红中".to_string());
        deck.push("白板".to_string());
    }

    let mut rng = thread_rng();
    deck.shuffle(&mut rng);

    println!("麻将牌池已生成！");
    println!("总牌数: {}", deck.len());

    println!("\n按 Enter 开始抽14张牌...");
    wait_enter();

    let mut hand: Vec<String> = Vec::new();
    let mut played_area: Vec<String> = Vec::new();

    for _ in 0..14 {
        hand.push(deck.pop().unwrap());
    }

    sort_hand(&mut hand);

    let mut hand_limit = 14;
    let mut turn = 1;

    loop {
        println!("\n====================");
        println!("第 {} 回合", turn);
        println!("====================");

        println!("当前剩余牌数: {}", deck.len());
        println!("当前手牌上限: {}", hand_limit);

        println!("\n你的手牌：");
        show_hand(&hand);

        println!("\n出牌区：");
        show_played_area(&played_area);

        println!("\n你想进行什么操作？");
        println!("输入 1 = 弃1摸1（可指定花色）");
        println!("输入 3 = 弃3摸3（全随机）");
        println!("输入 4 = 放置牌");
        println!("输入 0 = 结束游戏");

        let choice = read_number();

        if choice == 0 {
            println!("\n游戏结束！");
            break;
        }

        if choice == 1 || choice == 3 {
            let draw_count = choice;

            if hand.len() < draw_count {
                println!("你的手牌不够丢。");
                continue;
            }

            println!("\n你需要先丢掉 {} 张牌。", draw_count);

            for _ in 0..draw_count {
                println!("\n请选择你要丢掉的牌编号：");
                show_hand(&hand);

                let index = read_number();

                if index == 0 || index > hand.len() {
                    println!("编号错误。");
                    continue;
                }

                let removed = hand.remove(index - 1);
                println!("你丢掉了: {}", removed);
            }

            if draw_count == 1 {
                println!("\n你想要什么花色？");
                println!("1 = 万");
                println!("2 = 条");
                println!("3 = 筒");
                println!("其他数字 = 随机");

                let suit_choice = read_number();

                let wanted_suit = match suit_choice {
                    1 => "万",
                    2 => "条",
                    3 => "筒",
                    _ => "",
                };

                if wanted_suit != "" {
                    draw_by_suit(&mut deck, &mut hand, wanted_suit);
                } else {
                    draw_random(&mut deck, &mut hand);
                }
            } else {
                println!("\n开始随机抽3张牌...");

                for _ in 0..3 {
                    draw_random(&mut deck, &mut hand);
                }
            }

            sort_hand(&mut hand);

            println!("\n本回合结束，你现在的手牌：");
            show_hand(&hand);

            turn += 1;
        } else if choice == 4 {
            println!("\n你想放置几张牌？");
            println!("输入 2 = 放置两对");
            println!("输入 3 = 放置顺子或三张一样");

            let place_count = read_number();

            if place_count != 2 && place_count != 3 {
                println!("只能放置 2 张或 3 张。");
                continue;
            }

            if hand.len() < place_count {
                println!("你的手牌不够。");
                continue;
            }

            println!("\n请选择你要放置的牌编号：");
            show_hand(&hand);

            let mut selected_indices: Vec<usize> = Vec::new();

            while selected_indices.len() < place_count {
                println!("输入第 {} 张牌的编号：", selected_indices.len() + 1);

                let index = read_number();

                if index == 0 || index > hand.len() {
                    println!("编号错误。");
                    continue;
                }

                let real_index = index - 1;

                if selected_indices.contains(&real_index) {
                    println!("这张牌已经选过了。");
                    continue;
                }

                selected_indices.push(real_index);
            }

            let selected_tiles: Vec<String> = selected_indices
                .iter()
                .map(|&i| hand[i].clone())
                .collect();

            let valid = if place_count == 2 {
                is_pair(&selected_tiles)
            } else {
                is_triplet(&selected_tiles) || is_sequence(&selected_tiles)
            };

            if valid {
                println!("\n放置成功！");

                selected_indices.sort_by(|a, b| b.cmp(a));

                for index in selected_indices {
                    let tile = hand.remove(index);
                    played_area.push(tile);
                }

                hand_limit -= place_count;

                println!("新的手牌上限: {}", hand_limit);
            } else {
                println!("\n放置失败，这些牌不符合规则，已经返回手牌。");
            }

            sort_hand(&mut hand);

            println!("\n当前手牌：");
            show_hand(&hand);

            println!("\n当前出牌区：");
            show_played_area(&played_area);

            turn += 1;
        } else {
            println!("请输入 1、3、4 或 0。");
        }

        if hand.is_empty() {

            println!("\nYou Win!");
        
            break;
        }
    }
}

fn wait_enter() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn read_number() -> usize {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    input
        .trim()
        .parse::<usize>()
        .unwrap_or(999)
}

fn show_hand(hand: &Vec<String>) {
    for (i, tile) in hand.iter().enumerate() {
        println!("{}: {}", i + 1, tile);
    }
}

fn show_played_area(played_area: &Vec<String>) {
    if played_area.is_empty() {
        println!("目前为空");
    } else {
        for tile in played_area {
            println!("{}", tile);
        }
    }
}

fn draw_random(deck: &mut Vec<String>, hand: &mut Vec<String>) {
    if let Some(tile) = deck.pop() {
        println!("你抽到了: {}", tile);
        hand.push(tile);
    } else {
        println!("牌池已经空了！");
    }
}

fn draw_by_suit(deck: &mut Vec<String>, hand: &mut Vec<String>, suit: &str) {
    let position = deck
        .iter()
        .position(|tile| tile.contains(suit));

    match position {
        Some(index) => {
            let tile = deck.remove(index);
            println!("你指定花色抽到了: {}", tile);
            hand.push(tile);
        }

        None => {
            println!("这个花色已经没有牌了，改为随机抽牌。");
            draw_random(deck, hand);
        }
    }
}

fn is_pair(tiles: &Vec<String>) -> bool {
    tiles.len() == 2 && tiles[0] == tiles[1]
}

fn is_triplet(tiles: &Vec<String>) -> bool {
    tiles.len() == 3 && tiles[0] == tiles[1] && tiles[1] == tiles[2]
}

fn is_sequence(tiles: &Vec<String>) -> bool {
    if tiles.len() != 3 {
        return false;
    }

    let mut parsed_tiles: Vec<(i32, String)> = Vec::new();

    for tile in tiles {
        if tile.contains("万") {
            let num = tile.chars().next().unwrap().to_digit(10).unwrap() as i32;
            parsed_tiles.push((num, "万".to_string()));
        } else if tile.contains("条") {
            let num = tile.chars().next().unwrap().to_digit(10).unwrap() as i32;
            parsed_tiles.push((num, "条".to_string()));
        } else if tile.contains("筒") {
            let num = tile.chars().next().unwrap().to_digit(10).unwrap() as i32;
            parsed_tiles.push((num, "筒".to_string()));
        } else {
            return false;
        }
    }

    if parsed_tiles[0].1 != parsed_tiles[1].1 || parsed_tiles[1].1 != parsed_tiles[2].1 {
        return false;
    }

    parsed_tiles.sort_by_key(|tile| tile.0);

    parsed_tiles[0].0 + 1 == parsed_tiles[1].0
        && parsed_tiles[1].0 + 1 == parsed_tiles[2].0
}

fn sort_hand(hand: &mut Vec<String>) {
    hand.sort_by_key(|tile| tile_value(tile));
}

fn tile_value(tile: &String) -> i32 {
    if tile.contains("万") {
        let num = tile.chars().next().unwrap().to_digit(10).unwrap() as i32;
        return 100 + num;
    }

    if tile.contains("条") {
        let num = tile.chars().next().unwrap().to_digit(10).unwrap() as i32;
        return 200 + num;
    }

    if tile.contains("筒") {
        let num = tile.chars().next().unwrap().to_digit(10).unwrap() as i32;
        return 300 + num;
    }

    if tile == "红中" {
        return 400;
    }

    if tile == "白板" {
        return 500;
    }

    999
}
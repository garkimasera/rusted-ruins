use super::gen::gen_item_from_id;
use crate::game::extrait::*;
use common::gamedata::*;
use common::item_selector::ItemSelector;
use num_rational::Ratio;

pub fn append_to_converter(container_item: &mut Item, item: Item, n: u32) {
    let products =
        find_attr!(item.obj(), ItemObjAttr::ConvertableByContainer { products, .. } => products)
            .unwrap();

    let container_item_list = if let Some(ItemAttr::Container(container)) =
        find_attr_mut!(container_item, ItemAttr::Container)
    {
        &mut container.item_list
    } else {
        return;
    };

    for product in products {
        let product_item = gen_item_from_id(&product.0, 1);

        container_item_list.append_simple(product_item, n * product.1);
    }

    game_log!("container-convert-item"; item=item, n=n, container=container_item);
}

pub fn append_to_mixed_converter(container_item: &mut Item, item: Item, n: u32) {
    let function =
        find_attr!(container_item.obj(), ItemObjAttr::Container { function, .. } => function)
            .unwrap();

    if let ContainerFunction::ConvertMixed {
        duration,
        product,
        product_multiplier,
        ingredients,
    } = function
    {
        let container_item_list = if let Some(ItemAttr::Container(container)) =
            find_attr_mut!(container_item, ItemAttr::Container)
        {
            &mut container.item_list
        } else {
            return;
        };

        if duration.as_secs() > 0 {
            // Caluculate average remaining duration
            let sum_items = container_item_list
                .items
                .iter()
                .map(|item| item.1)
                .sum::<u32>() as u64;
            let remaining = if let Some(time) = container_item.time {
                time.remaining.as_secs()
            } else {
                duration.as_secs()
            };

            let new_remaining =
                (remaining * sum_items + duration.as_secs() * n as u64) / (sum_items + n as u64);
            container_item.reset_time(Duration::from_seconds(new_remaining));
        }

        let container_item_list = if let Some(ItemAttr::Container(container)) =
            find_attr_mut!(container_item, ItemAttr::Container)
        {
            &mut container.item_list
        } else {
            return;
        };

        container_item_list.append_simple(item, n);

        if calc_n_product(&container_item_list.items, ingredients) == 0 {
            container_item.time = None;
        } else if container_item.time.is_none() {
            do_mixed_convert(
                container_item_list,
                product,
                *product_multiplier,
                ingredients,
            );
        }
    }
}

pub fn do_mixed_convert(
    item_list: &mut ItemList,
    product: &str,
    product_multiplier: u32,
    ingredients: &[(ItemSelector, u32)],
) {
    let n_product = calc_n_product(&item_list.items, ingredients);

    if n_product == 0 {
        return;
    }

    let product = gen_item_from_id(product, 1);
    item_list.append_simple(product, n_product * product_multiplier);

    item_list.retain(
        |item, n| {
            for ingredient in ingredients {
                if ingredient.0.is(item.obj()) {
                    return 0;
                }
            }
            n
        },
        true,
    );
}

fn calc_n_product(items: &[(Item, u32)], ingredients: &[(ItemSelector, u32)]) -> u32 {
    let mut sum = Ratio::new(0u32, 1);

    for (item, n) in items {
        for ingredient in ingredients {
            if ingredient.0.is(item.obj()) {
                sum += Ratio::new(*n, ingredient.1);
                break;
            }
        }
    }

    sum.to_integer()
}

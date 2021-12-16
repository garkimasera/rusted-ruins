//! Quest handlings

use super::item::gen::choose_item_by_item_selector;
use common::gamedata::*;
use common::gobj;
use common::obj::SiteGenObject;
use common::objholder::ItemIdx;
use common::sitegen::QuestGenData;
use fnv::FnvHashMap;
use rules::RULES;
use std::collections::HashSet;

/// Update quest list of current town
pub fn update_town_quest(gd: &mut GameData) {
    let mid = gd.get_current_mapid();
    let sid = mid.sid();
    let site = gd.region.get_site_mut(mid.sid());
    let sg: &SiteGenObject = gobj::get_by_id(site.id.as_ref().unwrap());
    let town = match gd.region.get_site_mut(mid.sid()).content {
        SiteContent::Town { ref mut town } => town,
        _ => unreachable!(),
    };

    let current_time = gd.time.current_time();
    let duration_from_last_update = current_time.duration_from(town.quests_last_update);
    if duration_from_last_update < Duration::from_days(RULES.quest.update_duration_days.into()) {
        trace!("No need to update quests for this town");
        return;
    }

    town.quests.clear();

    // TODO: the number of available quests is changed by the town economy scale.
    let n_quest = 5;
    let mut chosen_quests: HashSet<usize> = HashSet::default();
    let quest_gen_list: Vec<(usize, _)> = sg.quests.iter().enumerate().collect();

    for _ in 0..n_quest {
        if let Some((_, (i, qg))) = rng::choose(&quest_gen_list, |(i, q)| {
            let factor = if chosen_quests.contains(i) {
                RULES.quest.duplicate_factor
            } else {
                1.0
            };
            factor * q.weight()
        }) {
            chosen_quests.insert(*i);
            town.quests.push(create_quest(qg, sid));
        }
    }

    town.quests_last_update = current_time;
    trace!("Quest update for this town");
}

fn create_quest(qg: &QuestGenData, sid: SiteId) -> TownQuest {
    match qg {
        QuestGenData::ItemDelivering {
            text_id,
            deadline,
            reward,
            item,
            n,
            ..
        } => TownQuest {
            sid,
            text_id: text_id.clone(),
            deadline: Some(*deadline),
            reward: reward.clone(),
            kind: TownQuestKind::ItemDelivering {
                idx: choose_item_by_item_selector(item).unwrap_or_default(),
                n: *n,
            },
        },
    }
}

/// Returns available quests in the current town
pub fn available_quests(gd: &GameData) -> &[TownQuest] {
    let mid = gd.get_current_mapid();
    let town = match gd.region.get_site(mid.sid()).content {
        SiteContent::Town { ref town } => town,
        _ => unreachable!(),
    };

    town.quests.as_ref()
}

/// Returns the index of reportable quests in the current town
pub fn reportable_quests(gd: &GameData) -> Vec<u32> {
    let sid = gd.get_current_mapid().sid();

    gd.quest
        .town_quests
        .iter()
        .enumerate()
        .filter_map(|(i, (quest_state, quest))| {
            if *quest_state == TownQuestState::Reportable && quest.sid == sid {
                Some(i as u32)
            } else {
                None
            }
        })
        .collect()
}

/// Undertake quests in the current town
pub fn undertake_quests(gd: &mut GameData, mut targets: Vec<usize>) {
    let mid = gd.get_current_mapid();
    let town = match gd.region.get_site_mut(mid.sid()).content {
        SiteContent::Town { ref mut town } => town,
        _ => unreachable!(),
    };

    targets.sort_unstable();
    let mut quests = Vec::new();
    for (i, target) in targets.iter().enumerate() {
        quests.push(town.quests.remove(target - i));
    }
    for quest in quests.into_iter() {
        gd.quest.town_quests.push((TownQuestState::Active, quest));
    }
}

pub fn report_quests(gd: &mut GameData, mut targets: Vec<usize>) {
    targets.sort_unstable();
    let mut quests = Vec::new();
    for (i, target) in targets.iter().enumerate() {
        let (quest_state, quest) = gd.quest.town_quests.remove(target - i);
        assert_eq!(quest_state, TownQuestState::Reportable);
        quests.push(quest);
    }

    // for quest in quests.into_iter() {}
}

pub fn update_quest_status(_gd: &mut GameData) {}

pub fn update_delivery_chest(gd: &mut GameData, ill: ItemListLocation) {
    let sid = if let ItemListLocation::Container {
        ill:
            ItemListLocationExceptContainer::OnMap {
                mid: MapId::SiteMap { sid, .. },
                ..
            },
        ..
    } = ill
    {
        sid
    } else {
        warn!("Invalid located delivery chest");
        return;
    };

    let mut delivery_chest_content: Vec<(ItemIdx, u32)> = Vec::new();
    for (item, n) in gd.get_item_list(ill).iter() {
        if let Some((_, a)) = delivery_chest_content
            .iter_mut()
            .find(|(idx, _)| *idx == item.idx)
        {
            *a += *n;
        } else {
            delivery_chest_content.push((item.idx, *n));
        }
    }

    if let SiteContent::Town { town } = &mut gd.region.get_site_mut(sid).content {
        town.delivery_chest = Some(ill);
        town.delivery_chest_content = delivery_chest_content;
    }

    update_item_delivery_quest_status(gd, Some(sid));
}

pub fn update_item_delivery_quest_status(gd: &mut GameData, sid: Option<SiteId>) {
    let mut delivery_chest_contents: FnvHashMap<SiteId, Vec<(ItemIdx, u32)>> =
        FnvHashMap::default();

    for (state, quest) in gd.quest.town_quests.iter_mut() {
        if sid.is_some() && quest.sid != sid.unwrap() {
            continue;
        }
        let sid = quest.sid;

        let (idx, n) = if let TownQuestKind::ItemDelivering { idx, n } = quest.kind {
            (idx, n)
        } else {
            continue;
        };

        if delivery_chest_contents.get(&sid).is_none() {
            if let SiteContent::Town { town } = &gd.region.get_site(sid).content {
                delivery_chest_contents.insert(sid, town.delivery_chest_content.clone());
            } else {
                continue;
            };
        }

        let c = delivery_chest_contents.get_mut(&sid).unwrap();

        if let Some((_, n_content)) = c.iter_mut().find(|(idx_content, _)| *idx_content == idx) {
            if *n_content >= n {
                *n_content -= n;
                *state = TownQuestState::Reportable;
            } else {
                *state = TownQuestState::Active;
            }
        }
    }
}

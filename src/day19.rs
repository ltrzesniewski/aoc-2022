use crate::common::get_input_lines;
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

const ROBOT_TYPES: usize = 4;

#[derive(Copy, Clone)]
enum Res {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

struct Puzzle {
    blueprints: Vec<Blueprint>,
}

struct Blueprint {
    id: usize,
    ore_robot_cost_in_ore: usize,
    clay_robot_cost_in_ore: usize,
    obsidian_robot_cost_in_ore: usize,
    obsidian_robot_cost_in_clay: usize,
    geode_robot_cost_in_ore: usize,
    geode_robot_cost_in_obsidian: usize,
}

#[derive(Clone)]
struct Simulation<'a> {
    blueprint: &'a Blueprint,
    resources: [ResourceStatus; ROBOT_TYPES],
    next_robot: Res,
}

#[derive(Default, Copy, Clone)]
struct ResourceStatus {
    count: usize,
    robots: usize,
    pending_robots: usize,
}

#[allow(dead_code)]
pub fn run() {
    let puzzle = Puzzle::parse(get_input_lines());

    let result = part1(&puzzle);
    println!("Result (part 1): {result}");

    let result = part2(&puzzle);
    println!("Result (part 2): {result}");
}

fn part1(puzzle: &Puzzle) -> usize {
    puzzle
        .blueprints
        .iter()
        .map(|bp| bp.id * max_geodes(&bp, 24))
        .sum()
}

fn part2(puzzle: &Puzzle) -> usize {
    puzzle
        .blueprints
        .iter()
        .take(3)
        .map(|bp| max_geodes(&bp, 32))
        .fold(1, |a, b| a * b)
}

fn max_geodes(blueprint: &Blueprint, minutes: usize) -> usize {
    let mut simulations = vec![
        Simulation::new(&blueprint, Res::Ore),
        Simulation::new(&blueprint, Res::Clay),
    ];

    let mut new_simulations = vec![];

    for minute in 0..minutes {
        for simulation in simulations.iter_mut() {
            simulation.collect_resources();
            simulation.end_building_robots();

            if simulation.can_build_robot(simulation.next_robot) {
                simulation.start_building_robot(simulation.next_robot);
                simulation.branch(&mut new_simulations);
            }
        }

        simulations.append(&mut new_simulations);

        let remaining_minutes = minutes - minute - 1;

        let max_possible_score = simulations
            .iter()
            .map(|s| s.max_possible_geodes(remaining_minutes))
            .max()
            .unwrap();

        simulations.retain(|s| {
            // This should have worked without the scaling factor, I don't understand why it doesn't :'(
            s.max_possible_geodes(remaining_minutes) >= (max_possible_score as f64 * 0.8) as usize
        });
    }

    simulations.iter().map(|s| s.geode_count()).max().unwrap()
}

impl<'a> Simulation<'a> {
    fn new(blueprint: &'a Blueprint, next_robot: Res) -> Simulation<'a> {
        let mut result = Self {
            blueprint,
            resources: [ResourceStatus::default(); ROBOT_TYPES],
            next_robot,
        };

        result.resources[Res::Ore].robots = 1;
        result
    }

    fn branch(&mut self, simulations: &mut Vec<Simulation<'a>>) {
        if !self.has_enough_ore_robots() {
            let mut clone = self.clone();
            clone.next_robot = Res::Ore;
            simulations.push(clone);
        }

        if !self.has_enough_clay_robots() {
            let mut clone = self.clone();
            clone.next_robot = Res::Clay;
            simulations.push(clone);
        }

        if !self.has_enough_obsidian_robots() {
            let mut clone = self.clone();
            clone.next_robot = Res::Obsidian;
            simulations.push(clone);
        }

        self.next_robot = Res::Geode;
    }

    fn geode_count(&self) -> usize {
        self.resources[Res::Geode].count
    }

    fn max_possible_geodes(&self, minutes: usize) -> usize {
        self.resources[Res::Geode].max_possible_count(minutes)
    }

    fn start_building_robot(&mut self, kind: Res) {
        match kind {
            Res::Ore => self.start_building_ore_robot(),
            Res::Clay => self.start_building_clay_robot(),
            Res::Obsidian => self.start_building_obsidian_robot(),
            Res::Geode => self.start_building_geode_robot(),
        }
    }

    fn collect_resources(&mut self) {
        for res in self.resources.iter_mut() {
            res.collect();
        }
    }

    fn end_building_robots(&mut self) {
        for res in self.resources.iter_mut() {
            res.end_building_robots();
        }
    }

    fn can_build_robot(&self, kind: Res) -> bool {
        match kind {
            Res::Ore => self.can_build_ore_robot(),
            Res::Clay => self.can_build_clay_robot(),
            Res::Obsidian => self.can_build_obsidian_robot(),
            Res::Geode => self.can_build_geode_robot(),
        }
    }

    fn has_enough_ore_robots(&self) -> bool {
        self.resources[Res::Ore].robots
            >= self
                .blueprint
                .ore_robot_cost_in_ore
                .max(self.blueprint.clay_robot_cost_in_ore)
                .max(self.blueprint.obsidian_robot_cost_in_ore)
                .max(self.blueprint.geode_robot_cost_in_ore)
    }

    fn can_build_ore_robot(&self) -> bool {
        self.resources[Res::Ore].count >= self.blueprint.ore_robot_cost_in_ore
    }

    fn start_building_ore_robot(&mut self) {
        self.resources[Res::Ore].count -= self.blueprint.ore_robot_cost_in_ore;
        self.resources[Res::Ore].pending_robots += 1;
    }

    fn has_enough_clay_robots(&self) -> bool {
        self.resources[Res::Clay].robots >= self.blueprint.obsidian_robot_cost_in_clay
    }

    fn can_build_clay_robot(&self) -> bool {
        self.resources[Res::Ore].count >= self.blueprint.clay_robot_cost_in_ore
    }

    fn start_building_clay_robot(&mut self) {
        self.resources[Res::Ore].count -= self.blueprint.clay_robot_cost_in_ore;
        self.resources[Res::Clay].pending_robots += 1;
    }

    fn has_enough_obsidian_robots(&self) -> bool {
        self.resources[Res::Obsidian].robots >= self.blueprint.geode_robot_cost_in_obsidian
    }

    fn can_build_obsidian_robot(&self) -> bool {
        self.resources[Res::Ore].count >= self.blueprint.obsidian_robot_cost_in_ore
            && self.resources[Res::Clay].count >= self.blueprint.obsidian_robot_cost_in_clay
    }

    fn start_building_obsidian_robot(&mut self) {
        self.resources[Res::Ore].count -= self.blueprint.obsidian_robot_cost_in_ore;
        self.resources[Res::Clay].count -= self.blueprint.obsidian_robot_cost_in_clay;
        self.resources[Res::Obsidian].pending_robots += 1;
    }

    fn can_build_geode_robot(&self) -> bool {
        self.resources[Res::Ore].count >= self.blueprint.geode_robot_cost_in_ore
            && self.resources[Res::Obsidian].count >= self.blueprint.geode_robot_cost_in_obsidian
    }

    fn start_building_geode_robot(&mut self) {
        self.resources[Res::Ore].count -= self.blueprint.geode_robot_cost_in_ore;
        self.resources[Res::Obsidian].count -= self.blueprint.geode_robot_cost_in_obsidian;
        self.resources[Res::Geode].pending_robots += 1;
    }
}

impl ResourceStatus {
    fn collect(&mut self) {
        self.count += self.robots;
    }

    fn end_building_robots(&mut self) {
        self.robots += self.pending_robots;
        self.pending_robots = 0;
    }

    fn max_possible_count(&self, minutes: usize) -> usize {
        let mut result = self.count;
        let mut robots = self.robots;

        for _minute in 0..minutes {
            result += robots;
            robots += 1;
        }

        result
    }
}

impl Index<Res> for [ResourceStatus; ROBOT_TYPES] {
    type Output = ResourceStatus;

    fn index(&self, index: Res) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Res> for [ResourceStatus; ROBOT_TYPES] {
    fn index_mut(&mut self, index: Res) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl Puzzle {
    fn parse(lines: Vec<String>) -> Puzzle {
        let mut blueprints = vec![];

        for line in lines {
            if !line.is_empty() {
                blueprints.push(Blueprint::from_str(&line).unwrap());
            }
        }

        Self { blueprints }
    }
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Blueprint (?P<id>[0-9]+): Each ore robot costs (?P<ore_ore>[0-9]+) ore\. Each clay robot costs (?P<clay_ore>[0-9]+) ore\. Each obsidian robot costs (?P<obsidian_ore>[0-9]+) ore and (?P<obsidian_clay>[0-9]+) clay\. Each geode robot costs (?P<geode_ore>[0-9]+) ore and (?P<geode_obsidian>[0-9]+) obsidian\.").unwrap();
        }

        let cap = RE.captures(s).unwrap();

        Ok(Self {
            id: cap["id"].parse().unwrap(),
            ore_robot_cost_in_ore: cap["ore_ore"].parse().unwrap(),
            clay_robot_cost_in_ore: cap["clay_ore"].parse().unwrap(),
            obsidian_robot_cost_in_ore: cap["obsidian_ore"].parse().unwrap(),
            obsidian_robot_cost_in_clay: cap["obsidian_clay"].parse().unwrap(),
            geode_robot_cost_in_ore: cap["geode_ore"].parse().unwrap(),
            geode_robot_cost_in_obsidian: cap["geode_obsidian"].parse().unwrap(),
        })
    }
}

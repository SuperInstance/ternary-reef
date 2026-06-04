#![forbid(unsafe_code)]

//! Coral reef ecosystem pattern for long-lived collective intelligence.
//!
//! Models how a fleet develops persistent structures over time: slow-growing coral
//! frameworks, individual polyp agents, symbiotic energy relationships, spatial
//! zoning, and stress responses (bleaching events).

use std::collections::HashMap;

// ---- Core types ----

/// Ternary value for classification and health states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ternary {
    Negative,
    Neutral,
    Positive,
}

/// Unique identifier for a coral structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CoralId(pub u64);

/// Unique identifier for a polyp (individual agent).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PolypId(pub u64);

/// Unique identifier for a symbiodinium (energy provider).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbiontId(pub u64);

/// Unique zone identifier within a reef.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ZoneId(pub u64);

// ---- Polyp ----

/// Health state of a polyp.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolypHealth {
    Healthy,
    Stressed,
    Bleached,
    Dead,
}

/// An individual coral agent — the basic unit of the reef.
#[derive(Debug, Clone)]
pub struct Polyp {
    id: PolypId,
    coral_id: Option<CoralId>,
    health: PolypHealth,
    energy: u32,
    max_energy: u32,
    zone: Option<ZoneId>,
    symbiont: Option<SymbiontId>,
}

impl Polyp {
    pub fn new(id: PolypId) -> Self {
        Self {
            id,
            coral_id: None,
            health: PolypHealth::Healthy,
            energy: 50,
            max_energy: 100,
            zone: None,
            symbiont: None,
        }
    }

    pub fn id(&self) -> PolypId {
        self.id
    }

    pub fn health(&self) -> PolypHealth {
        self.health
    }

    pub fn energy(&self) -> u32 {
        self.energy
    }

    pub fn coral_id(&self) -> Option<CoralId> {
        self.coral_id
    }

    pub fn zone(&self) -> Option<ZoneId> {
        self.zone
    }

    pub fn symbiont(&self) -> Option<SymbiontId> {
        self.symbiont
    }

    /// Attach this polyp to a coral structure.
    pub fn attach_to_coral(&mut self, coral_id: CoralId) {
        self.coral_id = Some(coral_id);
    }

    /// Detach from coral.
    pub fn detach_from_coral(&mut self) {
        self.coral_id = None;
    }

    /// Place this polyp in a zone.
    pub fn place_in_zone(&mut self, zone: ZoneId) {
        self.zone = Some(zone);
    }

    /// Bind a symbiont to this polyp for energy supply.
    pub fn bind_symbiont(&mut self, symbiont: SymbiontId) {
        self.symbiont = Some(symbiont);
    }

    /// Remove the symbiont.
    pub fn remove_symbiont(&mut self) {
        self.symbiont = None;
    }

    /// Add energy (e.g., from symbiont). Capped at max.
    pub fn add_energy(&mut self, amount: u32) -> u32 {
        self.energy = (self.energy + amount).min(self.max_energy);
        self.energy
    }

    /// Consume energy for activities. Returns remaining energy or error.
    pub fn consume_energy(&mut self, amount: u32) -> Result<u32, &'static str> {
        if self.energy < amount {
            return Err("Insufficient energy");
        }
        self.energy -= amount;
        Ok(self.energy)
    }

    /// Apply stress to the polyp. May degrade health.
    pub fn apply_stress(&mut self, amount: u32) {
        if amount > 60 {
            self.health = PolypHealth::Bleached;
        } else if amount > 30 {
            self.health = PolypHealth::Stressed;
        }
    }

    /// Recover from stress/bleaching. Requires energy.
    pub fn recover(&mut self) -> Result<(), &'static str> {
        match self.health {
            PolypHealth::Bleached => {
                self.consume_energy(20)?;
                self.health = PolypHealth::Stressed;
                Ok(())
            }
            PolypHealth::Stressed => {
                self.consume_energy(10)?;
                self.health = PolypHealth::Healthy;
                Ok(())
            }
            PolypHealth::Dead => Err("Cannot recover from death"),
            PolypHealth::Healthy => Ok(()),
        }
    }

    /// Kill this polyp.
    pub fn kill(&mut self) {
        self.health = PolypHealth::Dead;
        self.energy = 0;
    }

    pub fn is_alive(&self) -> bool {
        !matches!(self.health, PolypHealth::Dead)
    }
}

// ---- Symbiodinium ----

/// An endosymbiont that provides energy to polyps.
#[derive(Debug, Clone)]
pub struct Symbiodinium {
    id: SymbiontId,
    energy_output: u32,
    active: bool,
    host_polyp: Option<PolypId>,
}

impl Symbiodinium {
    pub fn new(id: SymbiontId, energy_output: u32) -> Self {
        Self {
            id,
            energy_output,
            active: true,
            host_polyp: None,
        }
    }

    pub fn id(&self) -> SymbiontId {
        self.id
    }

    pub fn energy_output(&self) -> u32 {
        if self.active {
            self.energy_output
        } else {
            0
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn host(&self) -> Option<PolypId> {
        self.host_polyp
    }

    /// Bind this symbiont to a host polyp.
    pub fn bind(&mut self, polyp: PolypId) {
        self.host_polyp = Some(polyp);
    }

    /// Unbind from host.
    pub fn unbind(&mut self) {
        self.host_polyp = None;
    }

    /// Deactivate (e.g., expelled during bleaching).
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Reactivate.
    pub fn reactivate(&mut self) {
        self.active = true;
    }
}

// ---- Coral ----

/// Growth stage of a coral structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GrowthStage {
    /// Just settled, minimal structure.
    Seedling,
    /// Growing, small structure.
    Juvenile,
    /// Mature, full structure.
    Adult,
    /// Ancient, very resilient.
    Ancient,
}

/// A slow-growing persistent structure built by polyps.
#[derive(Debug, Clone)]
pub struct Coral {
    id: CoralId,
    stage: GrowthStage,
    growth_points: u32,
    growth_threshold: u32,
    polyps: Vec<PolypId>,
    zone: Option<ZoneId>,
    resilience: u32,
}

impl Coral {
    pub fn new(id: CoralId) -> Self {
        Self {
            id,
            stage: GrowthStage::Seedling,
            growth_points: 0,
            growth_threshold: 100,
            polyps: Vec::new(),
            zone: None,
            resilience: 10,
        }
    }

    pub fn id(&self) -> CoralId {
        self.id
    }

    pub fn stage(&self) -> GrowthStage {
        self.stage
    }

    pub fn growth_points(&self) -> u32 {
        self.growth_points
    }

    pub fn polyp_count(&self) -> usize {
        self.polyps.len()
    }

    pub fn polyps(&self) -> &[PolypId] {
        &self.polyps
    }

    pub fn zone(&self) -> Option<ZoneId> {
        self.zone
    }

    pub fn resilience(&self) -> u32 {
        self.resilience
    }

    /// Place this coral in a zone.
    pub fn place_in_zone(&mut self, zone: ZoneId) {
        self.zone = Some(zone);
    }

    /// Add a polyp to this coral structure.
    pub fn add_polyp(&mut self, polyp: PolypId) {
        if !self.polyps.contains(&polyp) {
            self.polyps.push(polyp);
        }
    }

    /// Remove a polyp from this coral.
    pub fn remove_polyp(&mut self, polyp: PolypId) {
        self.polyps.retain(|p| *p != polyp);
    }

    /// Add growth points. Returns true if growth stage advanced.
    pub fn grow(&mut self, points: u32) -> bool {
        self.growth_points += points;
        if self.growth_points >= self.growth_threshold {
            self.advance_stage()
        } else {
            false
        }
    }

    fn advance_stage(&mut self) -> bool {
        let advanced = match self.stage {
            GrowthStage::Seedling => {
                self.stage = GrowthStage::Juvenile;
                self.growth_threshold = 250;
                self.resilience = 25;
                true
            }
            GrowthStage::Juvenile => {
                self.stage = GrowthStage::Adult;
                self.growth_threshold = 600;
                self.resilience = 50;
                true
            }
            GrowthStage::Adult => {
                self.stage = GrowthStage::Ancient;
                self.resilience = 100;
                true
            }
            GrowthStage::Ancient => false,
        };
        if advanced {
            self.growth_points = 0;
        }
        advanced
    }

    /// Apply damage (reduce growth points, may kill polyps).
    pub fn take_damage(&mut self, damage: u32) {
        self.growth_points = self.growth_points.saturating_sub(damage);
    }

    /// Whether the coral is alive (has polyps or growth points).
    pub fn is_alive(&self) -> bool {
        !self.polyps.is_empty() || self.growth_points > 0
    }
}

// ---- ReefZone ----

/// A spatial partition of the reef, characterized by depth and light.
#[derive(Debug, Clone)]
pub struct ReefZone {
    id: ZoneId,
    name: String,
    depth: u32,
    light_level: u32, // 0-100
    corals: Vec<CoralId>,
    max_corals: usize,
}

impl ReefZone {
    pub fn new(id: ZoneId, name: &str, depth: u32, light_level: u32, max_corals: usize) -> Self {
        Self {
            id,
            name: name.to_string(),
            depth,
            light_level: light_level.min(100),
            corals: Vec::new(),
            max_corals,
        }
    }

    pub fn id(&self) -> ZoneId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn light_level(&self) -> u32 {
        self.light_level
    }

    pub fn corals(&self) -> &[CoralId] {
        &self.corals
    }

    /// Add a coral to this zone. Fails if full.
    pub fn add_coral(&mut self, coral: CoralId) -> Result<(), &'static str> {
        if self.corals.len() >= self.max_corals {
            return Err("Zone is full");
        }
        if self.corals.contains(&coral) {
            return Err("Coral already in zone");
        }
        self.corals.push(coral);
        Ok(())
    }

    pub fn remove_coral(&mut self, coral: CoralId) {
        self.corals.retain(|c| *c != coral);
    }

    pub fn coral_count(&self) -> usize {
        self.corals.len()
    }

    pub fn has_capacity(&self) -> bool {
        self.corals.len() < self.max_corals
    }

    /// Energy bonus based on light level.
    pub fn energy_bonus(&self) -> u32 {
        self.light_level / 10
    }
}

// ---- BleachingEvent ----

/// Severity of a bleaching event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BleachSeverity {
    Mild,
    Moderate,
    Severe,
}

/// Result of a bleaching event.
#[derive(Debug, Clone)]
pub struct BleachingResult {
    pub bleached_polyps: Vec<PolypId>,
    pub killed_polyps: Vec<PolypId>,
    pub expelled_symbionts: Vec<SymbiontId>,
    pub damaged_corals: Vec<CoralId>,
}

/// A stress event that can cause bleaching across the reef.
#[derive(Debug, Clone)]
pub struct BleachingEvent {
    severity: BleachSeverity,
    stress_amount: u32,
}

impl BleachingEvent {
    pub fn new(severity: BleachSeverity) -> Self {
        let stress_amount = match severity {
            BleachSeverity::Mild => 20,
            BleachSeverity::Moderate => 45,
            BleachSeverity::Severe => 80,
        };
        Self {
            severity,
            stress_amount,
        }
    }

    pub fn severity(&self) -> BleachSeverity {
        self.severity
    }

    pub fn stress_amount(&self) -> u32 {
        self.stress_amount
    }
}

// ---- Reef ----

/// The top-level reef ecosystem.
#[derive(Debug, Clone)]
pub struct Reef {
    name: String,
    corals: HashMap<CoralId, Coral>,
    polyps: HashMap<PolypId, Polyp>,
    symbionts: HashMap<SymbiontId, Symbiodinium>,
    zones: HashMap<ZoneId, ReefZone>,
    next_coral_id: u64,
    next_polyp_id: u64,
    next_symbiont_id: u64,
    next_zone_id: u64,
}

impl Reef {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            corals: HashMap::new(),
            polyps: HashMap::new(),
            symbionts: HashMap::new(),
            zones: HashMap::new(),
            next_coral_id: 0,
            next_polyp_id: 0,
            next_symbiont_id: 0,
            next_zone_id: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn coral_count(&self) -> usize {
        self.corals.len()
    }

    pub fn polyp_count(&self) -> usize {
        self.polyps.len()
    }

    pub fn symbiont_count(&self) -> usize {
        self.symbionts.len()
    }

    pub fn zone_count(&self) -> usize {
        self.zones.len()
    }

    // -- Creation methods --

    pub fn create_coral(&mut self) -> CoralId {
        let id = CoralId(self.next_coral_id);
        self.next_coral_id += 1;
        self.corals.insert(id, Coral::new(id));
        id
    }

    pub fn create_polyp(&mut self) -> PolypId {
        let id = PolypId(self.next_polyp_id);
        self.next_polyp_id += 1;
        self.polyps.insert(id, Polyp::new(id));
        id
    }

    pub fn create_symbiont(&mut self, energy_output: u32) -> SymbiontId {
        let id = SymbiontId(self.next_symbiont_id);
        self.next_symbiont_id += 1;
        self.symbionts.insert(id, Symbiodinium::new(id, energy_output));
        id
    }

    pub fn create_zone(&mut self, name: &str, depth: u32, light: u32, max_corals: usize) -> ZoneId {
        let id = ZoneId(self.next_zone_id);
        self.next_zone_id += 1;
        self.zones.insert(id, ReefZone::new(id, name, depth, light, max_corals));
        id
    }

    // -- Accessors --

    pub fn coral(&self, id: CoralId) -> Option<&Coral> {
        self.corals.get(&id)
    }

    pub fn coral_mut(&mut self, id: CoralId) -> Option<&mut Coral> {
        self.corals.get_mut(&id)
    }

    pub fn polyp(&self, id: PolypId) -> Option<&Polyp> {
        self.polyps.get(&id)
    }

    pub fn polyp_mut(&mut self, id: PolypId) -> Option<&mut Polyp> {
        self.polyps.get_mut(&id)
    }

    pub fn symbiont(&self, id: SymbiontId) -> Option<&Symbiodinium> {
        self.symbionts.get(&id)
    }

    pub fn symbiont_mut(&mut self, id: SymbiontId) -> Option<&mut Symbiodinium> {
        self.symbionts.get_mut(&id)
    }

    pub fn zone(&self, id: ZoneId) -> Option<&ReefZone> {
        self.zones.get(&id)
    }

    pub fn zone_mut(&mut self, id: ZoneId) -> Option<&mut ReefZone> {
        self.zones.get_mut(&id)
    }

    /// Count alive polyps.
    pub fn alive_polyp_count(&self) -> usize {
        self.polyps.values().filter(|p| p.is_alive()).count()
    }

    /// Feed energy from symbionts to their host polyps.
    pub fn feed_cycle(&mut self) {
        // Collect energy transfers
        let transfers: Vec<(PolypId, u32)> = self
            .symbionts
            .values()
            .filter_map(|s| s.host_polyp.map(|p| (p, s.energy_output())))
            .collect();

        for (polyp_id, energy) in transfers {
            if let Some(polyp) = self.polyps.get_mut(&polyp_id) {
                polyp.add_energy(energy);
            }
        }
    }

    /// Trigger a bleaching event across the entire reef.
    pub fn trigger_bleaching(&mut self, event: &BleachingEvent) -> BleachingResult {
        let stress = event.stress_amount();
        let mut result = BleachingResult {
            bleached_polyps: Vec::new(),
            killed_polyps: Vec::new(),
            expelled_symbionts: Vec::new(),
            damaged_corals: Vec::new(),
        };

        // Stress polyps
        let polyp_ids: Vec<PolypId> = self.polyps.keys().copied().collect();
        for pid in polyp_ids {
            if let Some(polyp) = self.polyps.get_mut(&pid) {
                let prev = polyp.health();
                polyp.apply_stress(stress);
                if matches!(polyp.health(), PolypHealth::Bleached) && !matches!(prev, PolypHealth::Bleached) {
                    result.bleached_polyps.push(pid);
                }
                if matches!(polyp.health(), PolypHealth::Dead) {
                    result.killed_polyps.push(pid);
                }
            }
        }

        // Expel symbionts from bleached polyps
        // Collect (symbiont_id, host_polyp_id) pairs first to avoid borrow conflicts
        let symb_hosts: Vec<(SymbiontId, Option<PolypId>)> = self.symbionts.keys()
            .map(|&sid| (sid, self.symbionts.get(&sid).and_then(|s| s.host())))
            .collect();
        for (sid, host_opt) in symb_hosts {
            if let Some(host) = host_opt {
                let is_affected = self.polyps.get(&host)
                    .map(|p| matches!(p.health(), PolypHealth::Bleached | PolypHealth::Dead))
                    .unwrap_or(false);
                if is_affected {
                    if let Some(symb) = self.symbiont_mut(sid) {
                        symb.deactivate();
                    }
                    result.expelled_symbionts.push(sid);
                }
            }
        }

        // Damage corals
        let coral_ids: Vec<CoralId> = self.corals.keys().copied().collect();
        for cid in coral_ids {
            if let Some(coral) = self.corals.get_mut(&cid) {
                let effective_stress = stress.saturating_sub(coral.resilience());
                if effective_stress > 0 {
                    coral.take_damage(effective_stress);
                    result.damaged_corals.push(cid);
                }
            }
        }

        result
    }

    /// Recovery cycle: polyps with energy can recover health.
    pub fn recovery_cycle(&mut self) -> usize {
        let mut recovered = 0;
        let polyp_ids: Vec<PolypId> = self.polyps.keys().copied().collect();
        for pid in polyp_ids {
            if let Some(polyp) = self.polyps.get_mut(&pid) {
                let prev = polyp.health();
                let _ = polyp.recover();
                if polyp.health() != prev {
                    recovered += 1;
                }
            }
        }
        recovered
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- Polyp tests --

    #[test]
    fn polyp_new() {
        let p = Polyp::new(PolypId(0));
        assert!(p.is_alive());
        assert_eq!(p.health(), PolypHealth::Healthy);
        assert_eq!(p.energy(), 50);
        assert!(p.coral_id().is_none());
        assert!(p.zone().is_none());
        assert!(p.symbiont().is_none());
    }

    #[test]
    fn polyp_energy_management() {
        let mut p = Polyp::new(PolypId(0));
        p.add_energy(30);
        assert_eq!(p.energy(), 80); // capped at 100
        p.add_energy(50);
        assert_eq!(p.energy(), 100); // cap
        assert!(p.consume_energy(40).is_ok());
        assert_eq!(p.energy(), 60);
        assert!(p.consume_energy(70).is_err()); // not enough
    }

    #[test]
    fn polyp_stress_and_recovery() {
        let mut p = Polyp::new(PolypId(0));
        p.apply_stress(40);
        assert_eq!(p.health(), PolypHealth::Stressed);
        p.add_energy(50); // ensure enough energy
        p.recover().unwrap();
        assert_eq!(p.health(), PolypHealth::Healthy);
    }

    #[test]
    fn polyp_bleaching() {
        let mut p = Polyp::new(PolypId(0));
        p.apply_stress(70);
        assert_eq!(p.health(), PolypHealth::Bleached);
        p.add_energy(50);
        p.recover().unwrap();
        assert_eq!(p.health(), PolypHealth::Stressed);
        p.add_energy(50);
        p.recover().unwrap();
        assert_eq!(p.health(), PolypHealth::Healthy);
    }

    #[test]
    fn polyp_death() {
        let mut p = Polyp::new(PolypId(0));
        p.kill();
        assert!(!p.is_alive());
        assert_eq!(p.energy(), 0);
        assert!(p.recover().is_err());
    }

    #[test]
    fn polyp_attach_and_zone() {
        let mut p = Polyp::new(PolypId(0));
        p.attach_to_coral(CoralId(1));
        assert_eq!(p.coral_id(), Some(CoralId(1)));
        p.place_in_zone(ZoneId(2));
        assert_eq!(p.zone(), Some(ZoneId(2)));
        p.detach_from_coral();
        assert!(p.coral_id().is_none());
    }

    // -- Symbiodinium tests --

    #[test]
    fn symbiont_energy_output() {
        let mut s = Symbiodinium::new(SymbiontId(0), 15);
        assert_eq!(s.energy_output(), 15);
        s.deactivate();
        assert_eq!(s.energy_output(), 0);
        assert!(!s.is_active());
        s.reactivate();
        assert_eq!(s.energy_output(), 15);
    }

    #[test]
    fn symbiont_binding() {
        let mut s = Symbiodinium::new(SymbiontId(0), 10);
        assert!(s.host().is_none());
        s.bind(PolypId(1));
        assert_eq!(s.host(), Some(PolypId(1)));
        s.unbind();
        assert!(s.host().is_none());
    }

    // -- Coral tests --

    #[test]
    fn coral_growth_stages() {
        let mut c = Coral::new(CoralId(0));
        assert_eq!(c.stage(), GrowthStage::Seedling);
        assert!(!c.grow(50));
        assert!(c.grow(50)); // hits 100 threshold
        assert_eq!(c.stage(), GrowthStage::Juvenile);
        assert!(c.grow(250)); // hits juvenile threshold
        assert_eq!(c.stage(), GrowthStage::Adult);
        assert!(c.grow(600)); // hits adult threshold
        assert_eq!(c.stage(), GrowthStage::Ancient);
        assert!(!c.grow(1000)); // already ancient
    }

    #[test]
    fn coral_polyp_management() {
        let mut c = Coral::new(CoralId(0));
        c.add_polyp(PolypId(1));
        c.add_polyp(PolypId(2));
        assert_eq!(c.polyp_count(), 2);
        c.add_polyp(PolypId(1)); // duplicate, no effect
        assert_eq!(c.polyp_count(), 2);
        c.remove_polyp(PolypId(1));
        assert_eq!(c.polyp_count(), 1);
    }

    #[test]
    fn coral_damage() {
        let mut c = Coral::new(CoralId(0));
        c.grow(50);
        c.take_damage(30);
        assert_eq!(c.growth_points(), 20);
        c.take_damage(30);
        assert_eq!(c.growth_points(), 0);
    }

    #[test]
    fn coral_resilience_increases() {
        let mut c = Coral::new(CoralId(0));
        assert_eq!(c.resilience(), 10);
        c.grow(100); // advance to juvenile
        assert_eq!(c.resilience(), 25);
    }

    // -- ReefZone tests --

    #[test]
    fn zone_capacity() {
        let mut z = ReefZone::new(ZoneId(0), "shallow", 10, 80, 2);
        assert!(z.has_capacity());
        z.add_coral(CoralId(1)).unwrap();
        z.add_coral(CoralId(2)).unwrap();
        assert!(!z.has_capacity());
        assert!(z.add_coral(CoralId(3)).is_err());
    }

    #[test]
    fn zone_energy_bonus() {
        let z = ReefZone::new(ZoneId(0), "surface", 5, 90, 10);
        assert_eq!(z.energy_bonus(), 9);
        let deep = ReefZone::new(ZoneId(1), "deep", 100, 10, 10);
        assert_eq!(deep.energy_bonus(), 1);
    }

    #[test]
    fn zone_light_capped() {
        let z = ReefZone::new(ZoneId(0), "surface", 5, 150, 10);
        assert_eq!(z.light_level(), 100);
    }

    #[test]
    fn zone_remove_coral() {
        let mut z = ReefZone::new(ZoneId(0), "mid", 30, 50, 5);
        z.add_coral(CoralId(1)).unwrap();
        z.remove_coral(CoralId(1));
        assert_eq!(z.coral_count(), 0);
    }

    // -- Reef integration tests --

    #[test]
    fn reef_create_entities() {
        let mut r = Reef::new("test");
        let c = r.create_coral();
        let p = r.create_polyp();
        let s = r.create_symbiont(20);
        let z = r.create_zone("shallow", 10, 80, 5);
        assert_eq!(r.coral_count(), 1);
        assert_eq!(r.polyp_count(), 1);
        assert_eq!(r.symbiont_count(), 1);
        assert_eq!(r.zone_count(), 1);
    }

    #[test]
    fn reef_feed_cycle() {
        let mut r = Reef::new("test");
        let p = r.create_polyp();
        let s = r.create_symbiont(20);
        r.symbiont_mut(s).unwrap().bind(p);
        let initial_energy = r.polyp(p).unwrap().energy();
        r.feed_cycle();
        assert!(r.polyp(p).unwrap().energy() > initial_energy);
    }

    #[test]
    fn reef_bleaching_mild() {
        let mut r = Reef::new("test");
        let p = r.create_polyp();
        let s = r.create_symbiont(20);
        r.symbiont_mut(s).unwrap().bind(p);

        let event = BleachingEvent::new(BleachSeverity::Mild);
        let result = r.trigger_bleaching(&event);
        // Mild stress shouldn't bleach (20 < 30 threshold for stressed)
        assert!(result.bleached_polyps.is_empty());
    }

    #[test]
    fn reef_bleaching_severe() {
        let mut r = Reef::new("test");
        let p = r.create_polyp();
        let s = r.create_symbiont(20);
        r.symbiont_mut(s).unwrap().bind(p);

        let event = BleachingEvent::new(BleachSeverity::Severe);
        let result = r.trigger_bleaching(&event);
        assert_eq!(result.bleached_polyps.len(), 1);
        assert_eq!(result.expelled_symbionts.len(), 1);
    }

    #[test]
    fn reef_recovery_cycle() {
        let mut r = Reef::new("test");
        let p = r.create_polyp();
        r.polyp_mut(p).unwrap().apply_stress(40); // stressed
        r.polyp_mut(p).unwrap().add_energy(50); // ensure energy
        let recovered = r.recovery_cycle();
        assert_eq!(recovered, 1);
        assert_eq!(r.polyp(p).unwrap().health(), PolypHealth::Healthy);
    }

    #[test]
    fn reef_coral_in_zone_with_polyps() {
        let mut r = Reef::new("test");
        let z = r.create_zone("shallow", 10, 80, 5);
        let c = r.create_coral();
        let p = r.create_polyp();

        r.coral_mut(c).unwrap().place_in_zone(z);
        r.coral_mut(c).unwrap().add_polyp(p);
        r.polyp_mut(p).unwrap().attach_to_coral(c);
        r.polyp_mut(p).unwrap().place_in_zone(z);
        r.zone_mut(z).unwrap().add_coral(c).unwrap();

        assert_eq!(r.zone(z).unwrap().coral_count(), 1);
        assert_eq!(r.coral(c).unwrap().polyp_count(), 1);
    }
}

use crate::resources::Resources;

/// Trait cho các Plugin trong hệ thống.
pub trait Plugin {
    fn build(&self, app: &mut crate::App);
}

/// Trait cho tất cả các System trong ECS.
pub trait System {
    fn run(&mut self, world: &mut World, resources: &mut Resources);
}

/// ECS World lưu trữ tất cả các component cho mọi entity.
#[derive(Default)]
pub struct World {
    pub texts: Vec<Option<crate::task::components::Text>>,
    pub statuses: Vec<Option<crate::task::components::Status>>,
    pub priorities: Vec<Option<crate::task::components::Priority>>,
    pub selecteds: Vec<Option<crate::ui::components::Selected>>,
    pub editings: Vec<Option<crate::ui::components::Editing>>,
    pub visibles: Vec<Option<crate::ui::components::Visible>>,
    pub hovers: Vec<Option<crate::ui::components::Hover>>,
    pub actives: Vec<Option<crate::ui::components::Active>>,
    pub clicks: Vec<Option<crate::ui::components::Click>>,
    pub dirties: Vec<Option<crate::task::components::Dirty>>,
    pub disableds: Vec<Option<crate::ui::components::Disabled>>,
    pub bounds: Vec<Option<crate::ui::components::Bounds>>,
    pub styles: Vec<Option<crate::ui::components::Style>>,
    pub creates: Vec<Option<crate::task::components::Create>>,
    pub deletes: Vec<Option<crate::task::components::Delete>>,
    pub parents: Vec<Option<crate::task::components::Parent>>,
    pub childrens: Vec<Option<crate::task::components::Children>>,
    pub to_delete: Vec<Option<()>>,
    pub collapseds: Vec<Option<crate::task::components::Collapsed>>,
    pub dues: Vec<Option<crate::task::components::Due>>,
    pub schedulings: Vec<Option<crate::task::components::Scheduling>>,
    pub aligns: Vec<Option<crate::ui::components::Align>>,
    pub entity_count: usize,
    pub container: Vec<Option<crate::ui::components::Container>>,
    pub flows: Vec<Option<crate::ui::components::Flow>>,
    pub justifys: Vec<Option<crate::ui::components::Justify>>,
    pub buttons: Vec<Option<crate::ui::components::Button>>,
    pub users: Vec<Option<crate::user::components::User>>,
    pub owners: Vec<Option<crate::user::components::Owner>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            texts: vec![],
            statuses: vec![],
            priorities: vec![],
            selecteds: vec![],
            editings: vec![],
            visibles: vec![],
            hovers: vec![],
            actives: vec![],
            clicks: vec![],
            dirties: vec![],
            disableds: vec![],
            bounds: vec![],
            styles: vec![],
            creates: vec![],
            deletes: vec![],
            parents: vec![],
            childrens: vec![],
            to_delete: vec![],
            collapseds: vec![],
            dues: vec![],
            schedulings: vec![],
            aligns: vec![],
            entity_count: 0,
            container: vec![],
            flows: vec![],
            justifys: vec![],
            buttons: vec![],
            users: vec![],
            owners: vec![],
        }
    }
    
    pub fn spawn(&mut self) -> usize {
        let id = self.entity_count;
        self.entity_count += 1;
        self.texts.push(None);
        self.statuses.push(None);
        self.priorities.push(None);
        self.selecteds.push(None);
        self.editings.push(None);
        self.visibles.push(None);
        self.hovers.push(None);
        self.actives.push(None);
        self.clicks.push(None);
        self.dirties.push(None);
        self.disableds.push(None);
        self.bounds.push(None);
        self.styles.push(None);
        self.creates.push(None);
        self.deletes.push(None);
        self.parents.push(None);
        self.childrens.push(None);
        self.to_delete.push(None);
        self.collapseds.push(None);
        self.dues.push(None);
        self.schedulings.push(None);
        self.aligns.push(None);
        self.buttons.push(None);
        self.container.push(None);
        self.flows.push(None);
        self.justifys.push(None);
        self.users.push(None);
        self.owners.push(None);
        id
    }
    
    pub fn mark_for_delete(&mut self, id: usize) {
        self.to_delete[id] = Some(());
    }
    
    /// Xóa tất cả entity đã được đánh dấu to_delete và thu gọn các vector component.
    pub fn sweep(&mut self) {
        let mut keep = vec![];
        for id in 0..self.entity_count {
            if self.to_delete[id].is_none() {
                keep.push(id);
            }
        }
        macro_rules! sweep_vec {
            ($vec:expr) => {
                let mut new_vec = vec![];
                for &id in &keep {
                    new_vec.push($vec[id].take());
                }
                $vec = new_vec;
            };
        }
        sweep_vec!(self.texts);
        sweep_vec!(self.statuses);
        sweep_vec!(self.priorities);
        sweep_vec!(self.selecteds);
        sweep_vec!(self.editings);
        sweep_vec!(self.visibles);
        sweep_vec!(self.hovers);
        sweep_vec!(self.actives);
        sweep_vec!(self.clicks);
        sweep_vec!(self.dirties);
        sweep_vec!(self.disableds);
        sweep_vec!(self.bounds);
        sweep_vec!(self.styles);
        sweep_vec!(self.creates);
        sweep_vec!(self.deletes);
        sweep_vec!(self.parents);
        sweep_vec!(self.childrens);
        sweep_vec!(self.to_delete);
        sweep_vec!(self.collapseds);
        sweep_vec!(self.dues);
        sweep_vec!(self.schedulings);
        sweep_vec!(self.aligns);
        sweep_vec!(self.buttons);
        sweep_vec!(self.container);
        sweep_vec!(self.flows);
        sweep_vec!(self.justifys);
        sweep_vec!(self.users);
        sweep_vec!(self.owners);
        self.entity_count = keep.len();
    }
}

/// Bộ điều phối hệ thống, quản lý và thực thi các system.
#[derive(Default)]
pub struct Scheduler {
    systems: Vec<Box<dyn System>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }
    
    pub fn add(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
    }
    
    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        for system in self.systems.iter_mut() {
            system.run(world, resources);
        }
    }
} 
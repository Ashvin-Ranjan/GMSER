use log::{info, warn};
use std::io::Cursor;

use crate::data_loader::utils::{
    chunk::Chunk,
    cursor::{handle_cursor_alignment, CustomCursor, Deserializable},
    error::DataLoadError,
};

#[derive(Debug)]
pub struct Object {
    pub name: String,
    pub sprite_id: u32,
    pub visible: bool,
    pub managed: bool,
    pub solid: bool,
    pub depth: i32,
    pub persistent: bool,
    pub parent_object_id: u32,
    pub mask_sprite_id: u32,
    pub physics: ObjectPhysics,
    pub events: ObjectEvents,
}

#[derive(Debug)]
pub struct ObjectEvents {
    pub create: Vec<Event>,
    pub destroy: Vec<Event>,
    pub alarm: Vec<Event>,
    pub step: Vec<Event>,
    pub collision: Vec<Event>,
    pub keyboard: Vec<Event>,
    pub mouse: Vec<Event>,
    pub other: Vec<Event>,
    pub draw: Vec<Event>,
    pub key_press: Vec<Event>,
    pub key_release: Vec<Event>,
    pub trigger: Vec<Event>,
    pub clean_up: Vec<Event>,
    pub gesture: Vec<Event>,
    pub pre_create: Vec<Event>,
}

impl ObjectEvents {
    const NUM_ARRAYS: usize = 15;
}

#[derive(Debug)]
pub struct ObjectPhysics {
    pub is_enabled: bool,
    pub sensor: bool,
    pub shape: CollisionShape,
    pub density: f32,
    pub restitution: f32,
    pub group: u32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub friction: f32,
    pub is_awake: bool,
    pub is_kinematic: bool,
    pub vertices: Vec<PhysicsVertex>,
}

#[derive(Debug)]
pub enum CollisionShape {
    CIRCLE,
    BOX,
    CUSTOM,
}

#[derive(Debug)]
pub struct PhysicsVertex {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub subtype: u32,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub lib_id: u32,
    pub id: u32,
    pub kind: u32,
    pub use_relative: bool,
    pub is_question: bool,
    pub use_apply_to: bool,
    pub exe_type: u32,
    pub action_name: Option<String>,
    pub code_id: u32,
    pub argument_count: u32,
    pub who: u32,
    pub relative: bool,
    pub is_not: bool,
}

#[derive(Debug)]
pub struct ObjtChunk {
    pub size: u32,
    pub objects: Vec<Object>,
}

impl Chunk for ObjtChunk {
    const IDENT: [u8; 4] = [0x4F, 0x42, 0x4A, 0x54]; // "OBJT"
}

impl Deserializable for ObjtChunk {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        info!("Deserializing OBJT");

        let ident = cursor.read_ident()?;

        if ident != ObjtChunk::IDENT {
            return Err(DataLoadError::UnexpectedIdent {
                pos: cursor.position() - 4,
                expected: ObjtChunk::IDENT,
                actual: ident,
            });
        }

        let size = cursor.read_u32()?;

        let start_pos = cursor.position();

        let objects = cursor.read_pointer_list::<Object>(0)?;

        handle_cursor_alignment(cursor, start_pos, size as u64, true)?;

        Ok(ObjtChunk { size, objects })
    }
}

impl Deserializable for Object {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let name = cursor.read_obj_pointer::<String>(0)?;
        let sprite_id = cursor.read_u32()?;
        let visible = cursor.read_wide_boolean()?;
        let managed = cursor.read_wide_boolean()?;
        let solid = cursor.read_wide_boolean()?;
        let depth = cursor.read_i32()?;
        let persistent = cursor.read_wide_boolean()?;
        let parent_object_id = cursor.read_u32()?;
        let mask_sprite_id = cursor.read_u32()?;
        let physics = ObjectPhysics::deserialize(cursor)?;
        let events = ObjectEvents::deserialize(cursor)?;

        Ok(Object {
            name,
            sprite_id,
            visible,
            managed,
            solid,
            depth,
            persistent,
            parent_object_id,
            mask_sprite_id,
            physics,
            events,
        })
    }
}

impl Deserializable for ObjectPhysics {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let is_enabled = cursor.read_wide_boolean()?;
        let sensor = cursor.read_wide_boolean()?;

        let shape_index = cursor.read_u32()?;
        let shape = match shape_index {
            0 => Ok(CollisionShape::CIRCLE),
            1 => Ok(CollisionShape::BOX),
            2 => Ok(CollisionShape::CUSTOM),
            _ => Err(DataLoadError::InvalidCollisionShapeKind {
                pos: cursor.position() - 4,
                kind: shape_index,
            }),
        }?;

        let density = cursor.read_f32()?;
        let restitution = cursor.read_f32()?;
        let group = cursor.read_u32()?;
        let linear_damping = cursor.read_f32()?;
        let angular_damping = cursor.read_f32()?;
        let vertex_count = cursor.read_u32()?;
        let friction = cursor.read_f32()?;
        let is_awake = cursor.read_wide_boolean()?;
        let is_kinematic = cursor.read_wide_boolean()?;

        let mut vertices = Vec::new();
        for _ in 0..vertex_count {
            vertices.push(PhysicsVertex::deserialize(cursor)?);
        }

        Ok(ObjectPhysics {
            is_enabled,
            sensor,
            shape,
            density,
            restitution,
            group,
            linear_damping,
            angular_damping,
            friction,
            is_awake,
            is_kinematic,
            vertices,
        })
    }
}

impl Deserializable for PhysicsVertex {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let x = cursor.read_f32()?;
        let y = cursor.read_f32()?;

        Ok(PhysicsVertex { x, y })
    }
}

impl Deserializable for Event {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let subtype = cursor.read_u32()?;
        let actions = cursor.read_pointer_list::<Action>(0)?;

        Ok(Event { subtype, actions })
    }
}

impl Deserializable for Action {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let lib_id = cursor.read_u32()?;
        let id = cursor.read_u32()?;
        let kind = cursor.read_u32()?;
        let use_relative = cursor.read_wide_boolean()?;
        let is_question = cursor.read_wide_boolean()?;
        let use_apply_to = cursor.read_wide_boolean()?;
        let exe_type = cursor.read_u32()?;
        let action_name = cursor.read_opt_pointer::<String>(0)?;
        let code_id = cursor.read_u32()?;
        let argument_count = cursor.read_u32()?;
        let who = cursor.read_u32()?;
        let relative = cursor.read_wide_boolean()?;
        let is_not = cursor.read_wide_boolean()?;
        let unk_expect_0 = cursor.read_u32()?;
        if unk_expect_0 != 0 {
            warn!(
                "Expected 0 at position {}, got {}",
                cursor.position() - 4,
                unk_expect_0
            );
        }

        Ok(Action {
            lib_id,
            id,
            kind,
            use_relative,
            is_question,
            use_apply_to,
            exe_type,
            action_name,
            code_id,
            argument_count,
            who,
            relative,
            is_not,
        })
    }
}

impl Deserializable for ObjectEvents {
    fn deserialize(cursor: &mut Cursor<&[u8]>) -> Result<Self, DataLoadError>
    where
        Self: Sized,
    {
        let start_pos = cursor.position();
        let events = Vec::<Vec<Event>>::deserialize(cursor)?;
        if events.len() != ObjectEvents::NUM_ARRAYS {
            return Err(DataLoadError::InvalidEventArrayLength {
                pos: start_pos,
                actual: events.len(),
                correct: ObjectEvents::NUM_ARRAYS,
            });
        }

        Ok(ObjectEvents {
            create: events[0].clone(),
            destroy: events[1].clone(),
            alarm: events[2].clone(),
            step: events[3].clone(),
            collision: events[4].clone(),
            keyboard: events[5].clone(),
            mouse: events[6].clone(),
            other: events[7].clone(),
            draw: events[8].clone(),
            key_press: events[9].clone(),
            key_release: events[10].clone(),
            trigger: events[11].clone(),
            clean_up: events[12].clone(),
            gesture: events[13].clone(),
            pre_create: events[14].clone(),
        })
    }
}

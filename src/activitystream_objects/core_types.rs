use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

use super::{
    activities::*,
    actors::Actor,
    collections::ExtendsCollection,
    link::LinkSimpleOrExpanded,
    object::{Object, ObjectWrapper},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityStream {
    #[serde(flatten)]
    pub content: ContextWrap,
}

impl ActivityStream {
    pub fn get_actor(self) -> Option<Box<Actor>> {
        // match self.content.activity_stream {
        //     RangeLinkExtendsObject::Object(ExtendsObject::Actor(x)) => Some(x),
        //     _ => None,
        // }
        match self.content.activity_stream {
            ExtendsObject::Actor(x) => Some(x),
            _ => None,
        }
    }
    pub fn get_activity(self) -> Option<Box<ExtendsIntransitive>> {
        // match self.content.activity_stream {
        //     RangeLinkExtendsObject::Object(ExtendsObject::ExtendsIntransitive(x)) => Some(x),
        //     _ => None,
        // }
        match self.content.activity_stream {
            ExtendsObject::ExtendsIntransitive(x) => Some(x),
            _ => None,
        }
    }
    pub fn get_object(self) -> Option<Box<ObjectWrapper>> {
        // match self.content.activity_stream {
        //     RangeLinkExtendsObject::Object(ExtendsObject::Object(x)) => Some(x),
        //     _ => None,
        // }
        match self.content.activity_stream {
            ExtendsObject::Object(x) => Some(x),
            _ => None,
        }
    }
    pub fn get_extends_object(self) -> ExtendsObject {
        // match self.content.activity_stream {
        //     RangeLinkExtendsObject::Object(x) => Some(x),
        //     _ => None,
        // }
        return self.content.activity_stream;
    }
    pub fn is_activity(&self) -> bool {
        // matches!(
        //     &self.content.activity_stream,
        //     RangeLinkExtendsObject::Object(ExtendsObject::ExtendsIntransitive(_))
        // )
        matches!(
            &self.content.activity_stream,
            ExtendsObject::ExtendsIntransitive(_)
        )
    }
    // pub async fn verify_attribution(&self, cache: &Cache, conn: &Data<DbConn>) -> Result<(), ()> {
    //     // match &self.content.activity_stream {
    //     //     RangeLinkExtendsObject::Object(ExtendsObject::ExtendsIntransitive(x)) => match &**x {
    //     //         ExtendsIntransitive::ExtendsActivity(x) => x.verify_attribution(cache, conn).await,
    //     //         _ => Ok(()),
    //     //     },
    //     //     _ => Ok(()),
    //     // }
    //     match &self.content.activity_stream {
    //         ExtendsObject::ExtendsIntransitive(x) => match &**x {
    //             ExtendsIntransitive::ExtendsActivity(x) => x.verify_attribution(cache, conn).await,
    //             _ => Ok(()),
    //         },
    //         _ => Ok(()),
    //     }
    // }
    pub fn get_owner(&self) -> Option<&Url> {
        // match &self.content.activity_stream {
        //     RangeLinkExtendsObject::Object(x) => match x {
        //         ExtendsObject::Object(x) => match &x.object.attributed_to {
        //             Some(x) => Some(x.get_id()),
        //             None => None,
        //         },
        //         ExtendsObject::ExtendsIntransitive(x) => Some(x.get_actor()),
        //         ExtendsObject::ExtendsCollection(_) => None,
        //         ExtendsObject::Actor(x) => Some(x.get_id()),
        //     },
        //     RangeLinkExtendsObject::Link(x) => todo!(),
        // }
        match &self.content.activity_stream {
            ExtendsObject::Object(x) => match &x.object.attributed_to {
                Some(x) => Some(x.get_id()),
                None => None,
            },
            ExtendsObject::ExtendsIntransitive(x) => Some(x.get_actor()),
            ExtendsObject::ExtendsCollection(_) => None,
            ExtendsObject::Actor(x) => Some(x.get_id()),
        }
    }
}

//-------------------glue--------------
#[derive(Serialize, Deserialize, Debug, Clone)]
/// wraps base object to include context
pub struct ContextWrap {
    #[serde(rename = "@context")]
    pub context: Context,
    #[serde(flatten)]
    pub activity_stream: ExtendsObject,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Context {
    Array(Vec<ContextItem>),
    Single(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ContextItem {
    String(String),
    Map(HashMap<String, ContextMapItem>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ContextMapItem {
    String(String),
    Map(HashMap<String, String>),
}

//--------------------inheritence---------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExtendsObject {
    Object(Box<ObjectWrapper>),
    ExtendsIntransitive(Box<ExtendsIntransitive>),
    ExtendsCollection(Box<ExtendsCollection>),
    Actor(Box<Actor>),
}

impl ExtendsObject {
    pub fn get_as_object(&self) -> Option<&Object> {
        let ExtendsObject::Object(object) = self else {
            return None;
        };
        Some(&object.object)
    }
    pub fn get_as_activity(&self) -> Option<&ExtendsIntransitive> {
        let ExtendsObject::ExtendsIntransitive(activity) = self else {
            return None;
        };
        Some(activity)
    }
    pub fn get_id(&self) -> &Url {
        match self {
            ExtendsObject::Object(x) => &x.object.id.id,
            ExtendsObject::ExtendsIntransitive(x) => x.get_id(),
            ExtendsObject::ExtendsCollection(_x) => todo!(),
            ExtendsObject::Actor(x) => &x.id,
        }
    }
}

//--------------primitive-----------------

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum LinkOrArray {
    Single(Box<LinkSimpleOrExpanded>),
    Multiple(Vec<LinkSimpleOrExpanded>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SimpleLinkOrArray {
    Single(Url),
    Multiple(Vec<Url>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum RangeLinkObjOrArray {
    Single(RangeLinkExtendsObject),
    Multiple(Vec<RangeLinkExtendsObject>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// represents a field that could be an object or a link
pub enum RangeLinkExtendsObject {
    Object(ExtendsObject),
    Link(Box<LinkSimpleOrExpanded>),
}

impl RangeLinkExtendsObject {
    // pub async fn get_concrete(
    //     &self,
    //     cache: &Cache,
    //     conn: &Data<DbConn>,
    // ) -> Result<ExtendsObject, ConcreteErr> {
    //     match self {
    //         RangeLinkExtendsObject::Object(x) => Ok(x.clone()),
    //         RangeLinkExtendsObject::Link(x) => {
    //             let val = fetch_object(x.get_id(), cache, conn).await;

    //             match val {
    //                 Ok(x) => {
    //                     // let object = x.get_extends_object();

    //                     // match object {
    //                     //     Some(x) => Ok(x),
    //                     //     None => Err(ConcreteErr::NotAnObject),
    //                     // }
    //                     Ok(x.get_extends_object())
    //                 }
    //                 Err(x) => Err(ConcreteErr::FetchErr(x)),
    //             }
    //         }
    //     }
    // }
    pub fn get_id(&self) -> &Url {
        match self {
            RangeLinkExtendsObject::Object(x) => x.get_id(),
            RangeLinkExtendsObject::Link(x) => x.get_id(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// represents a field that could be an object or a link
pub enum RangeLinkObject {
    Object(ObjectWrapper),
    Link(Box<LinkSimpleOrExpanded>),
}

// #[derive(Debug, Clone)]
// pub enum ConcreteErr {
//     FetchErr(FetchErr),
//     NotAnObject,
// }
impl RangeLinkObject {
    pub fn get_id(&self) -> &Url {
        match self {
            RangeLinkObject::Object(x) => &x.object.id.id,
            RangeLinkObject::Link(x) => x.get_id(),
        }
    }
    // pub async fn get_concrete(
    //     &self,
    //     cache: &Cache,
    //     conn: &Data<DbConn>,
    // ) -> Result<ObjectWrapper, ConcreteErr> {
    //     match self {
    //         RangeLinkObject::Object(x) => Ok(x.clone()),
    //         RangeLinkObject::Link(x) => {
    //             let val = fetch_object(x.get_id(), cache, conn).await;

    //             match val {
    //                 Ok(x) => {
    //                     let object = x.get_object();

    //                     match object {
    //                         Some(x) => Ok(*x),
    //                         None => Err(ConcreteErr::NotAnObject),
    //                     }
    //                 }
    //                 Err(x) => Err(ConcreteErr::FetchErr(x)),
    //             }
    //         }
    //     }
    // }
}

use webapi::media_devices::MediaDevices;
use webcore::value::Reference;

// https://developer.mozilla.org/en-US/docs/Web/API/Navigator
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Navigator")]
pub struct Navigator(Reference);

impl Navigator {
    //https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaDevices
    pub fn media_devices(&self) -> MediaDevices {
        js! (
            return @{self}.mediaDevices;
        )
    }
}
use webcore::value::Reference;

//https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MediaDevices")]
pub struct MediaDevices(Reference);

// https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MediaDeviceInfo")]
pub struct MediaDeviceInfo(Reference);

//https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MediaStream")]
pub struct MediaStream(Reference);

//https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamConstraints
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "MediaStreamConstraints")]
pub struct MediaStreamConstraints(Reference);

//https://developer.mozilla.org/en-US/docs/Web/API/MediaTrackSupportedConstraints
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MediaTrackSupportedConstraints {
    //TODO
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeviceKind {
    VideoInput,
    AudioInput,
    AudioOutput,
}

impl MediaDevices {
    //https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/enumerateDevices
    pub fn enumerate_devices(&self) -> MediaDeviceInfo {
        js! (
            return @{self}.enumerateDevices();
        ).try_into().unwrap()
    }

    //https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getSupportedConstraints
    pub fn get_supported_contraints(&self) -> MediaTrackSupportedConstraints {
        js! (
            return @{self}.getSupportedContraints();
        ).try_into().unwrap()
    }

    //https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia
    pub fn get_user_media(&self, contraints: MediaStreamConstraints) -> Result<MediaStream, ()> {
        js! (
            return @{self}.getUserMedia(@{contraints});
        ).try_into().unwrap()
    }
}

impl MediaDeviceInfo {
    //https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/deviceId
    pub fn get_device_id(&self) -> String {
        js! (
            return @{self}.deviceId;
        ).try_into().unwrap()
    }

    //https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/groupId
    pub fn get_group_id(&self) -> String {
        js! (
            return @{self}.groupId;
        ).try_into().unwrap()
    }

    //https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/kind
    pub fn get_kind(&self) -> DeviceKind {
        let kind: String = js! (
            return @{self}.kind;
        ).try_into().unwrap();
        match kind.as_ref() {
            "videoinput" => {
                DeviceKind::VideoInput
            }
            "audioinput" => {
                DeviceKind::AudioInput
            }
            "audiooutput" => {
                DeviceKind::AudioOutput
            }
            _ => panic!("Unexpected input for device kind")
        }
    }

    //https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/label
    pub fn get_label(&self) -> String {
        js! (
            return @{self}.label;
        ).try_into().unwrap()
    }
}
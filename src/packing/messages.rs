use num_derive::*;

#[derive(FromPrimitive, ToPrimitive)]
pub enum Message {
    // Device Messages
    GetService = 2,
    StateService = 3,
    GetHostInfo = 12,
    StateHostInfo = 13,
    GetHostFirmware = 14,
    StateHostFirmware = 15,
    GetWifiInfo = 16,
    StateWifiInfo = 17,
    GetWifiFirmware = 18,
    StateWifiFirmware = 19,
    GetPower = 20,
    SetPower = 21,
    StatePower = 22,
    GetLabel = 23,
    SetLabel = 24,
    StateLabel = 25,
    GetVersion = 32,
    StateVersion = 33,
    GetInfo = 34,
    StateInfo = 35,
    Acknowledgement = 45,
    GetLocation = 48,
    SetLocation = 49,
    StateLocation = 50,
    GetGroup = 51,
    SetGroup = 52,
    StateGroup = 53,
    EchoRequest = 58,
    EchoResponse = 59,

    // Light Messages
    Get = 101,
    SetColor = 102,
    SetWaveform = 103,
    SetWaveformOptional = 119,
    State = 107,
    GetLightPower = 116,
    SetLightPower = 117,
    StateLightPower = 118,
    GetInfrared = 120,
    StateInfrared = 121,
    SetInfrared = 122,

    // MultiZone Messages
    SetExtendedColorZones = 510,
    GetExtendedColorZones = 511,
    StateExtendedColorZones = 512,
    SetColorZones = 501,
    GetColorZones = 502,
    StateZone = 503,
    StateMultiZone = 506,

    // Tile Messages
    GetDeviceChain = 701,
    StateDeviceChain = 702,
    SetUserPosition = 703,
    GetTileState64 = 707,
    StateTileState64 = 711,
    SetTileState64 = 715,

    // Switch Messages
    GetRelayPower = 816,
    SetRelayPower = 817,
    StateRelayPower = 818,

    // Firmware Effects
    SetMultiZoneEffect = 508,
    GetMultiZoneEffect = 507,
    StateMultiZoneEffect = 509,
    SetTileEffect = 719,
    GetTileEffect = 718,
    StateTileEffect = 720,
}

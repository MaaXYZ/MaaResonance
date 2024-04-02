type CallbackEvent =
    | "Invalid"
    | "ResourceStartLoading"
    | "ResourceLoadingCompleted"
    | "ResourceLoadingFailed"
    | "ControllerUUIDGot"
    | "ControllerUUIDGetFailed"
    | "ControllerResolutionGot"
    | "ControllerResolutionGetFailed"
    | "ControllerScreencapInited"
    | "ControllerScreencapInitFailed"
    | "ControllerTouchInputInited"
    | "ControllerTouchInputInitFailed"
    | "ControllerActionStarted"
    | "ControllerActionCompleted"
    | "ControllerActionFailed"
    | "ControllerConnectSuccess"
    | "ControllerConnectFailed"
    | "TaskStarted"
    | "TaskCompleted"
    | "TaskFailed"
    | "TaskStopped"
    | "TaskFocusHit"
    | "TaskFocusRunout"
    | "TaskFocusCompleted";

export default interface CallbackPayload {
    event: CallbackEvent;
    data: any;
}

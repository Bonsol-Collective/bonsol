"use strict";
// automatically generated by the FlatBuffers compiler, do not modify
Object.defineProperty(exports, "__esModule", { value: true });
exports.unionListToChannelInstructionData = exports.unionToChannelInstructionData = exports.ChannelInstructionData = void 0;
const execution_request_v1_js_1 = require("../bonsol/execution-request-v1.js");
const status_v1_js_1 = require("../bonsol/status-v1.js");
var ChannelInstructionData;
(function (ChannelInstructionData) {
    ChannelInstructionData[ChannelInstructionData["NONE"] = 0] = "NONE";
    ChannelInstructionData[ChannelInstructionData["ExecuteV1"] = 1] = "ExecuteV1";
    ChannelInstructionData[ChannelInstructionData["StatusV1"] = 2] = "StatusV1";
})(ChannelInstructionData || (exports.ChannelInstructionData = ChannelInstructionData = {}));
function unionToChannelInstructionData(type, accessor) {
    switch (ChannelInstructionData[type]) {
        case 'NONE': return null;
        case 'ExecuteV1': return accessor(new execution_request_v1_js_1.ExecutionRequestV1());
        case 'StatusV1': return accessor(new status_v1_js_1.StatusV1());
        default: return null;
    }
}
exports.unionToChannelInstructionData = unionToChannelInstructionData;
function unionListToChannelInstructionData(type, accessor, index) {
    switch (ChannelInstructionData[type]) {
        case 'NONE': return null;
        case 'ExecuteV1': return accessor(index, new execution_request_v1_js_1.ExecutionRequestV1());
        case 'StatusV1': return accessor(index, new status_v1_js_1.StatusV1());
        default: return null;
    }
}
exports.unionListToChannelInstructionData = unionListToChannelInstructionData;

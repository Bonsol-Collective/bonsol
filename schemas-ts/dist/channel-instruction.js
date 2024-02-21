"use strict";
// automatically generated by the FlatBuffers compiler, do not modify
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.ChannelInstruction = void 0;
const flatbuffers = __importStar(require("flatbuffers"));
const channel_instruction_data_js_1 = require("./channel-instruction-data.js");
class ChannelInstruction {
    constructor() {
        this.bb = null;
        this.bb_pos = 0;
    }
    __init(i, bb) {
        this.bb_pos = i;
        this.bb = bb;
        return this;
    }
    static getRootAsChannelInstruction(bb, obj) {
        return (obj || new ChannelInstruction()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
    }
    static getSizePrefixedRootAsChannelInstruction(bb, obj) {
        bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
        return (obj || new ChannelInstruction()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
    }
    instructionType() {
        const offset = this.bb.__offset(this.bb_pos, 4);
        return offset ? this.bb.readUint8(this.bb_pos + offset) : channel_instruction_data_js_1.ChannelInstructionData.NONE;
    }
    instruction(obj) {
        const offset = this.bb.__offset(this.bb_pos, 6);
        return offset ? this.bb.__union(obj, this.bb_pos + offset) : null;
    }
    static startChannelInstruction(builder) {
        builder.startObject(2);
    }
    static addInstructionType(builder, instructionType) {
        builder.addFieldInt8(0, instructionType, channel_instruction_data_js_1.ChannelInstructionData.NONE);
    }
    static addInstruction(builder, instructionOffset) {
        builder.addFieldOffset(1, instructionOffset, 0);
    }
    static endChannelInstruction(builder) {
        const offset = builder.endObject();
        return offset;
    }
    static finishChannelInstructionBuffer(builder, offset) {
        builder.finish(offset);
    }
    static finishSizePrefixedChannelInstructionBuffer(builder, offset) {
        builder.finish(offset, undefined, true);
    }
    static createChannelInstruction(builder, instructionType, instructionOffset) {
        ChannelInstruction.startChannelInstruction(builder);
        ChannelInstruction.addInstructionType(builder, instructionType);
        ChannelInstruction.addInstruction(builder, instructionOffset);
        return ChannelInstruction.endChannelInstruction(builder);
    }
}
exports.ChannelInstruction = ChannelInstruction;

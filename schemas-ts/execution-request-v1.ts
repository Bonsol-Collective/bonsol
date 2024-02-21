// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { ExecutionInputType } from './execution-input-type.js';


export class ExecutionRequestV1 {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):ExecutionRequestV1 {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsExecutionRequestV1(bb:flatbuffers.ByteBuffer, obj?:ExecutionRequestV1):ExecutionRequestV1 {
  return (obj || new ExecutionRequestV1()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsExecutionRequestV1(bb:flatbuffers.ByteBuffer, obj?:ExecutionRequestV1):ExecutionRequestV1 {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new ExecutionRequestV1()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

inputType():ExecutionInputType {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : ExecutionInputType.URL;
}

mutate_input_type(value:ExecutionInputType):boolean {
  const offset = this.bb!.__offset(this.bb_pos, 4);

  if (offset === 0) {
    return false;
  }

  this.bb!.writeUint8(this.bb_pos + offset, value);
  return true;
}

tip():bigint {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readUint64(this.bb_pos + offset) : BigInt('0');
}

mutate_tip(value:bigint):boolean {
  const offset = this.bb!.__offset(this.bb_pos, 6);

  if (offset === 0) {
    return false;
  }

  this.bb!.writeUint64(this.bb_pos + offset, value);
  return true;
}

executionId(index: number):number|null {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? this.bb!.readUint8(this.bb!.__vector(this.bb_pos + offset) + index) : 0;
}

executionIdLength():number {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

executionIdArray():Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? new Uint8Array(this.bb!.bytes().buffer, this.bb!.bytes().byteOffset + this.bb!.__vector(this.bb_pos + offset), this.bb!.__vector_len(this.bb_pos + offset)) : null;
}

imageId(index: number):number|null {
  const offset = this.bb!.__offset(this.bb_pos, 10);
  return offset ? this.bb!.readUint8(this.bb!.__vector(this.bb_pos + offset) + index) : 0;
}

imageIdLength():number {
  const offset = this.bb!.__offset(this.bb_pos, 10);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

imageIdArray():Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 10);
  return offset ? new Uint8Array(this.bb!.bytes().buffer, this.bb!.bytes().byteOffset + this.bb!.__vector(this.bb_pos + offset), this.bb!.__vector_len(this.bb_pos + offset)) : null;
}

callbackProgramId(index: number):number|null {
  const offset = this.bb!.__offset(this.bb_pos, 12);
  return offset ? this.bb!.readUint8(this.bb!.__vector(this.bb_pos + offset) + index) : 0;
}

callbackProgramIdLength():number {
  const offset = this.bb!.__offset(this.bb_pos, 12);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

callbackProgramIdArray():Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 12);
  return offset ? new Uint8Array(this.bb!.bytes().buffer, this.bb!.bytes().byteOffset + this.bb!.__vector(this.bb_pos + offset), this.bb!.__vector_len(this.bb_pos + offset)) : null;
}

callbackInstructionPrefix(index: number):number|null {
  const offset = this.bb!.__offset(this.bb_pos, 14);
  return offset ? this.bb!.readUint8(this.bb!.__vector(this.bb_pos + offset) + index) : 0;
}

callbackInstructionPrefixLength():number {
  const offset = this.bb!.__offset(this.bb_pos, 14);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

callbackInstructionPrefixArray():Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 14);
  return offset ? new Uint8Array(this.bb!.bytes().buffer, this.bb!.bytes().byteOffset + this.bb!.__vector(this.bb_pos + offset), this.bb!.__vector_len(this.bb_pos + offset)) : null;
}

inputData(index: number):number|null {
  const offset = this.bb!.__offset(this.bb_pos, 16);
  return offset ? this.bb!.readUint8(this.bb!.__vector(this.bb_pos + offset) + index) : 0;
}

inputDataLength():number {
  const offset = this.bb!.__offset(this.bb_pos, 16);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

inputDataArray():Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 16);
  return offset ? new Uint8Array(this.bb!.bytes().buffer, this.bb!.bytes().byteOffset + this.bb!.__vector(this.bb_pos + offset), this.bb!.__vector_len(this.bb_pos + offset)) : null;
}

static startExecutionRequestV1(builder:flatbuffers.Builder) {
  builder.startObject(7);
}

static addInputType(builder:flatbuffers.Builder, inputType:ExecutionInputType) {
  builder.addFieldInt8(0, inputType, ExecutionInputType.URL);
}

static addTip(builder:flatbuffers.Builder, tip:bigint) {
  builder.addFieldInt64(1, tip, BigInt('0'));
}

static addExecutionId(builder:flatbuffers.Builder, executionIdOffset:flatbuffers.Offset) {
  builder.addFieldOffset(2, executionIdOffset, 0);
}

static createExecutionIdVector(builder:flatbuffers.Builder, data:number[]|Uint8Array):flatbuffers.Offset {
  builder.startVector(1, data.length, 1);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addInt8(data[i]!);
  }
  return builder.endVector();
}

static startExecutionIdVector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(1, numElems, 1);
}

static addImageId(builder:flatbuffers.Builder, imageIdOffset:flatbuffers.Offset) {
  builder.addFieldOffset(3, imageIdOffset, 0);
}

static createImageIdVector(builder:flatbuffers.Builder, data:number[]|Uint8Array):flatbuffers.Offset {
  builder.startVector(1, data.length, 1);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addInt8(data[i]!);
  }
  return builder.endVector();
}

static startImageIdVector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(1, numElems, 1);
}

static addCallbackProgramId(builder:flatbuffers.Builder, callbackProgramIdOffset:flatbuffers.Offset) {
  builder.addFieldOffset(4, callbackProgramIdOffset, 0);
}

static createCallbackProgramIdVector(builder:flatbuffers.Builder, data:number[]|Uint8Array):flatbuffers.Offset {
  builder.startVector(1, data.length, 1);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addInt8(data[i]!);
  }
  return builder.endVector();
}

static startCallbackProgramIdVector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(1, numElems, 1);
}

static addCallbackInstructionPrefix(builder:flatbuffers.Builder, callbackInstructionPrefixOffset:flatbuffers.Offset) {
  builder.addFieldOffset(5, callbackInstructionPrefixOffset, 0);
}

static createCallbackInstructionPrefixVector(builder:flatbuffers.Builder, data:number[]|Uint8Array):flatbuffers.Offset {
  builder.startVector(1, data.length, 1);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addInt8(data[i]!);
  }
  return builder.endVector();
}

static startCallbackInstructionPrefixVector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(1, numElems, 1);
}

static addInputData(builder:flatbuffers.Builder, inputDataOffset:flatbuffers.Offset) {
  builder.addFieldOffset(6, inputDataOffset, 0);
}

static createInputDataVector(builder:flatbuffers.Builder, data:number[]|Uint8Array):flatbuffers.Offset {
  builder.startVector(1, data.length, 1);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addInt8(data[i]!);
  }
  return builder.endVector();
}

static startInputDataVector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(1, numElems, 1);
}

static endExecutionRequestV1(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static finishExecutionRequestV1Buffer(builder:flatbuffers.Builder, offset:flatbuffers.Offset) {
  builder.finish(offset);
}

static finishSizePrefixedExecutionRequestV1Buffer(builder:flatbuffers.Builder, offset:flatbuffers.Offset) {
  builder.finish(offset, undefined, true);
}

static createExecutionRequestV1(builder:flatbuffers.Builder, inputType:ExecutionInputType, tip:bigint, executionIdOffset:flatbuffers.Offset, imageIdOffset:flatbuffers.Offset, callbackProgramIdOffset:flatbuffers.Offset, callbackInstructionPrefixOffset:flatbuffers.Offset, inputDataOffset:flatbuffers.Offset):flatbuffers.Offset {
  ExecutionRequestV1.startExecutionRequestV1(builder);
  ExecutionRequestV1.addInputType(builder, inputType);
  ExecutionRequestV1.addTip(builder, tip);
  ExecutionRequestV1.addExecutionId(builder, executionIdOffset);
  ExecutionRequestV1.addImageId(builder, imageIdOffset);
  ExecutionRequestV1.addCallbackProgramId(builder, callbackProgramIdOffset);
  ExecutionRequestV1.addCallbackInstructionPrefix(builder, callbackInstructionPrefixOffset);
  ExecutionRequestV1.addInputData(builder, inputDataOffset);
  return ExecutionRequestV1.endExecutionRequestV1(builder);
}
}

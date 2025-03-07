// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import * as flatbuffers from 'flatbuffers';

import { ChannelInstructionIxType } from './channel-instruction-ix-type.js';


export class ChannelInstruction implements flatbuffers.IUnpackableObject<ChannelInstructionT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):ChannelInstruction {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsChannelInstruction(bb:flatbuffers.ByteBuffer, obj?:ChannelInstruction):ChannelInstruction {
  return (obj || new ChannelInstruction()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsChannelInstruction(bb:flatbuffers.ByteBuffer, obj?:ChannelInstruction):ChannelInstruction {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new ChannelInstruction()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

ixType():ChannelInstructionIxType {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : ChannelInstructionIxType.ExecuteV1;
}

mutate_ix_type(value:ChannelInstructionIxType):boolean {
  const offset = this.bb!.__offset(this.bb_pos, 4);

  if (offset === 0) {
    return false;
  }

  this.bb!.writeUint8(this.bb_pos + offset, value);
  return true;
}

executeV1(index: number):number|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readUint8(this.bb!.__vector(this.bb_pos + offset) + index) : 0;
}

executeV1Length():number {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

executeV1Array():Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? new Uint8Array(this.bb!.bytes().buffer, this.bb!.bytes().byteOffset + this.bb!.__vector(this.bb_pos + offset), this.bb!.__vector_len(this.bb_pos + offset)) : null;
}

deployV1(index: number):number|null {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? this.bb!.readUint8(this.bb!.__vector(this.bb_pos + offset) + index) : 0;
}

deployV1Length():number {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

deployV1Array():Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? new Uint8Array(this.bb!.bytes().buffer, this.bb!.bytes().byteOffset + this.bb!.__vector(this.bb_pos + offset), this.bb!.__vector_len(this.bb_pos + offset)) : null;
}

claimV1(index: number):number|null {
  const offset = this.bb!.__offset(this.bb_pos, 10);
  return offset ? this.bb!.readUint8(this.bb!.__vector(this.bb_pos + offset) + index) : 0;
}

claimV1Length():number {
  const offset = this.bb!.__offset(this.bb_pos, 10);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

claimV1Array():Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 10);
  return offset ? new Uint8Array(this.bb!.bytes().buffer, this.bb!.bytes().byteOffset + this.bb!.__vector(this.bb_pos + offset), this.bb!.__vector_len(this.bb_pos + offset)) : null;
}

static startChannelInstruction(builder:flatbuffers.Builder) {
  builder.startObject(4);
}

static addIxType(builder:flatbuffers.Builder, ixType:ChannelInstructionIxType) {
  builder.addFieldInt8(0, ixType, ChannelInstructionIxType.ExecuteV1);
}

static addExecuteV1(builder:flatbuffers.Builder, executeV1Offset:flatbuffers.Offset) {
  builder.addFieldOffset(1, executeV1Offset, 0);
}

static createExecuteV1Vector(builder:flatbuffers.Builder, data:number[]|Uint8Array):flatbuffers.Offset {
  builder.startVector(1, data.length, 1);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addInt8(data[i]!);
  }
  return builder.endVector();
}

static startExecuteV1Vector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(1, numElems, 1);
}

static addDeployV1(builder:flatbuffers.Builder, deployV1Offset:flatbuffers.Offset) {
  builder.addFieldOffset(2, deployV1Offset, 0);
}

static createDeployV1Vector(builder:flatbuffers.Builder, data:number[]|Uint8Array):flatbuffers.Offset {
  builder.startVector(1, data.length, 1);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addInt8(data[i]!);
  }
  return builder.endVector();
}

static startDeployV1Vector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(1, numElems, 1);
}

static addClaimV1(builder:flatbuffers.Builder, claimV1Offset:flatbuffers.Offset) {
  builder.addFieldOffset(3, claimV1Offset, 0);
}

static createClaimV1Vector(builder:flatbuffers.Builder, data:number[]|Uint8Array):flatbuffers.Offset {
  builder.startVector(1, data.length, 1);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addInt8(data[i]!);
  }
  return builder.endVector();
}

static startClaimV1Vector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(1, numElems, 1);
}

static endChannelInstruction(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static finishChannelInstructionBuffer(builder:flatbuffers.Builder, offset:flatbuffers.Offset) {
  builder.finish(offset);
}

static finishSizePrefixedChannelInstructionBuffer(builder:flatbuffers.Builder, offset:flatbuffers.Offset) {
  builder.finish(offset, undefined, true);
}

static createChannelInstruction(builder:flatbuffers.Builder, ixType:ChannelInstructionIxType, executeV1Offset:flatbuffers.Offset, deployV1Offset:flatbuffers.Offset, claimV1Offset:flatbuffers.Offset):flatbuffers.Offset {
  ChannelInstruction.startChannelInstruction(builder);
  ChannelInstruction.addIxType(builder, ixType);
  ChannelInstruction.addExecuteV1(builder, executeV1Offset);
  ChannelInstruction.addDeployV1(builder, deployV1Offset);
  ChannelInstruction.addClaimV1(builder, claimV1Offset);
  return ChannelInstruction.endChannelInstruction(builder);
}

unpack(): ChannelInstructionT {
  return new ChannelInstructionT(
    this.ixType(),
    this.bb!.createScalarList<number>(this.executeV1.bind(this), this.executeV1Length()),
    this.bb!.createScalarList<number>(this.deployV1.bind(this), this.deployV1Length()),
    this.bb!.createScalarList<number>(this.claimV1.bind(this), this.claimV1Length())
  );
}


unpackTo(_o: ChannelInstructionT): void {
  _o.ixType = this.ixType();
  _o.executeV1 = this.bb!.createScalarList<number>(this.executeV1.bind(this), this.executeV1Length());
  _o.deployV1 = this.bb!.createScalarList<number>(this.deployV1.bind(this), this.deployV1Length());
  _o.claimV1 = this.bb!.createScalarList<number>(this.claimV1.bind(this), this.claimV1Length());
}
}

export class ChannelInstructionT implements flatbuffers.IGeneratedObject {
constructor(
  public ixType: ChannelInstructionIxType = ChannelInstructionIxType.ExecuteV1,
  public executeV1: (number)[] = [],
  public deployV1: (number)[] = [],
  public claimV1: (number)[] = []
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const executeV1 = ChannelInstruction.createExecuteV1Vector(builder, this.executeV1);
  const deployV1 = ChannelInstruction.createDeployV1Vector(builder, this.deployV1);
  const claimV1 = ChannelInstruction.createClaimV1Vector(builder, this.claimV1);

  return ChannelInstruction.createChannelInstruction(builder,
    this.ixType,
    executeV1,
    deployV1,
    claimV1
  );
}
}
